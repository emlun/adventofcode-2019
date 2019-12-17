use crate::common::Solution;
use crate::intcode::IntcodeComputer;
use std::collections::HashMap;
use std::collections::HashSet;

type Point = (i64, i64);

const ENABLE_OUTPUT: bool = false;

fn add(p1: &Point, p2: &Point) -> Point {
    (p1.0 + p2.0, p1.1 + p2.1)
}

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Path,
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

#[derive(Debug)]
struct State {
    world: HashMap<Point, Tile>,
    robot_pos: Point,
    robot_dir: Point,
    read_x: i64,
    read_y: i64,
}

impl State {
    fn new() -> State {
        State {
            world: HashMap::new(),
            robot_pos: (0, 0),
            robot_dir: (0, 1),
            read_x: 0,
            read_y: 0,
        }
    }
}

fn intersections(state: &State) -> HashSet<Point> {
    let minx = *state.world.keys().map(|(x, _)| x).min().unwrap_or(&0);
    let maxx = *state.world.keys().map(|(x, _)| x).max().unwrap_or(&0);
    let miny = *state.world.keys().map(|(_, y)| y).min().unwrap_or(&0);
    let maxy = *state.world.keys().map(|(_, y)| y).max().unwrap_or(&0);

    (miny..maxy)
        .flat_map(|y| {
            (minx..maxx).map(move |x| (x, y)).filter(|(x, y)| {
                if state.world.get(&(*x, *y)).unwrap_or(&Tile::Empty) == &Tile::Path {
                    let num_adjacent = adjacent(&(*x, *y))
                        .into_iter()
                        .filter(|(xx, yy)| {
                            state.world.get(&(*xx, *yy)).unwrap_or(&Tile::Empty) == &Tile::Path
                        })
                        .count();
                    num_adjacent > 2
                } else {
                    false
                }
            })
        })
        .collect()
}

fn print_state(state: &State) {
    let minx = *state.world.keys().map(|(x, _)| x).min().unwrap_or(&0);
    let maxx = *state.world.keys().map(|(x, _)| x).max().unwrap_or(&0);
    let miny = *state.world.keys().map(|(_, y)| y).min().unwrap_or(&0);
    let maxy = *state.world.keys().map(|(_, y)| y).max().unwrap_or(&0);

    let intrsct: HashSet<Point> = intersections(state);

    println!(
        "{}",
        (miny..=maxy)
            .rev()
            .rev()
            .map(|y| {
                (minx..=maxx)
                    .map(|x| {
                        if (x, y) == state.robot_pos {
                            "R"
                        } else if intrsct.contains(&(x, y)) {
                            "O"
                        } else {
                            match state.world.get(&(x, y)) {
                                None => " ",
                                Some(Tile::Empty) => ".",
                                Some(Tile::Path) => "#",
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

fn step_build_map(output: Option<i64>, mut state: State) -> (Option<i64>, State) {
    if let Some(output) = output {
        match output as u8 as char {
            '.' => {
                state
                    .world
                    .insert((state.read_x, state.read_y), Tile::Empty);
                state.read_x += 1;
            }
            '#' => {
                state.world.insert((state.read_x, state.read_y), Tile::Path);
                state.read_x += 1;
            }
            '^' | '>' | 'v' | '<' => {
                state.robot_pos = (state.read_x, state.read_y);
                state.robot_dir = match output as u8 as char {
                    '^' => (0, -1),
                    '>' => (1, 0),
                    'v' => (0, 1),
                    '<' => (-1, 0),
                    _ => unreachable!(),
                };
                state.world.insert((state.read_x, state.read_y), Tile::Path);
                state.read_x += 1;
            }
            'X' => {
                state.robot_pos = (state.read_x, state.read_y);
                state
                    .world
                    .insert((state.read_x, state.read_y), Tile::Empty);
                state.read_x += 1;
            }
            '\n' => {
                state.read_y += 1;
                state.read_x = 0;
            }
            _ => unreachable!(),
        };

        if ENABLE_OUTPUT {
            println!();
            print_state(&state);
        }
    }
    (None, state)
}

fn solve_a(computer: IntcodeComputer) -> (State, i64) {
    let finish = computer.run_with(None, State::new(), step_build_map);

    print_state(&finish);

    let intrsct = intersections(&finish);
    let solution = intrsct.into_iter().map(|(x, y)| x * y).sum::<i64>();

    (finish, solution)
}

fn solve_b(finish: State) -> u32 {
    0
}

pub fn solve(lines: &[String]) -> Solution {
    let computer: IntcodeComputer = lines.into();
    let (a_finish, a_solution) = solve_a(computer);
    (a_solution.to_string(), solve_b(a_finish).to_string())
}
