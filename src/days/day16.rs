use crate::common::Solution;
use crate::util::pascal::PASCAL_DIAGONAL_100;

const NUM_PHASES: usize = 100;

fn solve_a(digits: Vec<i32>) -> String {
    let phase_digit = |digits: &[i32], n: usize| -> i32 {
        if n >= digits.len() / 2 {
            digits.iter().skip(n).sum::<i32>() % 10
        } else if n >= digits.len() / 3 {
            digits.iter().skip(n).take(n + 1).sum::<i32>() % 10
        } else if n >= digits.len() / 4 {
            let positives: i32 = digits.iter().skip(n).take(n + 1).sum();
            let negatives: i32 = digits.iter().skip(3 * n + 2).take(n + 1).sum();
            (positives - negatives).abs() % 10
        } else {
            let positives: i32 = (n..digits.len())
                .step_by((n + 1) * 4)
                .flat_map(|i| digits.iter().skip(i).take(n + 1))
                .sum();

            let negatives: i32 = ((n + ((n + 1) * 2))..digits.len())
                .step_by((n + 1) * 4)
                .flat_map(|i| digits.iter().skip(i).take(n + 1))
                .sum();

            (positives - negatives).abs() % 10
        }
    };

    let transform = |digits: Vec<i32>| -> Vec<i32> {
        (0..NUM_PHASES).fold(digits, |digs, _| {
            (0..digs.len()).map(|i| phase_digit(&digs, i)).collect()
        })
    };

    transform(digits)
        .into_iter()
        .take(8)
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    let gcdab = gcd(a, b);
    (a / gcdab) * b
}

fn solve_b(digits: Vec<i32>) -> String {
    fn transform(digits: Vec<i32>, msg_offset: usize) -> Vec<String> {
        let l = digits.len();
        let digits_offset: Vec<i32> = digits
            .into_iter()
            .cycle()
            .skip(msg_offset % l)
            .take(l)
            .collect();

        let joint_cycle = lcm(PASCAL_DIAGONAL_100.len(), l);
        let tot_len = l * 10000 - msg_offset;
        let num_cycles = tot_len / joint_cycle;

        (0..8)
            .map(|i| {
                let sum_first_cycle: i32 = digits_offset
                    .iter()
                    .cycle()
                    .skip(i)
                    .take(joint_cycle)
                    .enumerate()
                    .map(|(index, digit)| {
                        PASCAL_DIAGONAL_100[index % PASCAL_DIAGONAL_100.len()] * *digit
                    })
                    .sum();

                let sum_last_cycle: i32 = digits_offset
                    .iter()
                    .cycle()
                    .take(tot_len)
                    .skip(i + num_cycles * joint_cycle)
                    .enumerate()
                    .map(|(index, digit)| {
                        PASCAL_DIAGONAL_100[index % PASCAL_DIAGONAL_100.len()] * *digit
                    })
                    .sum();

                (sum_first_cycle * num_cycles as i32 + sum_last_cycle) % 10
            })
            .map(|d| d.to_string())
            .collect()
    }

    let msg_offset: usize = digits
        .iter()
        .take(7)
        .fold(0, |result, d| result * 10 + (*d as usize));

    if msg_offset >= digits.len() * 10000 / 2 {
        transform(digits, msg_offset).join("")
    } else {
        panic!("Don't know how to solve when message offset is not past half the sequence!");
    }
}

pub fn solve(lines: &[String]) -> Solution {
    let digits: Vec<i32> = lines[0].chars().map(|c| (c as i32) - 48).collect();
    let a_solution = solve_a(digits.clone());
    let b_solution = solve_b(digits);
    (a_solution, b_solution)
}

#[cfg(test)]
mod tests {

    fn check_a(input: &str, expected_output: &str) {
        let digits: Vec<i32> = input.chars().map(|c| (c as i32) - 48).collect();
        let sol = super::solve_a(digits);
        assert_eq!(sol, expected_output);
    }

    fn check_b(input: &str, expected_output: &str) {
        let digits: Vec<i32> = input.chars().map(|c| (c as i32) - 48).collect();
        let sol = super::solve_b(digits);
        assert_eq!(sol, expected_output);
    }

    #[test]
    fn example_a1() {
        check_a("80871224585914546619083218645595", "24176176");
    }

    #[test]
    fn example_a2() {
        check_a("19617804207202209144916044189917", "73745418");
    }

    #[test]
    fn example_a3() {
        check_a("69317163492948606335995924319873", "52432133");
    }

    #[test]
    fn example_b1() {
        check_b("03036732577212944063491565474664", "84462026");
    }

    #[test]
    fn example_b2() {
        check_b("02935109699940807407585447034323", "78725270");
    }

    #[test]
    fn example_b3() {
        check_b("03081770884921959731165446850517", "53553731");
    }
}
