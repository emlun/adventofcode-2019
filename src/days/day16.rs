use crate::common::Solution;

fn solve_a(digits: Vec<i8>) -> String {
    const PATTERN_BASE: [i8; 4] = [0, 1, 0, -1];

    let pattern: Vec<Vec<i8>> = (0..digits.len() / 4)
        .map(|pattern_i| {
            let pattern_num = pattern_i + 1;
            (1..=digits.len())
                .map(|digit| PATTERN_BASE[(digit / pattern_num) % 4])
                .collect()
        })
        .collect();

    let phase_digit = |digits: &[i8], n: usize| -> i8 {
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
                .map(|i| (digits[i] * pattern[n][i]) as i32)
                .sum::<i32>()
                .abs()
                % 10) as i8
        }
    };

    let transform = |digits: Vec<i8>, phases: usize| -> Vec<i8> {
        (0..phases).fold(digits, |digs, _| {
            (0..digs.len()).map(|i| phase_digit(&digs, i)).collect()
        })
    };

    transform(digits, 100)
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

fn solve_b(digits: Vec<i8>) -> String {
    fn transform(digits: Vec<i8>, msg_offset: usize, phases: usize) -> Vec<i8> {
        // As it turns out, the relevant diagonal of Pascal's triangle is
        // periodic with a period of 16000 elements
        let pascal_period = 16000;
        let mut pascal: Vec<Vec<i8>> = Vec::with_capacity(phases);
        pascal.push(vec![]);
        pascal.push((0..=9).cycle().skip(1).take(pascal_period).collect());
        for phase in 2..phases {
            let mut row = Vec::with_capacity(pascal_period);
            row.push(1);
            for index in 1..pascal_period {
                row.push((row[index - 1] + pascal[phase - 1][index]) % 10);
            }
            pascal.push(row);
        }

        let l = digits.len();
        let digits_offset: Vec<i8> = digits
            .into_iter()
            .cycle()
            .skip(msg_offset % l)
            .take(l)
            .collect();

        let joint_cycle = lcm(pascal_period, l);
        let tot_len = l * 10000 - msg_offset;
        let num_cycles = tot_len / joint_cycle;

        (0..8)
            .map(|i| {
                let sum_first_cycle = digits_offset
                    .iter()
                    .cycle()
                    .skip(i)
                    .take(joint_cycle)
                    .enumerate()
                    .fold(0, |sum, (index, digit)| {
                        (sum + (pascal[phases - 1][index % pascal_period] * *digit)) % 10
                    });

                let sum_last_cycle = digits_offset
                    .iter()
                    .cycle()
                    .take(tot_len)
                    .skip(i + num_cycles * joint_cycle)
                    .enumerate()
                    .fold(0, |sum, (index, digit)| {
                        (sum + (pascal[phases - 1][index % pascal_period] * *digit)) % 10
                    });

                (sum_first_cycle * num_cycles as i8 + sum_last_cycle) % 10
            })
            .collect()
    }

    let msg_offset: usize = digits
        .iter()
        .take(7)
        .fold(0, |result, d| result * 10 + (*d as usize));

    if msg_offset >= digits.len() * 10000 / 2 {
        transform(digits, msg_offset, 100)
            .into_iter()
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

#[cfg(test)]
mod tests {

    fn check_a(input: &str, expected_output: &str) {
        let digits: Vec<i8> = input.chars().map(|c| (c as i8) - 48).collect();
        let sol = super::solve_a(digits);
        assert_eq!(sol, expected_output);
    }

    fn check_b(input: &str, expected_output: &str) {
        let digits: Vec<i8> = input.chars().map(|c| (c as i8) - 48).collect();
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
