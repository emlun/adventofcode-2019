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

#[derive(Eq, PartialEq)]
enum Entity {
    Key(KeyId),
    Door(KeyId),
}

#[derive(Eq, PartialEq)]
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
    fn new() -> Self {
        KeySet { keys: 0 }
    }

    fn insert(&mut self, key: KeyId) -> &mut Self {
        self.keys |= key.value;
        self
    }

    fn with(&self, key: KeyId) -> Self {
        KeySet {
            keys: self.keys | key.value,
        }
    }

    fn contains(&self, key: KeyId) -> bool {
        self.keys & key.value != 0
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

#[derive(Eq, PartialEq)]
struct World {
    tiles: Vec<Vec<Tile>>,
    keys: KeySet,
}

#[derive(Eq, PartialEq)]
struct State<'world> {
    world: &'world World,
    poss: Vec<Point>,
    collected: KeySet,
    len: usize,
}

impl<'world> State<'world> {
    fn available_moves(&self, pos: Point) -> Vec<(Point, usize, KeyId)> {
        let mut moves: Vec<(Point, usize, KeyId)> = Vec::new();
        let mut visited = HashSet::new();
        let mut queue: VecDeque<(Point, usize)> = VecDeque::new();
        queue.push_back((pos, 0));

        while let Some((step, len)) = queue.pop_front() {
            for next in adjacent(&step) {
                if !visited.contains(&next) && self.can_walk(&next) {
                    let next_len = len + 1;
                    if let Floor(Some(Key(k))) = self.world.tiles[next.1][next.0] {
                        if self.collected.contains(k) {
                            queue.push_back((next, next_len));
                        } else {
                            moves.push((next, next_len, k));
                        }
                    } else {
                        queue.push_back((next, next_len));
                    }
                }
            }

            visited.insert(step);
        }

        moves
    }

    fn can_walk(&self, pos: &Point) -> bool {
        match self.world.tiles[pos.1][pos.0] {
            Wall => false,
            Floor(None) => true,
            Floor(Some(Key(_))) => true,
            Floor(Some(Door(a))) => self.collected.contains(a),
        }
    }

    #[allow(dead_code)]
    fn print_state(&self) {
        println!(
            "{}",
            self.world
                .tiles
                .iter()
                .enumerate()
                .map(|(r, row)| row
                    .iter()
                    .enumerate()
                    .map(|(c, tile)| if self.poss.contains(&(c, r)) {
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
}

impl Ord for State<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.len.cmp(&self.len)
    }
}

impl PartialOrd for State<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_world(lines: &[String]) -> (World, Point) {
    let mut player_pos = (0, 0);
    let mut keys = KeySet::new();
    let tiles = lines
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
                        Floor(None)
                    }
                    a => Floor(Some(if a == a.to_ascii_uppercase() {
                        Door(a.into())
                    } else {
                        keys.insert(a.into());
                        Key(a.into())
                    })),
                })
                .collect()
        })
        .collect();

    (World { tiles, keys }, player_pos)
}

fn dijkstra<'world>(world: &'world World, start_positions: &Vec<Point>) -> Option<State<'world>> {
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    let mut shortest_paths: HashMap<(KeySet, Point), usize> = HashMap::new();

    queue.push(State {
        world,
        poss: start_positions.clone(),
        collected: KeySet::new(),
        len: 0,
    });

    while let Some(state) = queue.pop() {
        if state.collected == world.keys {
            return Some(state);
        } else {
            for (posi, pos) in state.poss.iter().enumerate() {
                let shortest = shortest_paths
                    .entry((state.collected, *pos))
                    .or_insert(state.len + 1);
                if state.len < *shortest {
                    *shortest = state.len;

                    for (next_point, len_to_next, next_key) in state.available_moves(*pos) {
                        let collected = state.collected.with(next_key);
                        let mut poss = state.poss.clone();
                        poss[posi] = next_point;

                        queue.push(State {
                            world,
                            poss,
                            collected,
                            len: state.len + len_to_next,
                        });
                    }
                }
            }
        }
    }
    None
}

fn solve_a(world: &World, pos: Vec<Point>) -> usize {
    let found = dijkstra(world, &pos);
    found.unwrap().len
}

fn solve_b(mut world: World, pos: Point) -> usize {
    world.tiles[pos.1 - 1][pos.0] = Tile::Wall;
    world.tiles[pos.1][pos.0 - 1] = Tile::Wall;
    world.tiles[pos.1 + 1][pos.0] = Tile::Wall;
    world.tiles[pos.1][pos.0 + 1] = Tile::Wall;

    let pos = vec![
        (pos.0 - 1, pos.1 - 1),
        (pos.0 - 1, pos.1 + 1),
        (pos.0 + 1, pos.1 + 1),
        (pos.0 + 1, pos.1 - 1),
    ];

    let found = dijkstra(&world, &pos);
    found.unwrap().len
}

pub fn solve(lines: &[String]) -> Solution {
    let (world, pos) = parse_world(lines);
    let a_solution = solve_a(&world, vec![pos]);
    let b_solution = solve_b(world, pos);
    (a_solution.to_string(), b_solution.to_string())
}
