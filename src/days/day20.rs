use crate::common::Solution;
use std::collections::BinaryHeap;
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

fn steps_from(world: &World, nav: &mut Navigation, loc: &Loc, levels: bool) -> Vec<(Loc, usize)> {
    nav.available_moves(loc.pos)
        .iter()
        .flat_map(|route| {
            if route.to == world.start {
                None
            } else if route.to == world.goal {
                if loc.level == 0 {
                    Some((
                        Loc {
                            pos: route.to,
                            level: loc.level,
                        },
                        route.len,
                    ))
                } else {
                    None
                }
            } else {
                let next_level = if levels {
                    let is_outer_warp = route.to.0 == (world.outer_warp_ring.0).0
                        || route.to.0 == (world.outer_warp_ring.1).0
                        || route.to.1 == (world.outer_warp_ring.0).1
                        || route.to.1 == (world.outer_warp_ring.1).1;
                    if is_outer_warp {
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
                next_level.map(|next_level| {
                    (
                        Loc {
                            pos: world.warps.get(&route.to).unwrap().to,
                            level: next_level,
                        },
                        route.len,
                    )
                })
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

#[derive(Eq, PartialEq)]
struct State {
    pub loc: Loc,
    pub len: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.len.cmp(&self.len)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
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
                let warp_pos = walk_to_edge(&tiles, *pos, None).unwrap();
                let name_start = find_start_of_warp_name(&tiles, *pos);
                let name = read_warp_name(&tiles, name_start);
                let points = warp_names.entry(name).or_insert_with(Vec::new);
                if !points.contains(&warp_pos) {
                    points.push(warp_pos);
                }
            }
        }

        fn walk_to_edge(
            tiles: &HashMap<Point, Tile>,
            pos: Point,
            prev_pos: Option<&Point>,
        ) -> Option<Point> {
            for next_pos in adjacent(pos) {
                if Some(&next_pos) != prev_pos {
                    match tiles.get(&next_pos) {
                        Some(Tile::Floor) => return Some(pos),
                        Some(Tile::Warp(_)) => {
                            let r = walk_to_edge(tiles, next_pos, Some(&pos));
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

        fn walk_to_floor(tiles: &HashMap<Point, Tile>, pos: Point) -> Option<Point> {
            walk_to_edge(tiles, pos, None).and_then(|edge| {
                adjacent(edge)
                    .into_iter()
                    .find(|p| tiles.get(p) == Some(&Tile::Floor))
            })
        }

        let start = walk_to_edge(&tiles, warp_names.get("AA").unwrap()[0], None).unwrap();
        let goal = walk_to_edge(&tiles, warp_names.get("ZZ").unwrap()[0], None).unwrap();

        let warps = warp_names
            .into_iter()
            .filter(|(name, _)| name != "AA" && name != "ZZ")
            .fold(HashMap::new(), |mut warps, (name, points)| {
                warps.insert(
                    points[0],
                    Warp {
                        name: name.clone(),
                        to: walk_to_floor(&tiles, points[1]).unwrap(),
                    },
                );
                warps.insert(
                    points[1],
                    Warp {
                        name,
                        to: walk_to_floor(&tiles, points[0]).unwrap(),
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

struct Navigation<'world> {
    world: &'world World,
    moves: HashMap<Point, Vec<Route>>,
}

#[derive(Clone, Debug)]
struct Route {
    to: Point,
    len: usize,
}

impl<'world> Navigation<'world> {
    fn available_moves(&mut self, from: Point) -> &Vec<Route> {
        if self.moves.get(&from).is_none() {
            let mut moves: Vec<Route> = Vec::new();
            let mut visited: HashSet<Point> = HashSet::new();
            let mut queue: VecDeque<Route> = VecDeque::new();
            queue.push_back(Route { to: from, len: 0 });

            while let Some(route) = queue.pop_front() {
                for next_pos in adjacent(route.to) {
                    if !visited.contains(&next_pos) {
                        let next = Route {
                            to: next_pos,
                            len: route.len + 1,
                        };
                        visited.insert(next_pos);

                        match self.world.tiles.get(&next_pos) {
                            Some(Tile::Floor) => {
                                queue.push_back(next);
                            }

                            Some(Tile::Warp(_)) => {
                                if route.to != self.world.start && route.to != from {
                                    moves.push(next);
                                }
                            }

                            Some(Tile::Wall) | None => {}
                        }
                    }
                }
            }

            self.moves.insert(from, moves);
        }

        self.moves.get(&from).unwrap()
    }
}

fn dijkstra(world: &World, levels: bool) -> usize {
    let mut queue: BinaryHeap<State> = BinaryHeap::new();

    queue.push(State {
        loc: Loc {
            pos: world.start,
            level: 0,
        },
        len: 0,
    });

    let mut visited = HashSet::new();
    let mut nav = Navigation {
        world: &world,
        moves: HashMap::new(),
    };

    while let Some(state) = queue.pop() {
        if state.loc.pos == world.goal && state.loc.level == 0 {
            return state.len;
        } else {
            visited.insert(state.loc);
            for (next_loc, next_len) in steps_from(world, &mut nav, &state.loc, levels) {
                if !visited.contains(&next_loc) {
                    let next_state = State {
                        loc: next_loc,
                        len: state.len + next_len,
                    };
                    queue.push(next_state);
                }
            }
        }
    }

    unreachable!()
}

fn solve_a(world: &World) -> usize {
    dijkstra(world, false) - 2
}

fn solve_b(world: &World) -> usize {
    dijkstra(world, true) - 2
}

pub fn solve(lines: &[String]) -> Solution {
    let world = World::parse(lines);
    let a_solution = solve_a(&world);
    let b_solution = solve_b(&world);
    (a_solution.to_string(), b_solution.to_string())
}
