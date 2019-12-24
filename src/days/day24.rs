use crate::common::Solution;
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

fn solve_a(lines: &[String]) -> u128 {
    let initial_state = parse(lines);
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

// fn solve_b(lines: &[String]) -> u128 {
//     let poly = Deck::polynomial_for(119315717514047, lines);
//     poly.self_composed_deg1(101741582076661).apply(2020)
// }

pub fn solve(lines: &[String]) -> Solution {
    let a_solution = solve_a(lines);
    // let b_solution = solve_b(lines);
    let b_solution = "bar";
    (a_solution.to_string(), b_solution.to_string())
}
