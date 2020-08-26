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
        .dim()
}

fn format_state(state: &State) -> String {
    (0..9)
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
    const MAXI: usize = 5;

    let mut matrix: u64 = 0;
    let mut mask = 1 << 7;
    for y in 1..=MAXI {
        for x in 1..=MAXI {
            mask <<= 1;

            let neighbors = state.count_neighbors(x, y);
            if neighbors == 1 || (neighbors == 2 && !state.get(x, y)) {
                matrix |= mask;
            }
        }
        mask <<= 2;
    }
    next_state.value = matrix;

    (next_state, state)
}

type State = BoolMatrix;

#[derive(Clone)]
struct BoolMatrix {
    value: u64,
}

impl BoolMatrix {
    const DIM: usize = 7;

    const VALUE_MASK: u64 = ((2 | 4 | 8 | 16 | 32) << Self::DIM)
        | ((2 | 4 | 8 | 16 | 32) << (2 * Self::DIM))
        | ((2 | 4 | 8 | 16 | 32) << (3 * Self::DIM))
        | ((2 | 4 | 8 | 16 | 32) << (4 * Self::DIM))
        | ((2 | 4 | 8 | 16 | 32) << (5 * Self::DIM));
    const NEIGHBOR_MASK: u64 = 2 | (1 << Self::DIM) | (4 << Self::DIM) | (2 << (2 * Self::DIM));

    const LEFT_MIDDLE_MASK: u64 =
        (2 << (2 * Self::DIM)) | (2 << (3 * Self::DIM)) | (2 << (4 * Self::DIM));
    const RIGHT_MIDDLE_MASK: u64 =
        (32 << (2 * Self::DIM)) | (32 << (3 * Self::DIM)) | (32 << (4 * Self::DIM));

    const LEFT_MASK: u64 = (2 << Self::DIM) | Self::LEFT_MIDDLE_MASK | (2 << (5 * Self::DIM));
    const RIGHT_MASK: u64 = (32 << Self::DIM) | Self::RIGHT_MIDDLE_MASK | (32 << (5 * Self::DIM));
    const TOP_MASK: u64 = (2 | 4 | 8 | 16 | 32) << Self::DIM;
    const BOTTOM_MASK: u64 = (2 | 4 | 8 | 16 | 32) << (5 * Self::DIM);

    const PADDING_TOP_MASK: u64 = Self::TOP_MASK >> Self::DIM;
    const PADDING_RIGHT_MASK: u64 = Self::RIGHT_MASK << 1;
    const PADDING_BOTTOM_MASK: u64 = Self::BOTTOM_MASK << Self::DIM;
    const PADDING_LEFT_MASK: u64 = Self::LEFT_MASK >> 1;

    const INNER_PADDING_TOP_SHIFT: u64 = 6 * Self::DIM as u64;
    const INNER_PADDING_BOTTOM_SHIFT: u64 = 3 * Self::DIM as u64;
    const INNER_PADDING_RIGHT_SHIFT: u64 = 4 * Self::DIM as u64 + 1;
    const INNER_PADDING_LEFT_SHIFT: u64 = 4 * Self::DIM as u64 - 1;

    const NEIGHBOR_MASK_INNER_TOP: u64 = (Self::NEIGHBOR_MASK
        << Self::coords_to_index(3 - 1, 2 - 1))
        | (Self::TOP_MASK << Self::INNER_PADDING_TOP_SHIFT);
    const NEIGHBOR_MASK_INNER_RIGHT: u64 = (Self::NEIGHBOR_MASK
        << Self::coords_to_index(4 - 1, 3 - 1))
        | (32 << (7 * Self::DIM))
        | (32 << (8 * Self::DIM))
        | (Self::RIGHT_MIDDLE_MASK << Self::INNER_PADDING_RIGHT_SHIFT);
    const NEIGHBOR_MASK_INNER_BOTTOM: u64 = (Self::NEIGHBOR_MASK
        << Self::coords_to_index(3 - 1, 4 - 1))
        | (Self::BOTTOM_MASK << Self::INNER_PADDING_BOTTOM_SHIFT);
    const NEIGHBOR_MASK_INNER_LEFT: u64 = (Self::NEIGHBOR_MASK
        << Self::coords_to_index(2 - 1, 3 - 1))
        | (2 << (7 * Self::DIM))
        | (2 << (8 * Self::DIM))
        | (Self::LEFT_MIDDLE_MASK << Self::INNER_PADDING_LEFT_SHIFT);

    fn new() -> Self {
        Self { value: 0 }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        (self.value >> Self::coords_to_index(x, y)) & 1 != 0
    }

    fn count_neighbors(&self, x: usize, y: usize) -> u32 {
        debug_assert!(x > 0);
        debug_assert!(y > 0);
        (self.value & self.get_neighbor_mask(x, y)).count_ones()
    }

    fn get_neighbor_mask(&self, x: usize, y: usize) -> u64 {
        match (x, y) {
            (3, 2) => Self::NEIGHBOR_MASK_INNER_TOP,
            (4, 3) => Self::NEIGHBOR_MASK_INNER_RIGHT,
            (3, 4) => Self::NEIGHBOR_MASK_INNER_BOTTOM,
            (2, 3) => Self::NEIGHBOR_MASK_INNER_LEFT,
            _ => Self::NEIGHBOR_MASK << Self::coords_to_index(x - 1, y - 1),
        }
    }

    fn set_padding(&mut self, outer_neighbor_padding: u64, inner_neighbor_padding: u64) {
        self.value =
            (self.value & Self::VALUE_MASK) | outer_neighbor_padding | inner_neighbor_padding;
    }

    fn get_padding_for_inner_neighbor(&self) -> u64 {
        // if expressions seem to be a bit faster than multiplication
        (if self.get(3, 2) {
            Self::PADDING_TOP_MASK
        } else {
            0
        }) | (if self.get(4, 3) {
            Self::PADDING_RIGHT_MASK
        } else {
            0
        }) | (if self.get(3, 4) {
            Self::PADDING_BOTTOM_MASK
        } else {
            0
        }) | (if self.get(2, 3) {
            Self::PADDING_LEFT_MASK
        } else {
            0
        })
    }

    fn get_padding_for_outer_neighbor(&self) -> u64 {
        ((self.value & Self::TOP_MASK) << Self::INNER_PADDING_TOP_SHIFT)
            | ((self.value & Self::BOTTOM_MASK) << Self::INNER_PADDING_BOTTOM_SHIFT)
            | ((self.value & Self::LEFT_MIDDLE_MASK) << Self::INNER_PADDING_LEFT_SHIFT)
            | ((self.value & Self::RIGHT_MIDDLE_MASK) << Self::INNER_PADDING_RIGHT_SHIFT)
    }

    const fn coords_to_index(x: usize, y: usize) -> usize {
        y * Self::DIM + x
    }
}

struct BoolMatrixBuilder {
    value: u64,
}

impl BoolMatrixBuilder {
    fn dim(self) -> BoolMatrix {
        let mut m = BoolMatrix::new();
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
            empty_level: BoolMatrix::new(),
            min_level: 0,
            max_level: 0,
        };

        let padding_inwards = slf.get(0).get_padding_for_inner_neighbor();
        let padding_outwards = slf.get(0).get_padding_for_outer_neighbor();
        slf.set_padding(1, padding_inwards, 0);
        slf.set_padding(-1, 0, padding_outwards);

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

    fn set_padding(
        &mut self,
        level: i32,
        outer_neighbor_padding: u64,
        inner_neighbor_padding: u64,
    ) {
        let level_index = Self::level_to_index(level);
        if level <= self.max_level && level >= self.min_level {
            self.levels[level_index].set_padding(outer_neighbor_padding, inner_neighbor_padding);
        } else if (outer_neighbor_padding | inner_neighbor_padding) != 0 {
            self.get_mut(level, level_index)
                .set_padding(outer_neighbor_padding, inner_neighbor_padding);
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

        let mut matrix: u64 = 0;
        let mut mask = 1 << 7;
        for y in 1..=MAXI {
            for x in 1..=MAXI {
                mask <<= 1;

                if x == 3 && y == 3 {
                    continue;
                }

                let neighbors = lvl.count_neighbors(x, y);
                if neighbors == 1 || (neighbors == 2 && !lvl.get(x, y)) {
                    matrix |= mask;
                }
            }
            mask <<= 2;
        }
        next_state.set_value(level, matrix);
    }

    for level in (state.min_level - 1)..=(state.max_level + 1) {
        let lvldn_now = next_state.get(level - 1);
        let outer_neighbor_padding = lvldn_now.get_padding_for_inner_neighbor();
        let lvlup_now = next_state.get(level + 1);
        let inner_neighbor_padding = lvlup_now.get_padding_for_outer_neighbor();
        next_state.set_padding(level, outer_neighbor_padding, inner_neighbor_padding);
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
        .map(|level| (level.value & BoolMatrix::VALUE_MASK).count_ones())
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let initial_state = parse(lines);
    let a_solution = solve_a(initial_state.clone());
    let b_solution = solve_b(initial_state);
    (a_solution.to_string(), b_solution.to_string())
}
