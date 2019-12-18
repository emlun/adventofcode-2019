use crate::common::Solution;
use std::collections::BTreeSet;
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
    Key(char),
    Door(char),
}

enum Tile {
    Floor(Option<Entity>),
    Wall,
}

use Entity::{Door, Key};
use Tile::{Floor, Wall};

#[derive(Debug, Eq, Ord, PartialEq)]
struct State2 {
    pub poss: Vec<Point>,
    pub collected: BTreeSet<char>,
    pub len: usize,
}

impl PartialOrd for State2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.len.cmp(&self.len))
    }
}

struct World2 {
    tiles: Vec<Vec<Tile>>,
    keys: HashMap<Point, char>,
}

impl World2 {
    fn new(tiles: Vec<Vec<Tile>>) -> World2 {
        let mut keys = HashMap::new();
        for (r, row) in tiles.iter().enumerate() {
            for (c, tile) in row.iter().enumerate() {
                if let Floor(Some(Key(k))) = tile {
                    keys.insert((c, r), *k);
                }
            }
        }
        World2 { tiles, keys }
    }
}

fn compute_transfers(
    world: &World2,
    collected: &BTreeSet<char>,
) -> HashMap<Point, Vec<(Point, usize, char)>> {
    let mut result: HashMap<Point, Vec<(Point, usize, char)>> = HashMap::new();

    for start_pos in world.keys.keys() {
        let mut partmap: Vec<(Point, usize, char)> = Vec::new();
        let mut visited = HashSet::new();
        let mut queue: VecDeque<(Point, usize)> = VecDeque::new();
        queue.push_back((*start_pos, 0));

        while let Some((step, len)) = queue.pop_front() {
            for next in adjacent(&step) {
                if !visited.contains(&next) && can_walk2(world, &collected, &next) {
                    if let Floor(Some(Key(k))) = world.tiles[next.1][next.0] {
                        if !collected.contains(&k) {
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

fn parse_world(lines: &[String]) -> (World2, Point) {
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
                        Floor(Some(Key('@')))
                    }
                    a => Floor(Some(if a == a.to_ascii_uppercase() {
                        Door(a.to_ascii_lowercase())
                    } else {
                        Key(a)
                    })),
                })
                .collect()
        })
        .collect();
    (World2::new(world), player_pos)
}

fn can_walk2(world: &World2, collected: &BTreeSet<char>, pos: &Point) -> bool {
    match world.tiles[pos.1][pos.0] {
        Wall => false,
        Floor(None) => true,
        Floor(Some(Key(_))) => true,
        Floor(Some(Door(a))) => collected.contains(&a),
    }
}

fn dijkstra(
    world: &World2,
    start_positions: &Vec<Point>,
    start_collected: BTreeSet<char>,
) -> Option<State2> {
    let mut transfers: HashMap<BTreeSet<char>, HashMap<Point, Vec<(Point, usize, char)>>> =
        HashMap::new();
    let mut queue: BinaryHeap<State2> = BinaryHeap::new();

    let mut shortest_collections: HashMap<BTreeSet<char>, HashMap<char, usize>> = HashMap::new();

    queue.push(State2 {
        poss: start_positions.clone(),
        collected: start_collected,
        len: 0,
    });

    while let Some(state) = queue.pop() {
        if state.collected.len() == world.keys.len() {
            return Some(state);
        } else {
            for posi in 0..state.poss.len() {
                let last_key: &char = world.keys.get(&state.poss[posi]).unwrap();

                if shortest_collections
                    .get(&state.collected)
                    .and_then(|stands| stands.get(&last_key))
                    .map(|len| *len > state.len)
                    .unwrap_or(true)
                {
                    let trns = if let Some(t) = transfers.get(&state.collected) {
                        t
                    } else {
                        let t = compute_transfers(world, &state.collected);
                        transfers.insert(state.collected.clone(), t);
                        transfers.get(&state.collected).unwrap()
                    };

                    let shortcoll = shortest_collections
                        .entry(state.collected.clone())
                        .or_insert(HashMap::new())
                        .entry(*last_key)
                        .or_insert(state.len);

                    if state.len < *shortcoll {
                        *shortcoll = state.len;
                    }

                    for (next_point, len_to_next, next_key) in trns.get(&state.poss[posi]).unwrap()
                    {
                        let mut collected = state.collected.clone();
                        collected.insert(*next_key);

                        let mut poss = state.poss.clone();
                        poss[posi] = *next_point;

                        let next_state = State2 {
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

fn solve_a(world: &World2, pos: Vec<Point>, start_collected: BTreeSet<char>) -> usize {
    let found = dijkstra(world, &pos, start_collected);
    found.unwrap().len
}

// fn solve_b(world: &World2, pos: Point) -> String {
//     "".to_string()
// }

fn print_world(world: &World2) {
    println!(
        "{}",
        world
            .tiles
            .iter()
            .map(|row| row
                .iter()
                .map(|c| match c {
                    Wall => '#'.to_string(),
                    Floor(Some(Key(a))) => a.to_string(),
                    Floor(Some(Door(a))) => a.to_ascii_uppercase().to_string(),
                    Floor(None) => '.'.to_string(),
                })
                .collect::<Vec<String>>()
                .join(""))
            .collect::<Vec<String>>()
            .join("\n")
    );
}

fn print_state(world: &World2, state: &State2) {
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
                        Floor(Some(Key(a))) => a.to_string(),
                        Floor(Some(Door(a))) => a.to_ascii_uppercase().to_string(),
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
    let (mut world, pos) = parse_world(lines);

    let a_solution = solve_a(&world, vec![pos], BTreeSet::new());

    world.tiles[pos.1][pos.0] = Tile::Wall;
    world.tiles[pos.1 - 1][pos.0] = Tile::Wall;
    world.tiles[pos.1][pos.0 - 1] = Tile::Wall;
    world.tiles[pos.1 + 1][pos.0] = Tile::Wall;
    world.tiles[pos.1][pos.0 + 1] = Tile::Wall;
    world.keys.remove(&pos);
    world.keys.insert((pos.0 - 1, pos.1 - 1), '@');
    world.keys.insert((pos.0 - 1, pos.1 + 1), '#');
    world.keys.insert((pos.0 + 1, pos.1 + 1), '$');
    world.keys.insert((pos.0 + 1, pos.1 - 1), '%');

    print_world(&world);

    let pos = vec![
        (pos.0 - 1, pos.1 - 1),
        (pos.0 - 1, pos.1 + 1),
        (pos.0 + 1, pos.1 + 1),
        (pos.0 + 1, pos.1 - 1),
    ];

    let b_solution = solve_a(&world, pos, vec!['@', '#', '$', '%'].into_iter().collect());
    (a_solution.to_string(), b_solution.to_string())
}
