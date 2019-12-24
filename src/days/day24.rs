use crate::common::Solution;
use std::collections::HashMap;
use std::collections::HashSet;

fn parse(lines: &[String]) -> Vec<Vec<bool>> {
    [".....".to_string()]
        .iter()
        .chain(lines.iter())
        .chain([".....".to_string()].iter())
        .map(|line| {
            ".".chars()
                .chain(line.chars())
                .chain(".".chars())
                .map(|c| c == '#')
                .collect()
        })
        .collect()
}

fn print_state(state: &[Vec<bool>]) {
    let s: String = state
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| if *c { "#" } else { "." }.to_string())
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", s);
}

#[allow(dead_code)]
fn print_levels(state: &HashMap<i32, Vec<Vec<bool>>>) {
    let minl = *state.keys().min().unwrap();
    let maxl = *state.keys().max().unwrap();
    for level in minl..=maxl {
        println!("\nLevel {}", level);
        print_state(&state.get(&level).unwrap());
    }
}

fn score(state: &[Vec<bool>]) -> u128 {
    (1..=5)
        .flat_map(|y| (1..=5).map(move |x| (x, y)))
        .enumerate()
        .map(|(i, (x, y))| if state[y][x] { 1 << i } else { 0 })
        .sum()
}

fn update(
    state: Vec<Vec<bool>>,
    mut next_state: Vec<Vec<bool>>,
) -> (Vec<Vec<bool>>, Vec<Vec<bool>>) {
    for y in 1..(state.len() - 1) {
        for x in 1..(state[0].len() - 1) {
            let neighbors = [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
                .iter()
                .filter(|(x2, y2)| state[*y2][*x2])
                .count();
            next_state[y][x] = if state[y][x] {
                neighbors == 1
            } else {
                neighbors == 1 || neighbors == 2
            };
        }
    }
    (next_state, state)
}

fn empty_level() -> Vec<Vec<bool>> {
    vec![vec![false; 7]; 7]
}

type State = Vec<Vec<bool>>;
type LevelsState = HashMap<i32, State>;

fn update_b(
    state: LevelsState,
    mut next_state: LevelsState,
    empty_lvl: &Vec<Vec<bool>>,
) -> (LevelsState, LevelsState) {
    let maxi = 5;
    let lmin = state.keys().min().unwrap();
    let lmax = state.keys().max().unwrap();
    for level in (lmin - 1)..=(lmax + 1) {
        for y in 1..=maxi {
            for x in 1..=maxi {
                if x == 3 && y == 3 {
                    continue;
                }

                fn count_neighbors(
                    state: &LevelsState,
                    empty_lvl: &Vec<Vec<bool>>,
                    of_x: usize,
                    of_y: usize,
                    of_level: i32,
                    at_x: usize,
                    at_y: usize,
                ) -> usize {
                    let lv = if at_x == 3 && at_y == 3 {
                        of_level + 1
                    } else if at_x == 0 || at_x == 6 || at_y == 0 || at_y == 6 {
                        of_level - 1
                    } else {
                        of_level
                    };
                    let lvl = state.get(&lv).unwrap_or(empty_lvl);

                    if at_x == 3 && at_y == 3 {
                        if of_x == 2 {
                            (1..=5).filter(|y| lvl[*y][1]).count()
                        } else if of_x == 4 {
                            (1..=5).filter(|y| lvl[*y][5]).count()
                        } else if of_y == 2 {
                            (1..=5).filter(|x| lvl[1][*x]).count()
                        } else if of_y == 4 {
                            (1..=5).filter(|x| lvl[5][*x]).count()
                        } else {
                            unreachable!()
                        }
                    } else {
                        let cell = if at_x == 0 {
                            lvl[3][2]
                        } else if at_x == 6 {
                            lvl[3][4]
                        } else if at_y == 0 {
                            lvl[2][3]
                        } else if at_y == 6 {
                            lvl[4][3]
                        } else {
                            lvl[at_y][at_x]
                        };
                        if cell {
                            1
                        } else {
                            0
                        }
                    }
                }

                let neighbors: usize = [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
                    .iter()
                    .map(|(x2, y2)| count_neighbors(&state, empty_lvl, x, y, level, *x2, *y2))
                    .sum();
                next_state.entry(level).or_insert_with(empty_level)[y][x] =
                    if state.get(&level).unwrap_or(empty_lvl)[y][x] {
                        neighbors == 1
                    } else {
                        neighbors == 1 || neighbors == 2
                    };
            }
        }
    }
    (next_state, state)
}

fn solve_a(initial_state: Vec<Vec<bool>>) -> u128 {
    let mut state = initial_state.clone();
    let mut tmp = initial_state;
    let mut seen: HashSet<u128> = HashSet::new();
    loop {
        let sc = score(&state);
        if seen.contains(&sc) {
            break sc;
        }
        seen.insert(sc);
        let o = update(state, tmp);
        state = o.0;
        tmp = o.1;
    }
}

fn solve_b(initial_state: Vec<Vec<bool>>) -> usize {
    let mut state: HashMap<i32, Vec<Vec<bool>>> = HashMap::new();
    state.insert(0, initial_state);
    let mut tmp = state.clone();
    let empty_lvl = empty_level();

    for _ in 0..200 {
        let o = update_b(state, tmp, &empty_lvl);
        state = o.0;
        tmp = o.1;
    }

    state
        .values()
        .flat_map(|grid| grid.iter())
        .flat_map(|row| row.iter())
        .filter(|v| **v)
        .count()
}

pub fn solve(lines: &[String]) -> Solution {
    let initial_state = parse(lines);
    let a_solution = solve_a(initial_state.clone());
    let b_solution = solve_b(initial_state);
    (a_solution.to_string(), b_solution.to_string())
}
