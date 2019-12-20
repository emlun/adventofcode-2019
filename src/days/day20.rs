use crate::common::Solution;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::rc::Rc;

type Point = (i32, i32);

fn adjacent(pos: &Point) -> Vec<Point> {
    vec![
        (pos.0 + 1, pos.1 + 0),
        (pos.0 + 0, pos.1 + 1),
        (pos.0 - 1, pos.1 + 0),
        (pos.0 + 0, pos.1 - 1),
    ]
}

fn steps_from(world: &World, pos: &Point) -> Vec<Point> {
    adjacent(pos)
        .into_iter()
        .flat_map(|next_pos| match world.tiles.get(&next_pos) {
            None => None,
            Some(Tile::Wall) => None,
            Some(Tile::Floor) => Some(next_pos),
            Some(Tile::Warp(_)) => {
                if *pos == world.start {
                    None
                } else if next_pos == world.goal {
                    Some(next_pos)
                } else {
                    Some(world.warps.get(&next_pos).unwrap().to)
                }
            }
        })
        .collect()
}

#[derive(Eq, PartialEq)]
enum Tile {
    Floor,
    Wall,
    Warp(String),
}

#[derive(Debug, Eq, Ord, PartialEq)]
struct State {
    pub pos: Point,
    pub len: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.len.cmp(&self.len))
    }
}

struct World {
    tiles: HashMap<Point, Tile>,
    warps: HashMap<Point, Warp>,
    start: Point,
    goal: Point,
}

#[derive(Debug)]
struct Warp {
    name: String,
    to: Point,
}

impl World {
    fn parse(lines: &[String]) -> World {
        let tiles: HashMap<Point, Tile> = lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().flat_map(move |(x, c)| match c {
                    ' ' => None,
                    '#' => Some(((x as i32, y as i32), Tile::Wall)),
                    '.' => Some(((x as i32, y as i32), Tile::Floor)),
                    a => Some(((x as i32, y as i32), Tile::Warp(a.to_string()))),
                })
            })
            .collect();

        fn find_start_of_warp_name(tiles: &HashMap<Point, Tile>, pos: Point) -> Point {
            let continuation = &[(pos.0 - 1, pos.1), (pos.0, pos.1 - 1)]
                .into_iter()
                .flat_map(|next_pos| match tiles.get(&next_pos) {
                    Some(Tile::Warp(_)) => Some(find_start_of_warp_name(tiles, *next_pos)),
                    _ => None,
                })
                .next();
            continuation.unwrap_or(pos)
        }

        fn read_warp_name(tiles: &HashMap<Point, Tile>, pos: Point) -> String {
            let continuation = &[(pos.0 + 1, pos.1), (pos.0, pos.1 + 1)]
                .into_iter()
                .flat_map(|next_pos| match tiles.get(&next_pos) {
                    Some(Tile::Warp(n)) => Some(read_warp_name(tiles, *next_pos)),
                    _ => None,
                })
                .next();
            if let Some(Tile::Warp(n)) = tiles.get(&pos) {
                format!("{}{}", n, continuation.as_ref().unwrap_or(&"".to_string()))
            } else {
                unreachable!();
            }
        }

        let mut warp_names: HashMap<String, Vec<Point>> = HashMap::new();
        for (pos, tile) in tiles.iter() {
            if let Tile::Warp(_) = tile {
                if adjacent(pos)
                    .into_iter()
                    .any(|p| tiles.get(&p) == Some(&Tile::Floor))
                {
                    let name_start = find_start_of_warp_name(&tiles, *pos);
                    let name = read_warp_name(&tiles, name_start);
                    println!("{}", name);
                    warp_names.entry(name).or_insert_with(Vec::new).push(*pos);
                }
            }
        }

        fn walk_to_floor(
            tiles: &HashMap<Point, Tile>,
            pos: &Point,
            prev_pos: Option<&Point>,
        ) -> Option<Point> {
            for next_pos in adjacent(pos) {
                if Some(&next_pos) != prev_pos {
                    match tiles.get(&next_pos) {
                        Some(Tile::Floor) => return Some(next_pos),
                        Some(Tile::Warp(_)) => {
                            let r = walk_to_floor(tiles, &next_pos, Some(pos));
                            if r.is_some() {
                                return r;
                            }
                        }
                        _ => {}
                    }
                }
            }
            None
        }

        println!("{:?}", warp_names);
        let start = walk_to_floor(&tiles, &warp_names.get("AA").unwrap()[0], None).unwrap();
        let goal = walk_to_floor(&tiles, &warp_names.get("ZZ").unwrap()[0], None).unwrap();

        let warps = warp_names
            .into_iter()
            .filter(|(name, _)| name != "AA" && name != "ZZ")
            .fold(HashMap::new(), |mut warps, (name, points)| {
                warps.insert(
                    points[0],
                    Warp {
                        name: name.clone(),
                        to: walk_to_floor(&tiles, &points[1], None).unwrap(),
                    },
                );
                warps.insert(
                    points[1],
                    Warp {
                        name: name.clone(),
                        to: walk_to_floor(&tiles, &points[0], None).unwrap(),
                    },
                );
                warps
            });

        World {
            tiles,
            warps,
            start,
            goal,
        }
    }
}

fn can_walk(world: &World, pos: &Point) -> bool {
    match world.tiles.get(pos) {
        None => false,
        Some(Tile::Floor) => true,
        Some(Tile::Wall) => false,
        Some(Tile::Warp(_)) => true,
    }
}

fn bfs(world: &World) -> Option<State> {
    let mut queue: BinaryHeap<State> = BinaryHeap::new();

    queue.push(State {
        pos: world.start,
        len: 0,
    });

    let mut visited = HashSet::new();

    while let Some(state) = queue.pop() {
        if state.pos == world.goal {
            return Some(state);
        } else {
            visited.insert(state.pos);
            for next_point in steps_from(world, &state.pos) {
                if !visited.contains(&next_point) {
                    let next_state = State {
                        pos: next_point,
                        len: state.len + 1,
                    };
                    queue.push(next_state);
                }
            }
        }
    }
    None
}

fn solve_a(world: &World) -> usize {
    let found = bfs(world);
    found.unwrap().len
}

pub fn solve(lines: &[String]) -> Solution {
    let world = World::parse(lines);
    let a_solution = solve_a(&world);
    // let b_solution = solve_b(world);
    let b_solution = "";
    (a_solution.to_string(), b_solution.to_string())
}
