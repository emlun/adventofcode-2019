use crate::common::Solution;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Point = (i32, i32);

fn adjacent(pos: Point) -> Vec<Point> {
    vec![
        (pos.0 + 1, pos.1),
        (pos.0, pos.1 + 1),
        (pos.0 - 1, pos.1),
        (pos.0, pos.1 - 1),
    ]
}

fn steps_from(world: &World, loc: &Loc, levels: bool) -> Vec<Loc> {
    adjacent(loc.pos)
        .into_iter()
        .flat_map(|next_pos| match world.tiles.get(&next_pos) {
            None => None,
            Some(Tile::Wall) => None,
            Some(Tile::Floor) => Some(Loc {
                pos: next_pos,
                level: loc.level,
            }),
            Some(Tile::Warp(_)) => {
                if loc.pos == world.start {
                    None
                } else if loc.pos == world.goal {
                    if loc.level == 0 {
                        Some(Loc {
                            pos: next_pos,
                            level: loc.level,
                        })
                    } else {
                        None
                    }
                } else {
                    let next_level = if levels {
                        if next_pos.0 == (world.outer_warp_ring.0).0
                            || next_pos.0 == (world.outer_warp_ring.1).0
                            || next_pos.1 == (world.outer_warp_ring.0).1
                            || next_pos.1 == (world.outer_warp_ring.1).1
                        {
                            if loc.level == 0 {
                                None
                            } else {
                                Some(loc.level - 1)
                            }
                        } else {
                            Some(loc.level + 1)
                        }
                    } else {
                        Some(loc.level)
                    };
                    next_level.map(|next_level| Loc {
                        pos: world.warps.get(&next_pos).unwrap().to,
                        level: next_level,
                    })
                }
            }
        })
        .collect()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Loc {
    pos: Point,
    level: u32,
}

#[derive(Eq, PartialEq)]
enum Tile {
    Floor,
    Wall,
    Warp(String),
}

struct State {
    pub loc: Loc,
    pub len: usize,
}

struct World {
    tiles: HashMap<Point, Tile>,
    warps: HashMap<Point, Warp>,
    start: Point,
    goal: Point,
    outer_warp_ring: (Point, Point),
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
                .iter()
                .flat_map(|next_pos| match tiles.get(&next_pos) {
                    Some(Tile::Warp(_)) => Some(find_start_of_warp_name(tiles, *next_pos)),
                    _ => None,
                })
                .next();
            continuation.unwrap_or(pos)
        }

        fn read_warp_name(tiles: &HashMap<Point, Tile>, pos: Point) -> String {
            let continuation = &[(pos.0 + 1, pos.1), (pos.0, pos.1 + 1)]
                .iter()
                .flat_map(|next_pos| match tiles.get(&next_pos) {
                    Some(Tile::Warp(_)) => Some(read_warp_name(tiles, *next_pos)),
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
                if adjacent(*pos)
                    .into_iter()
                    .any(|p| tiles.get(&p) == Some(&Tile::Floor))
                {
                    let name_start = find_start_of_warp_name(&tiles, *pos);
                    let name = read_warp_name(&tiles, name_start);
                    warp_names.entry(name).or_insert_with(Vec::new).push(*pos);
                }
            }
        }

        fn walk_to_floor(
            tiles: &HashMap<Point, Tile>,
            pos: Point,
            prev_pos: Option<&Point>,
        ) -> Option<Point> {
            for next_pos in adjacent(pos) {
                if Some(&next_pos) != prev_pos {
                    match tiles.get(&next_pos) {
                        Some(Tile::Floor) => return Some(next_pos),
                        Some(Tile::Warp(_)) => {
                            let r = walk_to_floor(tiles, next_pos, Some(&pos));
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

        let start = walk_to_floor(&tiles, warp_names.get("AA").unwrap()[0], None).unwrap();
        let goal = walk_to_floor(&tiles, warp_names.get("ZZ").unwrap()[0], None).unwrap();

        let warps = warp_names
            .into_iter()
            .filter(|(name, _)| name != "AA" && name != "ZZ")
            .fold(HashMap::new(), |mut warps, (name, points)| {
                warps.insert(
                    points[0],
                    Warp {
                        name: name.clone(),
                        to: walk_to_floor(&tiles, points[1], None).unwrap(),
                    },
                );
                warps.insert(
                    points[1],
                    Warp {
                        name: name.clone(),
                        to: walk_to_floor(&tiles, points[0], None).unwrap(),
                    },
                );
                warps
            });

        let warps_min_x = *warps.keys().map(|(x, _)| x).min().unwrap();
        let warps_max_x = *warps.keys().map(|(x, _)| x).max().unwrap();
        let warps_min_y = *warps.keys().map(|(_, y)| y).min().unwrap();
        let warps_max_y = *warps.keys().map(|(_, y)| y).max().unwrap();

        World {
            tiles,
            warps,
            start,
            goal,
            outer_warp_ring: ((warps_min_x, warps_min_y), (warps_max_x, warps_max_y)),
        }
    }
}

fn bfs(world: &World, levels: bool) -> Option<State> {
    let mut queue: VecDeque<State> = VecDeque::new();

    queue.push_back(State {
        loc: Loc {
            pos: world.start,
            level: 0,
        },
        len: 0,
    });

    let mut visited = HashSet::new();

    while let Some(state) = queue.pop_front() {
        if state.loc.pos == world.goal && state.loc.level == 0 {
            return Some(state);
        } else {
            visited.insert(state.loc);
            for next_loc in steps_from(world, &state.loc, levels) {
                if !visited.contains(&next_loc) {
                    let next_state = State {
                        loc: next_loc,
                        len: state.len + 1,
                    };
                    queue.push_back(next_state);
                }
            }
        }
    }
    None
}

fn solve_a(world: &World) -> usize {
    let found = bfs(world, false);
    found.unwrap().len
}

fn solve_b(world: &World) -> usize {
    let found = bfs(world, true);
    found.unwrap().len
}

pub fn solve(lines: &[String]) -> Solution {
    let world = World::parse(lines);
    let a_solution = solve_a(&world);
    let b_solution = solve_b(&world);
    (a_solution.to_string(), b_solution.to_string())
}
