use crate::common::Solution;
use crate::intcode::IntcodeComputer;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Point = (i64, i64);

const ENABLE_OUTPUT: bool = false;

fn add(p1: &Point, p2: &Point) -> Point {
    (p1.0 + p2.0, p1.1 + p2.1)
}

#[derive(Eq, PartialEq)]
enum Tile {
    Floor(u32),
    Wall,
    Goal(u32),
}

fn rotate_cw(dir: &Point) -> Point {
    (dir.1, -dir.0)
}

fn rotate_ccw(dir: &Point) -> Point {
    (-dir.1, dir.0)
}

fn adjacent(pos: &Point) -> Vec<Point> {
    vec![
        add(pos, &(1, 0)),
        add(pos, &(0, 1)),
        add(pos, &(-1, 0)),
        add(pos, &(0, -1)),
    ]
}

fn dir_to_cmd(dir: Point) -> i64 {
    match dir {
        (1, 0) => 4,
        (0, 1) => 1,
        (-1, 0) => 3,
        (0, -1) => 2,
        _ => unreachable!(),
    }
}

struct State {
    pos: Point,
    dir: Point,
    dist: u32,
    computer: IntcodeComputer,
}

struct World {
    tiles: HashMap<Point, Tile>,
    goal_pos: Option<Point>,
}
impl World {
    fn new() -> World {
        let mut tiles = HashMap::new();
        tiles.insert((0, 0), Tile::Floor(0));
        World {
            tiles,
            goal_pos: None,
        }
    }
}

fn print_state(state: &State, world: &World) {
    let minx = *world.tiles.keys().map(|(x, _)| x).min().unwrap_or(&0);
    let maxx = *world.tiles.keys().map(|(x, _)| x).max().unwrap_or(&0);
    let miny = *world.tiles.keys().map(|(_, y)| y).min().unwrap_or(&0);
    let maxy = *world.tiles.keys().map(|(_, y)| y).max().unwrap_or(&0);

    println!(
        "{}",
        (miny..=maxy)
            .rev()
            .rev()
            .map(|y| {
                (minx..=maxx)
                    .map(|x| {
                        if (x, y) == state.pos {
                            match state.dir {
                                (1, 0) => ">",
                                (-1, 0) => "<",
                                (0, 1) => "v",
                                (0, -1) => "^",
                                _ => unreachable!(),
                            }
                        } else {
                            match world.tiles.get(&(x, y)) {
                                None => " ",
                                Some(Tile::Floor(_)) => ".",
                                Some(Tile::Wall) => "#",
                                Some(Tile::Goal(_)) => "X",
                            }
                        }
                    })
                    .collect::<Vec<&str>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n")
    );
}

fn print_distances(world: &World) {
    let minx = *world.tiles.keys().map(|(x, _)| x).min().unwrap_or(&0);
    let maxx = *world.tiles.keys().map(|(x, _)| x).max().unwrap_or(&0);
    let miny = *world.tiles.keys().map(|(_, y)| y).min().unwrap_or(&0);
    let maxy = *world.tiles.keys().map(|(_, y)| y).max().unwrap_or(&0);

    println!(
        "{}",
        (miny..=maxy)
            .rev()
            .rev()
            .map(|y| {
                (minx..=maxx)
                    .map(|x| match world.tiles.get(&(x, y)) {
                        None => "    ".to_string(),
                        Some(Tile::Floor(dist)) => format!("{: >4}", dist),
                        Some(Tile::Wall) => "    ".to_string(),
                        Some(Tile::Goal(_)) => " XX ".to_string(),
                    })
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n")
    );
}

fn build_map(computer: IntcodeComputer) -> World {
    let mut queue: VecDeque<State> = VecDeque::new();
    let mut world = World::new();

    for dir in &[(0, 1), (1, 0), (0, -1)] {
        queue.push_back(State {
            pos: (0, 0),
            dir: *dir,
            dist: 0,
            computer: computer.clone(),
        });
    }
    queue.push_back(State {
        pos: (0, 0),
        dir: (-1, 0),
        dist: 0,
        computer,
    });

    while let Some(mut state) = queue.pop_front() {
        let new_pos = add(&state.pos, &state.dir);
        if !world.tiles.contains_key(&new_pos) {
            if ENABLE_OUTPUT {
                println!("{}", state.dist);
                print_state(&state, &world);
                println!();
            }

            let command = dir_to_cmd(state.dir);
            state.computer.run_mut(Some(command));

            let output = state.computer.output.pop_front().unwrap();
            if output == 0 {
                world.tiles.insert(new_pos, Tile::Wall);
            } else {
                let dist = state.dist + 1;
                world.tiles.entry(new_pos).or_insert_with(|| {
                    if output == 1 {
                        Tile::Floor(dist)
                    } else {
                        Tile::Goal(dist)
                    }
                });

                if output == 2 {
                    world.goal_pos = Some(new_pos);
                }

                let next_candidates = [rotate_ccw(&state.dir), state.dir, rotate_cw(&state.dir)];
                let mut nexts: Vec<&Point> = next_candidates
                    .iter()
                    .filter(|dir| !world.tiles.contains_key(&add(&new_pos, dir)))
                    .collect();

                while nexts.len() > 1 {
                    queue.push_back(State {
                        pos: new_pos,
                        dir: *nexts.pop().unwrap(),
                        dist,
                        computer: state.computer.clone(),
                    });
                }

                if !nexts.is_empty() {
                    queue.push_back(State {
                        pos: new_pos,
                        dir: *nexts.pop().unwrap(),
                        dist,
                        computer: state.computer,
                    });
                }
            }
        }
    }

    world
}

fn solve_a(computer: IntcodeComputer) -> (World, u32) {
    let world = build_map(computer);

    let goal_dist = match world.tiles.get(&world.goal_pos.unwrap()).unwrap() {
        Tile::Goal(dist) => *dist,
        _ => unreachable!(),
    };

    if ENABLE_OUTPUT {
        print_distances(&world);
    }

    (world, goal_dist)
}

fn solve_b(world: World) -> u32 {
    let mut heads: HashSet<Point> = HashSet::new();
    let mut has_oxygen: HashSet<Point> = HashSet::new();
    heads.insert(world.goal_pos.unwrap());

    let mut time = 0;

    while !heads.is_empty() {
        time += 1;

        let new_heads = heads
            .iter()
            .flat_map(adjacent)
            .filter(|pos| {
                !has_oxygen.contains(pos)
                    && match world.tiles.get(pos) {
                        Some(Tile::Floor(_)) => true,
                        Some(Tile::Goal(_)) => true,
                        _ => false,
                    }
            })
            .collect();

        for head in heads.drain() {
            has_oxygen.insert(head);
        }
        heads = new_heads;
    }

    time - 1
}

pub fn solve(lines: &[String]) -> Solution {
    let computer: IntcodeComputer = lines.into();
    let (a_finish, a_solution) = solve_a(computer);
    (a_solution.to_string(), solve_b(a_finish).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        assert_eq!(rotate_cw(&(1, 0)), (0, -1));
        assert_eq!(rotate_ccw(&(1, 0)), (0, 1));
    }
}
