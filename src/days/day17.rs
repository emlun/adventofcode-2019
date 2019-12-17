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

fn rotate_ccw(dir: &Point) -> Point {
    (dir.1, -dir.0)
}

fn rotate_cw(dir: &Point) -> Point {
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

fn intersections(world: &HashMap<Point, Tile>) -> HashSet<Point> {
    let minx = *world.keys().map(|(x, _)| x).min().unwrap_or(&0);
    let maxx = *world.keys().map(|(x, _)| x).max().unwrap_or(&0);
    let miny = *world.keys().map(|(_, y)| y).min().unwrap_or(&0);
    let maxy = *world.keys().map(|(_, y)| y).max().unwrap_or(&0);

    (miny..maxy)
        .flat_map(|y| {
            (minx..maxx).map(move |x| (x, y)).filter(|(x, y)| {
                if world.get(&(*x, *y)).unwrap_or(&Tile::Empty) == &Tile::Path {
                    let num_adjacent = adjacent(&(*x, *y))
                        .into_iter()
                        .filter(|(xx, yy)| {
                            world.get(&(*xx, *yy)).unwrap_or(&Tile::Empty) == &Tile::Path
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

    let intrsct: HashSet<Point> = intersections(&state.world);

    println!(
        "{}",
        vec![format!(
            "    {}",
            (minx..=maxx)
                .map(|x| (x.abs() % 100 / 10).to_string())
                .collect::<Vec<String>>()
                .join("")
        )]
        .into_iter()
        .chain(
            vec![format!(
                "    {}",
                (minx..=maxx)
                    .map(|x| (x.abs() % 10).to_string())
                    .collect::<Vec<String>>()
                    .join("")
            )]
            .into_iter()
        )
        .chain((miny..=maxy).rev().rev().map(|y| {
            format!(
                "{: >3} {}",
                y,
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
            )
        }))
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

    let intrsct = intersections(&finish.world);
    let solution = intrsct.into_iter().map(|(x, y)| x * y).sum::<i64>();

    (finish, solution)
}

type Route = Vec<Step>;
#[derive(Debug)]
enum Step {
    F(usize),
    R(usize),
    L(usize),
}

fn is_path(world: &HashMap<Point, Tile>, pos: &Point) -> bool {
    world.get(pos).unwrap_or(&Tile::Empty) == &Tile::Path
}

fn compress_route(route: Route) -> Route {
    use Step::{F, L, R};
    route.into_iter().fold(Vec::new(), |mut rt, step| {
        if rt.len() > 0 {
            let endi = rt.len() - 1;
            match (&rt[endi], &step) {
                (F(f1), F(f2)) => rt[endi] = F(f1 + f2),
                (L(l1), F(f2)) => rt[endi] = L(l1 + f2),
                (R(r1), F(f2)) => rt[endi] = R(r1 + f2),
                _ => rt.push(step),
            };
        } else {
            rt.push(step);
        }
        rt
    })
}

fn follow_path_to_next_intersection(
    world: &HashMap<Point, Tile>,
    intrsct: &HashSet<Point>,
    start: Point,
    start_dir: Point,
) -> (Point, Route) {
    let mut pos = start;
    let mut dir = start_dir;
    let mut route = Vec::new();
    while !intrsct.contains(&pos) || route.is_empty() {
        let next = add(&pos, &dir);
        if is_path(world, &next) {
            route.push(Step::F(1));
            pos = next;
        } else {
            let dir_left = rotate_ccw(&dir);
            if is_path(world, &add(&pos, &dir_left)) {
                route.push(Step::L(0));
                dir = dir_left;
            } else {
                let dir_right = rotate_cw(&dir);
                route.push(Step::R(0));
                dir = dir_right;
            }
        }
    }
    (pos, compress_route(route))
}

fn simplest_path(
    world: &HashMap<Point, Tile>,
    start_pos: Point,
    start_dir: Point,
) -> Option<Route> {
    let mut pos = start_pos;
    let mut dir = start_dir;
    let mut route = Vec::new();
    let paths: HashSet<Point> = world
        .iter()
        .filter(|(_, v)| **v == Tile::Path)
        .map(|(k, _)| *k)
        .collect();
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(pos);

    loop {
        let next = add(&pos, &dir);
        if is_path(world, &next) {
            route.push(Step::F(1));
            pos = next;
            visited.insert(pos);
        } else {
            let dir_left = rotate_ccw(&dir);
            if is_path(world, &add(&pos, &dir_left)) {
                route.push(Step::L(0));
                dir = dir_left;
            } else {
                let dir_right = rotate_cw(&dir);
                if is_path(world, &add(&pos, &dir_right)) {
                    route.push(Step::R(0));
                    dir = dir_right;
                } else if visited == paths {
                    return Some(compress_route(route));
                } else {
                    return None;
                }
            }
        }
    }
}

fn intersection_transfers(world: &HashMap<Point, Tile>) -> HashMap<Point, Vec<(Point, Route)>> {
    let intrsct = intersections(world);
    intrsct
        .iter()
        .map(|start| {
            let routes = vec![(1, 0), (0, 1), (-1, 0), (0, -1)]
                .into_iter()
                .filter(|dir| world.get(&add(&start, &dir)).unwrap_or(&Tile::Empty) == &Tile::Path)
                .map(|dir| follow_path_to_next_intersection(world, &intrsct, *start, dir))
                .collect();
            (*start, routes)
        })
        .collect()
}

fn solve_b(finish_a: State, mut computer: IntcodeComputer) -> i64 {
    computer.prog[0] = 2;

    for (start, routes) in intersection_transfers(&finish_a.world) {
        for (end, route) in routes {
            println!(
                "({: >2},{: >2}) => ({: >2},{: >2}) : {:?}",
                start.0, start.1, end.0, end.1, route
            );
        }
    }

    println!(
        "{:?}",
        simplest_path(&finish_a.world, finish_a.robot_pos, finish_a.robot_dir)
    );

    let my_segments = {
        use Step::{L, R};
        vec![
            vec![R(12), L(8), R(6)],
            vec![L(8), R(8), R(6), R(12)],
            vec![R(12), L(6), R(6), R(8), R(6)],
        ]
    };
    let my_sequence = vec!['A', 'A', 'C', 'B', 'A', 'B', 'A', 'C', 'B', 'C'];

    let mut input_sequence = Vec::new();
    for cmd in my_sequence {
        input_sequence.push(cmd as u8 as i64);
        input_sequence.push(',' as u8 as i64);
    }
    input_sequence.remove(input_sequence.len() - 1);
    input_sequence.push('\n' as u8 as i64);
    for seg in my_segments {
        for cmd in seg {
            let chars: Vec<char> = match cmd {
                Step::F(d) => d.to_string().chars().collect(),
                Step::L(d) => vec!['L', ',']
                    .into_iter()
                    .chain(d.to_string().chars())
                    .collect(),
                Step::R(d) => vec!['R', ',']
                    .into_iter()
                    .chain(d.to_string().chars())
                    .collect(),
            };
            let mut chars: Vec<i64> = chars.into_iter().map(|c| c as u8 as i64).collect();
            input_sequence.append(&mut chars);
            input_sequence.push(',' as u8 as i64);
        }
        input_sequence.remove(input_sequence.len() - 1);
        input_sequence.push('\n' as u8 as i64);
    }
    input_sequence.push('n' as u8 as i64);
    input_sequence.push('\n' as u8 as i64);

    let output = computer.run(input_sequence);
    output[output.len() - 1]
}

pub fn solve(lines: &[String]) -> Solution {
    let computer: IntcodeComputer = lines.into();
    let (a_finish, a_solution) = solve_a(computer.clone());
    let b_solution = solve_b(a_finish, computer);
    (a_solution.to_string(), b_solution.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        assert_eq!(rotate_cw(&(1, 0)), (0, 1));
        assert_eq!(rotate_ccw(&(1, 0)), (0, -1));
    }
}
