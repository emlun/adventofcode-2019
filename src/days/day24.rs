use crate::common::Solution;
use std::collections::HashMap;
use std::collections::HashSet;

fn parse(lines: &[String]) -> Vec<Vec<bool>> {
    lines
        .iter()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn parse_a(lines: &[String]) -> Vec<Vec<bool>> {
    let mut state = parse(lines);
    for row in state.iter_mut() {
        row.insert(0, false);
        row.push(false);
    }
    state.insert(0, vec![false; 7]);
    state.push(vec![false; 7]);
    state
}

fn print_state(state: &Vec<Vec<bool>>) {
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

fn print_levels(state: &HashMap<i32, Vec<Vec<bool>>>) {
    let minl = *state.keys().min().unwrap();
    let maxl = *state.keys().max().unwrap();
    for level in minl..=maxl {
        let s: String = state
            .get(&level)
            .unwrap()
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| if *c { "#" } else { "." }.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n");
        println!("\nLevel {}:\n{}", level, s);
    }
}

fn score(state: &Vec<Vec<bool>>) -> u128 {
    state
        .iter()
        .enumerate()
        .filter(|(y, _)| *y > 0 && *y < 6)
        .flat_map(|(_, vy)| {
            vy.iter()
                .enumerate()
                .filter(|(x, _)| *x > 0 && *x < 6)
                .map(|(_, vx)| vx)
        })
        .enumerate()
        .map(|(i, v)| if *v { 1 << i } else { 0 })
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

fn update_b(
    mut state: HashMap<i32, Vec<Vec<bool>>>,
    mut next_state: HashMap<i32, Vec<Vec<bool>>>,
) -> (HashMap<i32, Vec<Vec<bool>>>, HashMap<i32, Vec<Vec<bool>>>) {
    let maxi = 5;
    let lmin = state.keys().min().unwrap();
    let lmax = state.keys().max().unwrap();
    let empty_lvl = empty_level();
    for level in (lmin - 1)..=(lmax + 1) {
        for y in 1..=maxi {
            for x in 1..=maxi {
                if x == 3 && y == 3 {
                    continue;
                }

                fn count_neighbors(
                    state: &HashMap<i32, Vec<Vec<bool>>>,
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
                    .map(|(x2, y2)| count_neighbors(&state, &empty_lvl, x, y, level, *x2, *y2))
                    .sum();
                // println!(
                //     "Cell {}:{:?} has {} live neighbors",
                //     level,
                //     (x, y),
                //     neighbors
                // );
                next_state.entry(level).or_insert_with(empty_level)[y][x] =
                    if state.get(&level).unwrap_or(&empty_lvl)[y][x] {
                        neighbors == 1
                    } else {
                        neighbors == 1 || neighbors == 2
                    };
            }
        }
    }
    (next_state, state)
}

fn solve_a(lines: &[String]) -> u128 {
    let initial_state = parse_a(lines);
    print_state(&initial_state);

    let mut state = initial_state.clone();
    let mut tmp = initial_state.clone();
    let mut seen: HashSet<u128> = HashSet::new();
    loop {
        let sc = score(&state);
        if seen.contains(&sc) {
            break sc;
        }
        seen.insert(sc);
        // println!();
        // print_state(&state);
        let o = update(state, tmp);
        state = o.0;
        tmp = o.1;
    }
}

fn solve_b(lines: &[String]) -> usize {
    let initial_state = parse_a(lines);
    let mut state: HashMap<i32, Vec<Vec<bool>>> = HashMap::new();
    state.insert(0, initial_state);
    let mut tmp = state.clone();

    for _ in 0..200 {
        println!();
        print_levels(&state);
        let o = update_b(state, tmp);
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
    let a_solution = solve_a(lines);
    let b_solution = solve_b(lines);
    (a_solution.to_string(), b_solution.to_string())
}
