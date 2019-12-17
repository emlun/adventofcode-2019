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
    let pattern_len = n * 4;
    let cycle_len = lcm(pattern_len as i64, digits.len() as i64) as usize;
    let cycles = total_len / cycle_len;
    let last_cycle_len = total_len % cycle_len;

    let cycles_sum = if cycles == 0 {
        0
    } else {
        ((0..cycle_len).map(|i| sum_term(i, digits, n)).sum::<i32>() * (cycles as i32))
    };
    let trailing_sum = (0..last_cycle_len)
        .map(|i| sum_term(i, digits, n))
        .sum::<i32>();

    ((cycles_sum + trailing_sum).abs() % 10) as i8
}

fn phase(digits: &Vec<i8>, repeats: usize) -> Vec<i8> {
    let total_len = digits.len() * repeats;
    let result = (1..=digits.len())
        .map(|i| phase_digit(digits, i, total_len))
        .collect();
    result
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
