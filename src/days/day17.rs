use crate::common::Solution;
use crate::intcode::IntcodeComputer;
use std::collections::HashSet;
use std::collections::LinkedList;

type Point = (i64, i64);

const ENABLE_OUTPUT: bool = false;

fn add(p1: &Point, p2: &Point) -> Point {
    (p1.0 + p2.0, p1.1 + p2.1)
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
    world: HashSet<Point>,
    robot_pos: Point,
    robot_dir: Point,
}

impl State {
    fn new() -> State {
        State {
            world: HashSet::new(),
            robot_pos: (0, 0),
            robot_dir: (0, 1),
        }
    }
}

fn intersections(world: &HashSet<Point>) -> HashSet<Point> {
    world
        .iter()
        .filter(|(x, y)| {
            if world.contains(&(*x, *y)) {
                let num_adjacent = adjacent(&(*x, *y))
                    .into_iter()
                    .filter(|(xx, yy)| world.contains(&(*xx, *yy)))
                    .count();
                num_adjacent > 2
            } else {
                false
            }
        })
        .copied()
        .collect()
}

fn print_state(state: &State) {
    let minx = *state.world.iter().map(|(x, _)| x).min().unwrap_or(&0);
    let maxx = *state.world.iter().map(|(x, _)| x).max().unwrap_or(&0);
    let miny = *state.world.iter().map(|(_, y)| y).min().unwrap_or(&0);
    let maxy = *state.world.iter().map(|(_, y)| y).max().unwrap_or(&0);

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
                        } else if state.world.contains(&(x, y)) {
                            "#"
                        } else {
                            "."
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

fn build_map(mut computer: IntcodeComputer) -> State {
    let mut state = State::new();

    let mut read_x = 0;
    let mut read_y = 0;

    while computer.is_running() {
        computer.run_mut(None);

        for output in computer.output.drain(..) {
            match output as u8 as char {
                '.' => {
                    read_x += 1;
                }
                '#' => {
                    state.world.insert((read_x, read_y));
                    read_x += 1;
                }
                '^' | '>' | 'v' | '<' => {
                    state.robot_pos = (read_x, read_y);
                    state.robot_dir = match output as u8 as char {
                        '^' => (0, -1),
                        '>' => (1, 0),
                        'v' => (0, 1),
                        '<' => (-1, 0),
                        _ => unreachable!(),
                    };
                    state.world.insert((read_x, read_y));
                    read_x += 1;
                }
                'X' => {
                    state.robot_pos = (read_x, read_y);
                    read_x += 1;
                }
                '\n' => {
                    read_y += 1;
                    read_x = 0;
                }
                _ => unreachable!(),
            };
        }

        if ENABLE_OUTPUT {
            println!();
            print_state(&state);
        }
    }
    state
}

fn solve_a(computer: IntcodeComputer) -> (State, i64) {
    let finish = build_map(computer);

    let intrsct = intersections(&finish.world);
    let solution = intrsct.into_iter().map(|(x, y)| x * y).sum::<i64>();

    (finish, solution)
}

type Route = Vec<Step>;
#[derive(Clone, Debug, PartialEq)]
enum Step {
    F(usize),
    R(usize),
    L(usize),
}

fn is_path(world: &HashSet<Point>, pos: &Point) -> bool {
    world.contains(pos)
}

fn compress_route(route: Route) -> Route {
    use Step::{F, L, R};
    route.into_iter().fold(Vec::new(), |mut rt, step| {
        if rt.is_empty() {
            rt.push(step);
        } else {
            let endi = rt.len() - 1;
            match (&rt[endi], &step) {
                (F(f1), F(f2)) => rt[endi] = F(f1 + f2),
                (L(l1), F(f2)) => rt[endi] = L(l1 + f2),
                (R(r1), F(f2)) => rt[endi] = R(r1 + f2),
                _ => rt.push(step),
            };
        }
        rt
    })
}

fn simplest_path(world: &HashSet<Point>, start_pos: Point, start_dir: Point) -> Route {
    let mut pos = start_pos;
    let mut dir = start_dir;
    let mut route = Vec::new();

    loop {
        let next = add(&pos, &dir);
        if is_path(world, &next) {
            route.push(Step::F(1));
            pos = next;
        } else {
            let dir_left = rotate_ccw(&dir);
            if is_path(world, &add(&pos, &dir_left)) {
                route.push(Step::L(1));
                dir = dir_left;
                pos = add(&pos, &dir);
            } else {
                let dir_right = rotate_cw(&dir);
                if is_path(world, &add(&pos, &dir_right)) {
                    route.push(Step::R(1));
                    dir = dir_right;
                    pos = add(&pos, &dir);
                } else {
                    return route;
                }
            }
        }
    }
}

fn subsequence_exists<T>(seq: &[T], subseq: &[T]) -> bool
where
    T: PartialEq,
{
    (0..(seq.len() - subseq.len())).any(|i| seq[i..].starts_with(subseq))
}

fn find_longest_repeated_subseq<T>(seq: &[T]) -> Option<&[T]>
where
    T: PartialEq,
{
    let mut end_min = 0;
    let mut end_max = seq.len();

    while end_max > end_min {
        let end = (end_max + end_min) / 2;
        if end == end_min {
            break;
        } else if subsequence_exists(&seq[end..], &seq[0..end]) {
            end_min = end;
        } else {
            end_max = end;
        }
    }

    if end_min > 0 {
        Some(&seq[0..end_min])
    } else {
        None
    }
}

fn find_subseq_covering<T>(seq: &[T], subseqs: &[&[T]]) -> Option<LinkedList<usize>>
where
    T: PartialEq,
{
    if seq.is_empty() {
        Some(LinkedList::new())
    } else {
        for (i, subseq) in subseqs.iter().enumerate() {
            if seq.starts_with(subseq) {
                if let Some(mut subfind) = find_subseq_covering(&seq[subseq.len()..], subseqs) {
                    subfind.push_front(i);
                    return Some(subfind);
                }
            }
        }

        None
    }
}

fn find_covering_subseqs<T>(seq: &[T], num_subseqs: usize) -> Option<(Vec<&[T]>, LinkedList<usize>)>
where
    T: PartialEq,
{
    fn fill_subseqs<'a, T>(
        seq: &'a [T],
        num_subseqs: usize,
        mut subseqs: Vec<&'a [T]>,
    ) -> Vec<&'a [T]>
    where
        T: PartialEq,
    {
        if seq.is_empty() || subseqs.len() == num_subseqs {
            subseqs
        } else if let Some(prefix) = subseqs.iter().find(|subseq| seq.starts_with(subseq)) {
            fill_subseqs(&seq[prefix.len()..], num_subseqs, subseqs)
        } else {
            let next = find_longest_repeated_subseq(seq).unwrap();
            subseqs.push(next);
            fill_subseqs(&seq[next.len()..], num_subseqs, subseqs)
        }
    }

    let mut subseqs: Vec<&[T]> = fill_subseqs(seq, num_subseqs, Vec::new());

    while !subseqs[0].is_empty() {
        if let Some(covering) = find_subseq_covering(seq, &subseqs) {
            return Some((subseqs, covering));
        } else {
            while !subseqs.is_empty() {
                let i = subseqs.len() - 1;
                subseqs[i] = subseqs[i].split_last().unwrap().1;
                if subseqs[subseqs.len() - 1].is_empty() {
                    subseqs.pop();
                } else {
                    subseqs = fill_subseqs(seq, num_subseqs, subseqs);
                    break;
                }
            }

            if subseqs.is_empty() {
                return None;
            }
        }
    }
    None
}

fn solve_b(finish_a: State, mut computer: IntcodeComputer) -> i64 {
    computer.prog[0] = 2;

    let full_route = simplest_path(&finish_a.world, finish_a.robot_pos, finish_a.robot_dir);
    let compressed_route = compress_route(full_route.clone());

    let (segments, mut sequence) = find_covering_subseqs(&compressed_route, 3)
        .or_else(|| find_covering_subseqs(&full_route, 3))
        .expect("Found no solution!");

    let mut input_sequence = vec!['A' as i64 + sequence.pop_front().unwrap() as i64];
    for i in sequence {
        input_sequence.push(b',' as i64);
        input_sequence.push('A' as i64 + i as i64);
    }
    input_sequence.push(b'\n' as i64);
    for seg in segments {
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
            input_sequence.append(&mut chars.into_iter().map(|c| c as u8 as i64).collect());
            input_sequence.push(b',' as i64);
        }
        input_sequence.pop();
        input_sequence.push(b'\n' as i64);
    }
    input_sequence.push(b'n' as i64);
    input_sequence.push(b'\n' as i64);

    let output = computer.run(input_sequence).output;
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
