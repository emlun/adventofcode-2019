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
    let d = (0..digits.len())
        .map(|i| sum_term(i, digits, n + 1))
        .sum::<i32>()
        .abs()
        % 10;
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

    let msg_offset: usize = digits
        .iter()
        .take(7)
        .fold(0, |result, d| result * 10 + (*d as usize));

    let b_solution = if msg_offset >= digits.len() * 10000 / 2 {
        let digits: Vec<i8> = digits
            .iter()
            .cycle()
            .skip(msg_offset)
            .take(digits.len() * 10000 - msg_offset)
            .copied()
            .collect();

        fn b_phase(mut digits: Vec<i8>) -> Vec<i8> {
            for i in (0..(digits.len() - 1)).rev() {
                digits[i] = (digits[i + 1] + digits[i]).abs() % 10;
            }
            digits
        }

        fn b_transform(digits: Vec<i8>, times: usize) -> Vec<i8> {
            if times == 0 {
                digits
            } else {
                b_transform(b_phase(digits), times - 1)
            }
        }

        b_transform(digits, 100)
            .into_iter()
            .take(8)
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .join("")
    } else {
        panic!("Don't know how to solve when message offset is not past half the sequence!");
    };

    (a_solution.to_string(), b_solution.to_string())
}
