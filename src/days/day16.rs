use crate::common::Solution;

fn solve_a(digits: Vec<i8>) -> String {
    const PATTERN_BASE: [i8; 4] = [0, 1, 0, -1];

    fn pattern_digit(pattern_num: usize, digit: usize) -> i8 {
        let index = ((digit + 1) / pattern_num) % 4;
        PATTERN_BASE[index]
    }

    fn sum_term(i: usize, digits: &[i8], pattern_num: usize) -> i32 {
        (digits[i] * pattern_digit(pattern_num, i)) as i32
    }

    fn phase_digit(digits: &[i8], n: usize) -> i8 {
        if n >= digits.len() / 2 {
            digits.iter().skip(n).fold(0, |s, a| (s + *a) % 10)
        } else if n >= digits.len() / 3 {
            digits
                .iter()
                .skip(n)
                .take(n + 1)
                .fold(0, |s, a| (s + a) % 10)
        } else if n >= digits.len() / 4 {
            let positives = digits
                .iter()
                .skip(n)
                .take(n + 1)
                .fold(0_i32, |s, a| (s + *a as i32) as i32);
            let negatives = digits
                .iter()
                .skip(3 * n + 2)
                .take(n + 1)
                .fold(positives, |s, a| (s - *a as i32) as i32);
            (negatives.abs() % 10) as i8
        } else {
            ((n..digits.len())
                .map(|i| sum_term(i, digits, n + 1))
                .sum::<i32>()
                .abs()
                % 10) as i8
        }
    }

    fn phase(digits: &[i8]) -> Vec<i8> {
        (0..digits.len()).map(|i| phase_digit(digits, i)).collect()
    }

    fn transform(digits: Vec<i8>, phases: usize) -> Vec<i8> {
        (0..phases).fold(digits, |digs, _| phase(&digs))
    }

    transform(digits, 100)
        .into_iter()
        .take(8)
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn solve_b(digits: Vec<i8>) -> String {
    fn phase(mut digits: Vec<i8>) -> Vec<i8> {
        for i in (0..(digits.len() - 1)).rev() {
            digits[i] = (digits[i + 1] + digits[i]) % 10;
        }
        digits
    }

    fn transform(digits: Vec<i8>, phases: usize) -> Vec<i8> {
        (0..phases).fold(digits, |digs, _| phase(digs))
    }

    let msg_offset: usize = digits
        .iter()
        .take(7)
        .fold(0, |result, d| result * 10 + (*d as usize));

    if msg_offset >= digits.len() * 10000 / 2 {
        let digits: Vec<i8> = digits
            .iter()
            .cycle()
            .skip(msg_offset)
            .take(digits.len() * 10000 - msg_offset)
            .copied()
            .collect();

        transform(digits, 100)
            .into_iter()
            .take(8)
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .join("")
    } else {
        panic!("Don't know how to solve when message offset is not past half the sequence!");
    }
}

pub fn solve(lines: &[String]) -> Solution {
    let digits: Vec<i8> = lines[0].chars().map(|c| (c as i8) - 48).collect();
    let a_solution = solve_a(digits.clone());
    let b_solution = solve_b(digits);
    (a_solution, b_solution)
}
