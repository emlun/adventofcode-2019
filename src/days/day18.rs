use crate::common::Solution;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Point = (usize, usize);

fn adjacent(pos: &Point) -> Vec<Point> {
    vec![
        (pos.0 + 1, pos.1 + 0),
        (pos.0 + 0, pos.1 + 1),
        (pos.0 - 1, pos.1 + 0),
        (pos.0 + 0, pos.1 - 1),
    ]
}

enum Entity {
    Key(KeyId),
    Door(KeyId),
}

enum Tile {
    Floor(Option<Entity>),
    Wall,
}

use Entity::{Door, Key};
use Tile::{Floor, Wall};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct KeyId {
    value: u32,
}

impl KeyId {
    fn to_char(&self) -> char {
        ((f64::from(self.value).log2().round() as u8) + ('a' as u8)) as char
    }
}

impl From<char> for KeyId {
    fn from(c: char) -> Self {
        let basis = if c.is_ascii_uppercase() { 'A' } else { 'a' };
        KeyId {
            value: 2_u32.pow(c as u32 - basis as u32),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct KeySet {
    keys: u32,
}

impl KeySet {
    fn insert(&mut self, key: KeyId) -> &mut Self {
        self.keys |= key.value;
        self
    }

    fn contains(&self, key: KeyId) -> bool {
        self.keys & key.value != 0
    }

    fn len_equals(&self, len: u32) -> bool {
        self.keys == 2_u32.pow(len) - 1
    }
}

impl<K> std::iter::FromIterator<K> for KeySet
where
    K: Into<KeyId>,
{
    fn from_iter<I>(it: I) -> Self
    where
        I: IntoIterator<Item = K>,
    {
        KeySet {
            keys: it.into_iter().fold(0, |keys, key| keys | key.into().value),
        }
    }
}

#[derive(Debug, Eq, Ord, PartialEq)]
struct State {
    pub poss: Vec<Point>,
    pub collected: KeySet,
    pub len: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.len.cmp(&self.len))
    }
}

struct World {
    tiles: Vec<Vec<Tile>>,
    keys: HashMap<Point, KeyId>,
}

impl World {
    fn new(tiles: Vec<Vec<Tile>>) -> World {
        let mut keys = HashMap::new();
        for (r, row) in tiles.iter().enumerate() {
            for (c, tile) in row.iter().enumerate() {
                if let Floor(Some(Key(k))) = tile {
                    keys.insert((c, r), KeyId::from(*k));
                }
            }
        }
        World { tiles, keys }
    }
}

fn compute_transfers(
    world: &World,
    collected: KeySet,
) -> HashMap<Point, Vec<(Point, usize, KeyId)>> {
    let mut result: HashMap<Point, Vec<(Point, usize, KeyId)>> = HashMap::new();

    for start_pos in world.keys.keys() {
        let mut partmap: Vec<(Point, usize, KeyId)> = Vec::new();
        let mut visited = HashSet::new();
        let mut queue: VecDeque<(Point, usize)> = VecDeque::new();
        queue.push_back((*start_pos, 0));

        while let Some((step, len)) = queue.pop_front() {
            for next in adjacent(&step) {
                if !visited.contains(&next) && can_walk(world, collected, &next) {
                    if let Floor(Some(Key(k))) = world.tiles[next.1][next.0] {
                        if !collected.contains(k) {
                            partmap.push((next, len + 1, k));
                        } else {
                            queue.push_back((next, len + 1));
                        }
                    } else {
                        queue.push_back((next, len + 1));
                    }
                }
            }

            visited.insert(step);
        }

        result.insert(*start_pos, partmap);
    }

    result
}

fn parse_world(lines: &[String]) -> (World, Point) {
    let mut player_pos = (0, 0);
    let world = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Wall,
                    '.' => Floor(None),
                    '@' => {
                        player_pos = (x, y);
                        Floor(Some(Key('{'.into())))
                    }
                    a => Floor(Some(if a == a.to_ascii_uppercase() {
                        Door(a.into())
                    } else {
                        Key(a.into())
                    })),
                })
                .collect()
        })
        .collect();
    (World::new(world), player_pos)
}

fn can_walk(world: &World, collected: KeySet, pos: &Point) -> bool {
    match world.tiles[pos.1][pos.0] {
        Wall => false,
        Floor(None) => true,
        Floor(Some(Key(_))) => true,
        Floor(Some(Door(a))) => collected.contains(a),
    }
}

fn dijkstra(world: &World, start_positions: &Vec<Point>, start_collected: KeySet) -> Option<State> {
    let mut transfers: HashMap<KeySet, HashMap<Point, Vec<(Point, usize, KeyId)>>> = HashMap::new();
    let mut queue: BinaryHeap<State> = BinaryHeap::new();

    let mut shortest_collections: HashMap<KeySet, HashMap<KeyId, usize>> = HashMap::new();

    queue.push(State {
        poss: start_positions.clone(),
        collected: start_collected,
        len: 0,
    });

    while let Some(state) = queue.pop() {
        if state.collected.len_equals(world.keys.len() as u32) {
            return Some(state);
        } else {
            for posi in 0..state.poss.len() {
                let last_key: &KeyId = world.keys.get(&state.poss[posi]).unwrap();

                if shortest_collections
                    .get(&state.collected)
                    .and_then(|stands| stands.get(&last_key))
                    .map(|len| *len > state.len)
                    .unwrap_or(true)
                {
                    let trns = if let Some(t) = transfers.get(&state.collected) {
                        t
                    } else {
                        let t = compute_transfers(world, state.collected);
                        transfers.insert(state.collected, t);
                        transfers.get(&state.collected).unwrap()
                    };

                    let shortcoll = shortest_collections
                        .entry(state.collected)
                        .or_insert(HashMap::new())
                        .entry(*last_key)
                        .or_insert(state.len);

                    if state.len < *shortcoll {
                        *shortcoll = state.len;
                    }

                    for (next_point, len_to_next, next_key) in trns.get(&state.poss[posi]).unwrap()
                    {
                        let mut collected = state.collected;
                        collected.insert(*next_key);

                        let mut poss = state.poss.clone();
                        poss[posi] = *next_point;

                        let next_state = State {
                            poss,
                            collected,
                            len: state.len + len_to_next,
                        };

                        queue.push(next_state);
                    }
                }
            }
        }
    }
    None
}

fn solve_a(world: &World, pos: Vec<Point>) -> usize {
    let found = dijkstra(world, &pos, vec!['{'].into_iter().collect());
    found.unwrap().len
}

fn solve_b(mut world: World, pos: Point) -> usize {
    world.tiles[pos.1][pos.0] = Tile::Wall;
    world.tiles[pos.1 - 1][pos.0] = Tile::Wall;
    world.tiles[pos.1][pos.0 - 1] = Tile::Wall;
    world.tiles[pos.1 + 1][pos.0] = Tile::Wall;
    world.tiles[pos.1][pos.0 + 1] = Tile::Wall;
    world.keys.remove(&pos);
    world.keys.insert((pos.0 - 1, pos.1 - 1), '{'.into());
    world.keys.insert((pos.0 - 1, pos.1 + 1), '|'.into());
    world.keys.insert((pos.0 + 1, pos.1 + 1), '}'.into());
    world.keys.insert((pos.0 + 1, pos.1 - 1), '~'.into());

    let pos = vec![
        (pos.0 - 1, pos.1 - 1),
        (pos.0 - 1, pos.1 + 1),
        (pos.0 + 1, pos.1 + 1),
        (pos.0 + 1, pos.1 - 1),
    ];

    let found = dijkstra(&world, &pos, vec!['{', '|', '}', '~'].into_iter().collect());
    found.unwrap().len
}

#[allow(dead_code)]
fn print_state(world: &World, state: &State) {
    println!(
        "{}",
        world
            .tiles
            .iter()
            .enumerate()
            .map(|(r, row)| row
                .iter()
                .enumerate()
                .map(|(c, tile)| if state.poss.contains(&(c, r)) {
                    '@'.to_string()
                } else {
                    match tile {
                        Wall => '#'.to_string(),
                        Floor(Some(Key(a))) => a.to_char().to_string(),
                        Floor(Some(Door(a))) => a.to_char().to_ascii_uppercase().to_string(),
                        Floor(None) => '.'.to_string(),
                    }
                })
                .collect::<Vec<String>>()
                .join(""))
            .collect::<Vec<String>>()
            .join("\n")
    );
}

pub fn solve(lines: &[String]) -> Solution {
    let (world, pos) = parse_world(lines);
    let a_solution = solve_a(&world, vec![pos]);
    let b_solution = solve_b(world, pos);
    (a_solution.to_string(), b_solution.to_string())
}