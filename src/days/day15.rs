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
    start_pos: Point,
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
            start_pos: pos,
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
        "\n{}",
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
                state.world.insert(
                    new_pos,
                    if output == 1 {
                        Tile::Floor(dist)
                    } else {
                        Tile::Goal(dist)
                    },
                );
                state.unexplored.remove(&new_pos);
                for unexplored_tile in &[
                    add(&state.pos, &state.dir),
                    add(&state.pos, &rotate_cw(&state.dir)),
                ] {
                    if !state.world.contains_key(&unexplored_tile) {
                        state.unexplored.insert(add(&state.pos, &state.dir));
                    }
                }
                state.dir = rotate_ccw(&state.dir);
                state.pos = new_pos;

                if output == 2 {
                    state.goal_pos = Some(state.pos);
                    print_state(&state);
                }
            }
            _ => unreachable!(),
        };

        if ENABLE_OUTPUT {
            print_state(&state);
        }
    }
    let halt = state.unexplored.is_empty();
    (Some(command), state, halt)
}

fn solve_a(computer: IntcodeComputer) -> u32 {
    let finish = computer.run_with_halt(
        Some(dir_to_cmd(State::new().dir)),
        State::new(),
        step_build_map,
    );

    println!("\n{}", finish.unexplored.len());
    print_state(&finish);

    let goal_dist = match finish.world.get(&finish.goal_pos.unwrap()).unwrap() {
        Tile::Goal(dist) => *dist,
        _ => unreachable!(),
    };
    let dist_reduction = adjacent(&finish.start_pos)
        .into_iter()
        .flat_map(|pos| dist_at(&finish.world, &pos))
        .max()
        .unwrap();
    println!("{} {}", goal_dist, dist_reduction);

    goal_dist - dist_reduction + 1
}

fn solve_b(_: IntcodeComputer) -> i64 {
    0
}

pub fn solve(lines: &[String]) -> Solution {
    let computer: IntcodeComputer = lines.into();
    (
        solve_a(computer.clone()).to_string(),
        solve_b(computer).to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        assert_eq!(rotate_cw((1, 0)), (0, -1));
        assert_eq!(rotate_ccw((1, 0)), (0, 1));
    }
}
