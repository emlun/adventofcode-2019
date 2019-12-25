use crate::common::Solution;
use std::collections::HashMap;
use std::collections::HashSet;

fn parse(lines: &[String]) -> BoolMatrix {
    [".....".to_string()]
        .iter()
        .chain(lines.iter())
        .chain([".....".to_string()].iter())
        .flat_map(|line| {
            ".".chars()
                .chain(line.chars())
                .chain(".".chars())
                .map(|c| c == '#')
        })
        .collect::<BoolMatrixBuilder>()
        .dim(7)
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

fn score(state: &State) -> u64 {
    (1..=5)
        .flat_map(|y| (1..=5).map(move |x| (x, y)))
        .enumerate()
        .map(|(i, (x, y))| if state.get(x, y) { 1 << i } else { 0 })
        .sum()
}

fn update(state: State, mut next_state: State) -> (State, State) {
    for y in 1..=5 {
        for x in 1..=5 {
            let neighbors = state.count_neighbors(x, y);
            next_state.set(x, y, neighbors == 1 || (neighbors == 2 && !state.get(x, y)));
        }
    }
    (next_state, state)
}

type State = BoolMatrix;

#[derive(Clone)]
struct BoolMatrix {
    dim: usize,
    value: u64,
    neighbor_mask: u64,
}

impl BoolMatrix {
    fn new(dim: usize) -> Self {
        let neighbor_mask = 2 | (1 << dim) | (4 << dim) | (2 << (2 * dim));
        println!("{:b}", neighbor_mask);
        Self {
            dim,
            value: 0,
            neighbor_mask,
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        (self.value >> self.coords_to_index(x, y)) & 1 != 0
    }

    fn count_neighbors(&self, x: usize, y: usize) -> u32 {
        debug_assert!(x > 0);
        debug_assert!(y > 0);
        ((self.value >> self.coords_to_index(x - 1, y - 1)) & self.neighbor_mask).count_ones()
    }

    fn set(&mut self, x: usize, y: usize, value: bool) {
        debug_assert!(x < self.dim);
        debug_assert!(y < self.dim);
        let mask = 1 << self.coords_to_index(x, y);
        if value {
            self.value |= mask;
        } else {
            self.value &= !mask;
        }
    }

    fn coords_to_index(&self, x: usize, y: usize) -> usize {
        debug_assert!(x < self.dim);
        debug_assert!(y < self.dim);
        y * self.dim + x
    }
}

struct BoolMatrixBuilder {
    value: u64,
}

impl BoolMatrixBuilder {
    fn dim(self, dim: usize) -> BoolMatrix {
        let mut m = BoolMatrix::new(dim);
        m.value = self.value;
        m
    }
}

impl std::iter::FromIterator<bool> for BoolMatrixBuilder {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        BoolMatrixBuilder {
            value: iter
                .into_iter()
                .enumerate()
                .fold(0, |matrix, (i, next)| matrix | ((next as u64) * (1 << i))),
        }
    }
}

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
            empty_level: BoolMatrix::new(7),
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
            self.levels.append(
                &mut (self.levels.len()..=index)
                    .map(|_| BoolMatrix::new(7))
                    .collect(),
            );
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
    const MAXI: usize = 5;
    for level in (state.min_level - 1)..=(state.max_level + 1) {
        for y in 1..=MAXI {
            for x in 1..=MAXI {
                if x == 3 && y == 3 {
                    continue;
                }

                let basic_neighbors = state.get(level).count_neighbors(x, y);

                let lvlup = state.get(level + 1);
                let lvldn = state.get(level - 1);
                let level_neighbors = match (x, y) {
                    (2, 3) => (1..=5).filter(|y| lvlup.get(1, *y)).count(),
                    (4, 3) => (1..=5).filter(|y| lvlup.get(5, *y)).count(),
                    (3, 2) => (1..=5).filter(|x| lvlup.get(*x, 1)).count(),
                    (3, 4) => (1..=5).filter(|x| lvlup.get(*x, 5)).count(),

                    (1, 1) => lvldn.get(2, 3) as usize + lvldn.get(3, 2) as usize,
                    (5, 1) => lvldn.get(4, 3) as usize + lvldn.get(3, 2) as usize,
                    (5, 5) => lvldn.get(4, 3) as usize + lvldn.get(3, 4) as usize,
                    (1, 5) => lvldn.get(2, 3) as usize + lvldn.get(3, 4) as usize,

                    (1, _) => lvldn.get(2, 3) as usize,
                    (5, _) => lvldn.get(4, 3) as usize,
                    (_, 1) => lvldn.get(3, 2) as usize,
                    (_, 5) => lvldn.get(3, 4) as usize,

                    _ => 0,
                };
                let neighbors = basic_neighbors as usize + level_neighbors;

                next_state.get_mut(level).set(
                    x,
                    y,
                    neighbors == 1 || (neighbors == 2 && !state.get(level).get(x, y)),
                );
            }
        }
    }
    (next_state, state)
}

fn solve_a(initial_state: State) -> u64 {
    let mut state = initial_state.clone();
    let mut tmp = initial_state;
    let mut seen: HashSet<u64> = HashSet::new();
    loop {
        if seen.contains(&state.value) {
            return score(&state);
        }
        seen.insert(state.value);
        let o = update(state, tmp);
        state = o.0;
        tmp = o.1;
    }
}

fn solve_b(initial_state: State) -> u32 {
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
        .map(|level| level.value.count_ones())
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let initial_state = parse(lines);
    let a_solution = solve_a(initial_state.clone());
    let b_solution = solve_b(initial_state);
    (a_solution.to_string(), b_solution.to_string())
}
