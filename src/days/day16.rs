use crate::common::Solution;

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

pub fn solve(lines: &[String]) -> Solution {
    let digits: Vec<u8> = lines[0].chars().map(|c| (c as u8) - 48).collect();

    fn phase_digit(digits: &Vec<u8>, n: usize, repeats: usize) -> u8 {
        let pattern = |n: usize| {
            vec![0, 1, 0, -1]
                .into_iter()
                .cycle()
                .flat_map(move |i| vec![i].into_iter().cycle().take(n + 1))
                .skip(1)
        };

        let pattern_length = (n + 1) * 4;
        let cycle_length = lcm(digits.len() as i64, pattern_length as i64) as usize;
        let sum_multiplier = repeats / (cycle_length / digits.len());
        let trailing_digits = (digits.len() * repeats) % cycle_length;

        // dbg!(
        //     repeats,
        //     digits.len(),
        //     pattern_length,
        //     cycle_length,
        //     sum_multiplier,
        //     trailing_digits
        // );

        let repeated_digits_sum = digits
            .iter()
            .cycle()
            .take(cycle_length * std::cmp::min(sum_multiplier, 1))
            .zip(pattern(n))
            .map(|(a, b)| {
                let c = *a as i32 * b;
                // dbg!(a, b, c);
                c
            })
            .sum::<i32>()
            * (sum_multiplier as i32);
        let trailing_digits_sum = digits
            .iter()
            .cycle()
            .take(trailing_digits)
            .zip(pattern(n))
            .map(|(a, b)| {
                let c = *a as i32 * b;
                // dbg!(a, b, c);
                c
            })
            .sum::<i32>();

        let d = (repeated_digits_sum + trailing_digits_sum).abs() % 10;
        // dbg!(repeated_digits_sum, trailing_digits_sum, d);
        d as u8
    }

    fn phase(digits: &Vec<u8>, repeats: usize) -> Vec<u8> {
        (0..digits.len())
            .map(|i| phase_digit(digits, i, repeats))
            .collect()
    };

    fn transform(digits: &Vec<u8>, phases: usize, repeats: usize) -> Vec<u8> {
        (0..phases).fold(digits.clone(), |digs, _| phase(&digs, repeats))
    }

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

    let b_solution = transform(
        &digits
            .iter()
            // .cycle()
            // .take(digits.len() * 10000)
            .copied()
            .collect(),
        100,
        10000,
    )
    .into_iter()
    .cycle()
    .skip(msg_offset)
    .take(8)
    .map(|d| d.to_string())
    .collect::<Vec<String>>()
    .join("");

    (a_solution.to_string(), b_solution.to_string())
}
