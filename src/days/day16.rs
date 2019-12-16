use crate::common::Solution;

const PATTERN_BASE: [i8; 4] = [0, 1, 0, -1];

fn pattern_digit(pattern_num: usize, digit: usize) -> i8 {
    let index = ((digit + 1) / pattern_num) % 4;
    PATTERN_BASE[index]
}

fn sum_term(i: usize, digits: &Vec<i8>, pattern_num: usize) -> i32 {
    (digits[i] * pattern_digit(pattern_num, i)) as i32
}

fn phase_digit(digits: &Vec<i8>, n: usize) -> i8 {
    // println!(
    //     "\n{} {:?} {:?}",
    //     n,
    //     digits,
    //     pattern(n).take(digits.len()).collect::<Vec<i8>>()
    // );
    let d = (0..digits.len())
        .map(|i| sum_term(i, digits, n + 1))
        .sum::<i32>()
        .abs()
        % 10;
    // dbg!(d)
    d as i8
}

fn phase(digits: &Vec<i8>) -> Vec<i8> {
    (0..digits.len()).map(|i| phase_digit(digits, i)).collect()
}

fn transform(digits: &Vec<i8>, phases: usize) -> Vec<i8> {
    (0..phases).fold(digits.clone(), |digs, _| phase(&digs))
}

pub fn solve(lines: &[String]) -> Solution {
    let digits: Vec<i8> = lines[0].chars().map(|c| (c as i8) - 48).collect();

    let a_solution = transform(&digits, 100)
        .into_iter()
        .take(8)
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join("");
    let b_solution = "";
    (a_solution.to_string(), b_solution.to_string())
}
