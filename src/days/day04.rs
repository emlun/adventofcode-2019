use crate::common::Solution;
use std::collections::HashMap;

fn satisfies_conditions(password: u32) -> bool {
    let s = password.to_string();
    let mut two_same: bool = false;
    for i in 1..s.len() {
        if s.chars().nth(i - 1) == s.chars().nth(i) {
            two_same = true;
            break;
        }
    }

    let mut increasing: bool = true;
    for i in 1..s.len() {
        let i1 = (s.chars().nth(i - 1).unwrap() as u8 - 0x30);
        let i2 = (s.chars().nth(i).unwrap() as u8 - 0x30);
        if i1 > i2 {
            increasing = false;
            break;
        }
    }

    return two_same && increasing;
}

fn satisfies_conditions_b(password: u32) -> bool {
    let s = password.to_string();
    let mut sames: HashMap<char, u8> = HashMap::new();
    for i in 1..s.len() {
        let c1 = s.chars().nth(i - 1).unwrap();
        if c1 == s.chars().nth(i).unwrap() {
            sames.insert(c1, sames.get(&c1).unwrap_or(&0) + 1);
        }
    }
    let two_same: bool = sames.values().any(|i| i == &1);

    let mut increasing: bool = true;
    for i in 1..s.len() {
        let i1 = (s.chars().nth(i - 1).unwrap() as u8 - 0x30);
        let i2 = (s.chars().nth(i).unwrap() as u8 - 0x30);
        if i1 > i2 {
            increasing = false;
            break;
        }
    }

    return two_same && increasing;
}

pub fn solve(lines: &[String]) -> Solution {
    let bounds = lines[0]
        .split('-')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();
    let low_bound: u32 = bounds[0];
    let high_bound: u32 = bounds[1];

    let mut count = 0;
    let mut count_b = 0;
    for password in low_bound..=high_bound {
        if satisfies_conditions(password) {
            count += 1;
        }

        if satisfies_conditions_b(password) {
            count_b += 1;
        }
    }

    let a_solution = count;
    let b_solution = count_b;
    (a_solution.to_string(), b_solution.to_string())
}
