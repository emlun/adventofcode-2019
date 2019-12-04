use crate::common::Solution;
use std::collections::HashMap;

fn satisfies_conditions_a(password: u32) -> bool {
    let s = password.to_string();

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

#[derive(Debug)]
struct PasswordNumber {
    digits: Vec<u8>,
}

impl PasswordNumber {
    fn next(&mut self) {
        let mut add_digit = true;
        for i in 0..self.digits.len() {
            if self.digits[i] == 9 {
                self.digits[i] = 0;
            } else {
                self.digits[i] += 1;
                add_digit = false;
                break;
            }
        }
        if add_digit {
            self.digits.push(1);
        }

        for i in (1..self.digits.len()).rev() {
            if self.digits[i] > self.digits[i - 1] {
                self.digits[i - 1] = self.digits[i];
            }
        }
    }
    fn as_u32(&self) -> u32 {
        let mut result: u32 = 0;
        let mut pow: u32 = 1;
        for i in 0..self.digits.len() {
            result += pow * (self.digits[i] as u32);
            pow *= 10;
        }
        result
    }
}

impl From<u32> for PasswordNumber {
    fn from(i: u32) -> PasswordNumber {
        let mut digits = Vec::new();
        let mut pow = 1;
        loop {
            if pow > i {
                break;
            }
            digits.push(((i % (pow * 10)) / pow) as u8);
            pow *= 10;
        }
        PasswordNumber { digits: digits }
    }
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
    let mut num = PasswordNumber::from(low_bound);
    loop {
        let numu = num.as_u32();
        if numu > high_bound {
            break;
        }
        if satisfies_conditions_a(numu) {
            count_a += 1;
            if satisfies_new_conditions_b(numu) {
                count_b += 1;
            }
        }
        num.next();
    }

    (count_a.to_string(), count_b.to_string())
}
