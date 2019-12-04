use crate::common::Solution;
use std::collections::HashMap;

fn satisfies_conditions_a(password: u32) -> bool {
    let s = password.to_string();

    for i in 1..s.len() {
        let i1 = s.chars().nth(i - 1).unwrap() as u8 - 0x30;
        let i2 = s.chars().nth(i).unwrap() as u8 - 0x30;
        if i1 > i2 {
            return false;
        }
    }

    for i in 1..s.len() {
        if s.chars().nth(i - 1) == s.chars().nth(i) {
            return true;
        }
    }
    return false;
}

fn satisfies_new_conditions_b(password: u32) -> bool {
    let s = password.to_string();
    let mut sames: HashMap<char, u8> = HashMap::new();
    for i in 1..s.len() {
        let c1 = s.chars().nth(i - 1).unwrap();
        if c1 == s.chars().nth(i).unwrap() {
            sames.insert(c1, sames.get(&c1).unwrap_or(&0) + 1);
        }
    }
    return sames.values().any(|i| i == &1);
}

pub fn solve(lines: &[String]) -> Solution {
    let bounds = lines[0]
        .split('-')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();
    let low_bound: u32 = bounds[0];
    let high_bound: u32 = bounds[1];

    let mut count_a = 0;
    let mut count_b = 0;
    for password in low_bound..=high_bound {
        if satisfies_conditions_a(password) {
            count_a += 1;
            if satisfies_new_conditions_b(password) {
                count_b += 1;
            }
        }
    }

    (count_a.to_string(), count_b.to_string())
}
