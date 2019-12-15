use crate::common::Solution;
use crate::intcode::IntcodeComputer;
use std::collections::HashMap;
use std::collections::HashSet;

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
    world: HashMap<Point, Tile>,
    goal_pos: Option<Point>,
    pos: Point,
    dir: Point,
    unexplored: HashSet<Point>,
}

impl State {
    fn new() -> State {
        let mut world = HashMap::new();
        let pos = (0, 0);
        world.insert(pos, Tile::Floor(0));
        State {
            world,
            goal_pos: None,
            pos,
            dir: (0, 1),
            unexplored: vec![(1, 0), (0, 1), (-1, 0), (0, -1)].into_iter().collect(),
        }
    }
}

fn dist_at(world: &HashMap<Point, Tile>, pos: &Point) -> Option<u32> {
    match world.get(&pos) {
        Some(Tile::Floor(dist)) => Some(*dist),
        Some(Tile::Goal(dist)) => Some(*dist),
        Some(Tile::Wall) => None,
        None => None,
    }
}

fn print_state(state: &State) {
    let minx = *state.world.keys().map(|(x, _)| x).min().unwrap_or(&0);
    let maxx = *state.world.keys().map(|(x, _)| x).max().unwrap_or(&0);
    let miny = *state.world.keys().map(|(_, y)| y).min().unwrap_or(&0);
    let maxy = *state.world.keys().map(|(_, y)| y).max().unwrap_or(&0);

    println!(
        "{}",
        (miny..=maxy)
            .rev()
            .rev()
            .map(|y| {
                (minx..=maxx)
                    .map(|x| {
                        if (x, y) == state.pos {
                            "D"
                        } else {
                            match state.world.get(&(x, y)) {
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

fn print_distances(state: &State) {
    let minx = *state.world.keys().map(|(x, _)| x).min().unwrap_or(&0);
    let maxx = *state.world.keys().map(|(x, _)| x).max().unwrap_or(&0);
    let miny = *state.world.keys().map(|(_, y)| y).min().unwrap_or(&0);
    let maxy = *state.world.keys().map(|(_, y)| y).max().unwrap_or(&0);

    println!(
        "{}",
        (miny..=maxy)
            .rev()
            .rev()
            .map(|y| {
                (minx..=maxx)
                    .map(|x| match state.world.get(&(x, y)) {
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

fn step_build_map(output: Option<i64>, mut state: State) -> (Option<i64>, State, bool) {
    let command = dir_to_cmd(state.dir);
    if let Some(output) = output {
        match output {
            0 => {
                let wall_pos = add(&state.pos, &state.dir);
                state.world.insert(wall_pos, Tile::Wall);
                state.unexplored.remove(&wall_pos);
                state.dir = rotate_cw(&state.dir);
            }
            1 | 2 => {
                let new_pos = add(&state.pos, &state.dir);
                let dist = adjacent(&new_pos)
                    .into_iter()
                    .flat_map(|prev_pos| dist_at(&state.world, &prev_pos))
                    .min()
                    .unwrap()
                    + 1;

                if !state.world.contains_key(&new_pos) {
                    state.world.insert(
                        new_pos,
                        if output == 1 {
                            Tile::Floor(dist)
                        } else {
                            Tile::Goal(dist)
                        },
                    );
                }

                state.unexplored.remove(&new_pos);
                for unexplored_tile in &[
                    add(&state.pos, &state.dir),
                    add(&state.pos, &rotate_cw(&state.dir)),
                ] {
                    if !state.world.contains_key(unexplored_tile) {
                        state.unexplored.insert(*unexplored_tile);
                    }
                }
                state.dir = rotate_ccw(&state.dir);
                state.pos = new_pos;

                if output == 2 {
                    state.goal_pos = Some(state.pos);
                }
            }
            _ => unreachable!(),
        };

        if ENABLE_OUTPUT {
            println!("\n{}", state.unexplored.len());
            if state.unexplored.len() < 10 {
                println!("{:?}", state.unexplored);
            }
            print_state(&state);
        }
    }
    let halt = state.unexplored.is_empty();
    (Some(command), state, halt)
}

fn solve_a(computer: IntcodeComputer) -> (State, u32) {
    let finish = computer.run_with_halt(
        Some(dir_to_cmd(State::new().dir)),
        State::new(),
        step_build_map,
    );

    let goal_dist = match finish.world.get(&finish.goal_pos.unwrap()).unwrap() {
        Tile::Goal(dist) => *dist,
        _ => unreachable!(),
    };

    if ENABLE_OUTPUT {
        print_distances(&finish);
    }

    (finish, goal_dist)
}

fn solve_b(finish: State) -> u32 {
    let mut heads: HashSet<Point> = HashSet::new();
    let mut has_oxygen: HashSet<Point> = HashSet::new();
    heads.insert(finish.goal_pos.unwrap());

    let mut time = 0;

    while !heads.is_empty() {
        time += 1;

        let new_heads = heads
            .iter()
            .flat_map(adjacent)
            .filter(|pos| {
                !has_oxygen.contains(pos)
                    && match finish.world.get(pos) {
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
