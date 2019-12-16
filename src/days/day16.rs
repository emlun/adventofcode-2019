use crate::common::Solution;

const PATTERN_BASE: [i8; 4] = [0, 1, 0, -1];

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

fn pattern_digit(pattern_num: usize, digit: usize) -> i8 {
    let index = ((digit + 1) / pattern_num) % 4;
    PATTERN_BASE[index]
}

fn sum_term(i: usize, digits: &Vec<i8>, pattern_num: usize) -> i32 {
    (digits[i % digits.len()] * pattern_digit(pattern_num, i)) as i32
}

fn phase_digit(digits: &Vec<i8>, n: usize, total_len: usize) -> i8 {
    // println!(
    //     "\n{} {:?} {:?}",
    //     n,
    //     digits,
    //     pattern(n).take(digits.len()).collect::<Vec<i8>>()
    // );

    let pattern_len = (n + 1) * 4;
    let cycle_len = lcm(pattern_len as i64, digits.len() as i64) as usize;
    let cycles = total_len / cycle_len;
    let last_cycle_len = total_len % cycle_len;

    let cycles_sum = if cycles == 0 {
        0
    } else {
        ((0..cycle_len)
            .map(|i| sum_term(i, digits, n + 1))
            .sum::<i32>()
            * (cycles as i32))
            .abs()
    };
    let trailing_sum = (0..last_cycle_len)
        .map(|i| sum_term(i, digits, n + 1))
        .sum::<i32>()
        .abs();

    ((cycles_sum + trailing_sum) % 10) as i8
}

fn phase(digits: &Vec<i8>, repeats: usize) -> Vec<i8> {
    let total_len = digits.len() * repeats;
    (0..digits.len())
        .map(|i| phase_digit(digits, i, total_len))
        .collect()
}

fn transform(digits: &Vec<i8>, phases: usize, repeats: usize) -> Vec<i8> {
    (0..phases).fold(digits.clone(), |digs, _| phase(&digs, repeats))
}

pub fn solve(lines: &[String]) -> Solution {
    let digits: Vec<i8> = lines[0].chars().map(|c| (c as i8) - 48).collect();

    let a_solution = transform(&digits, 100, 1)
        .into_iter()
        .take(8)
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join("");
    let b_solution = "";
    (a_solution.to_string(), b_solution.to_string())
}
