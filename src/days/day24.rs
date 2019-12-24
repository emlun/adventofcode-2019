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

#[derive(Clone)]
struct LevelsState {
    levels: Vec<State>,
    empty_level: State,
    min_level: i32,
    max_level: i32,
}

impl LevelsState {
    fn new(state: State) -> Self {
        LevelsState {
            levels: vec![state],
            empty_level: empty_level(),
            min_level: 0,
            max_level: 0,
        }
    }

    fn get(&self, level: i32) -> &State {
        let index = Self::level_to_index(level);
        if index < self.levels.len() {
            &self.levels[index]
        } else {
            &self.empty_level
        }
    }

    fn get_mut(&mut self, level: i32) -> &mut State {
        let index = Self::level_to_index(level);
        if self.levels.len() <= index {
            self.levels
                .append(&mut (self.levels.len()..=index).map(|_| empty_level()).collect());
        }
        if level < self.min_level {
            self.min_level = level;
        }
        if level > self.max_level {
            self.max_level = level;
        }
        &mut self.levels[index]
    }

    fn level_to_index(level: i32) -> usize {
        level.abs() as usize * 2 - ((level < 0) as usize)
    }
}

fn update_b(state: LevelsState, mut next_state: LevelsState) -> (LevelsState, LevelsState) {
    let maxi = 5;
    for level in (state.min_level - 1)..=(state.max_level + 1) {
        for y in 1..=maxi {
            for x in 1..=maxi {
                if x == 3 && y == 3 {
                    continue;
                }

                fn count_neighbors(
                    state: &LevelsState,
                    of_x: usize,
                    of_y: usize,
                    of_level: i32,
                    at_x: usize,
                    at_y: usize,
                ) -> usize {
                    let lv = match (at_x, at_y) {
                        (3, 3) => of_level + 1,
                        (0, _) | (6, _) | (_, 0) | (_, 6) => of_level - 1,
                        _ => of_level,
                    };
                    let lvl = state.get(lv);

                    match (at_x, at_y, of_x, of_y) {
                        (3, 3, 2, _) => (1..=5).filter(|y| lvl[*y][1]).count(),
                        (3, 3, 4, _) => (1..=5).filter(|y| lvl[*y][5]).count(),
                        (3, 3, _, 2) => (1..=5).filter(|x| lvl[1][*x]).count(),
                        (3, 3, _, 4) => (1..=5).filter(|x| lvl[5][*x]).count(),
                        (0, _, _, _) => lvl[3][2] as usize,
                        (6, _, _, _) => lvl[3][4] as usize,
                        (_, 0, _, _) => lvl[2][3] as usize,
                        (_, 6, _, _) => lvl[4][3] as usize,
                        _ => lvl[at_y][at_x] as usize,
                    }
                }

                let neighbors: usize = [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
                    .iter()
                    .map(|(x2, y2)| count_neighbors(&state, x, y, level, *x2, *y2))
                    .sum();
                next_state.get_mut(level)[y][x] = if state.get(level)[y][x] {
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
    let mut state = LevelsState::new(initial_state);
    let mut tmp = state.clone();

    for _ in 0..200 {
        let o = update_b(state, tmp);
        state = o.0;
        tmp = o.1;
    }

    state
        .levels
        .iter()
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
