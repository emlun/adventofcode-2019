use crate::common::Solution;
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

fn format_state(state: &State) -> String {
    (0..7)
        .map(|y| {
            (0..7)
                .map(|x| if state.get(x, y) { "#" } else { "." }.to_string())
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("\n")
}

#[allow(dead_code)]
fn print_levels(state: &LevelsState) {
    for level in state.min_level..=state.max_level {
        let s = format_state(state.get(level));
        println!("\nLevel {}\n{}", level, s);
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
    value_mask: u64,
    neighbor_mask: u64,
    padding_top_mask: u64,
    padding_right_mask: u64,
    padding_bottom_mask: u64,
    padding_left_mask: u64,
    left_mask: u64,
    right_mask: u64,
    top_mask: u64,
    bottom_mask: u64,
}

impl BoolMatrix {
    fn new(dim: usize) -> Self {
        let value_mask = ((2 | 4 | 8 | 16 | 32) << dim)
            | ((2 | 4 | 8 | 16 | 32) << (2 * dim))
            | ((2 | 4 | 8 | 16 | 32) << (3 * dim))
            | ((2 | 4 | 8 | 16 | 32) << (4 * dim))
            | ((2 | 4 | 8 | 16 | 32) << (5 * dim));
        let neighbor_mask = 2 | (1 << dim) | (4 << dim) | (2 << (2 * dim));

        let padding_top_mask = 2 | 4 | 8 | 16 | 32;
        let padding_right_mask = (64 << dim)
            | (64 << (2 * dim))
            | (64 << (3 * dim))
            | (64 << (4 * dim))
            | (64 << (5 * dim));
        let padding_bottom_mask = (2 | 4 | 8 | 16 | 32) << (6 * dim);
        let padding_left_mask =
            (1 << dim) | (1 << (2 * dim)) | (1 << (3 * dim)) | (1 << (4 * dim)) | (1 << (5 * dim));

        let left_mask =
            (2 << dim) | (2 << (2 * dim)) | (2 << (3 * dim)) | (2 << (4 * dim)) | (2 << (5 * dim));
        let right_mask = (32 << dim)
            | (32 << (2 * dim))
            | (32 << (3 * dim))
            | (32 << (4 * dim))
            | (32 << (5 * dim));
        let top_mask = (2 | 4 | 8 | 16 | 32) << dim;
        let bottom_mask = (2 | 4 | 8 | 16 | 32) << (5 * dim);

        Self {
            dim,
            value: 0,
            value_mask,
            neighbor_mask,
            padding_top_mask,
            padding_right_mask,
            padding_bottom_mask,
            padding_left_mask,
            left_mask,
            right_mask,
            top_mask,
            bottom_mask,
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        (self.value >> self.coords_to_index(x, y)) & 1 != 0
    }

    #[inline]
    fn count_left(&self) -> u32 {
        (self.value & self.left_mask).count_ones()
    }

    #[inline]
    fn count_right(&self) -> u32 {
        (self.value & self.right_mask).count_ones()
    }

    #[inline]
    fn count_top(&self) -> u32 {
        (self.value & self.top_mask).count_ones()
    }

    #[inline]
    fn count_bottom(&self) -> u32 {
        (self.value & self.bottom_mask).count_ones()
    }

    fn count_neighbors(&self, x: usize, y: usize) -> u32 {
        debug_assert!(x > 0);
        debug_assert!(y > 0);
        (self.value & self.get_neighbor_mask(x, y)).count_ones()
    }

    fn get_neighbor_mask(&self, x: usize, y: usize) -> u64 {
        self.neighbor_mask << self.coords_to_index(x - 1, y - 1)
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

    fn set_padding(&mut self, padding: u64) {
        self.value = (self.value & self.value_mask) | padding;
    }

    fn get_padding_for_inner_neighbor(&self) -> u64 {
        self.padding_top_mask * (self.get(3, 2) as u64)
            | self.padding_right_mask * (self.get(4, 3) as u64)
            | self.padding_bottom_mask * (self.get(3, 4) as u64)
            | self.padding_left_mask * (self.get(2, 3) as u64)
    }

    fn coords_to_index_for_dim(dim: usize, x: usize, y: usize) -> usize {
        debug_assert!(x < dim);
        debug_assert!(y < dim);
        y * dim + x
    }

    fn coords_to_index(&self, x: usize, y: usize) -> usize {
        Self::coords_to_index_for_dim(self.dim, x, y)
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
        let mut slf = LevelsState {
            levels: vec![state],
            empty_level: BoolMatrix::new(7),
            min_level: 0,
            max_level: 0,
        };

        let padding = slf.get(0).get_padding_for_inner_neighbor();
        slf.set_padding(1, padding);

        slf
    }

    fn get(&self, level: i32) -> &State {
        let index = Self::level_to_index(level);
        if index < self.levels.len() {
            &self.levels[index]
        } else {
            &self.empty_level
        }
    }

    fn get_mut(&mut self, level: i32, index: usize) -> &mut State {
        while self.levels.len() <= index {
            self.levels.push(self.empty_level.clone());
        }
        if level < self.min_level {
            self.min_level = level;
        } else if level > self.max_level {
            self.max_level = level;
        }
        &mut self.levels[index]
    }

    fn set_value(&mut self, level: i32, value: u64) {
        let level_index = Self::level_to_index(level);
        if level <= self.max_level && level >= self.min_level {
            self.levels[level_index].value = value;
        } else if value != 0 {
            self.get_mut(level, level_index).value = value;
        }
    }

    fn set_padding(&mut self, level: i32, padding: u64) {
        let level_index = Self::level_to_index(level);
        if level <= self.max_level && level >= self.min_level {
            self.levels[level_index].set_padding(padding);
        } else if padding != 0 {
            self.get_mut(level, level_index).set_padding(padding);
        }
    }

    fn level_to_index(level: i32) -> usize {
        level.abs() as usize * 2 - ((level < 0) as usize)
    }
}

fn update_b(state: LevelsState, mut next_state: LevelsState) -> (LevelsState, LevelsState) {
    const MAXI: usize = 5;
    for level in (state.min_level - 1)..=(state.max_level + 1) {
        let lvl = state.get(level);
        let lvlup = state.get(level + 1);

        let mut matrix: u64 = 0;
        let mut mask = 1 << 7;
        for y in 1..=MAXI {
            for x in 1..=MAXI {
                mask <<= 1;

                if x == 3 && y == 3 {
                    continue;
                }

                let basic_neighbors = lvl.count_neighbors(x, y);
                let level_neighbors = match (x, y) {
                    (2, 3) => lvlup.count_left() as usize,
                    (4, 3) => lvlup.count_right() as usize,
                    (3, 2) => lvlup.count_top() as usize,
                    (3, 4) => lvlup.count_bottom() as usize,

                    _ => 0,
                };
                let neighbors = basic_neighbors as usize + level_neighbors;

                if neighbors == 1 || (neighbors == 2 && !lvl.get(x, y)) {
                    matrix |= mask;
                }
            }
            mask <<= 2;
        }
        next_state.set_value(level, matrix);

        let lvldn_now = next_state.get(level - 1);
        let padding = lvldn_now.get_padding_for_inner_neighbor();
        next_state.set_padding(level, padding);
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
        .map(|level| (level.value & level.value_mask).count_ones())
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let initial_state = parse(lines);
    let a_solution = solve_a(initial_state.clone());
    let b_solution = solve_b(initial_state);
    (a_solution.to_string(), b_solution.to_string())
}
