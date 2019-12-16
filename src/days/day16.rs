use crate::common::Solution;

const PATTERN_BASE: [i8; 4] = [0, 1, 0, -1];

fn pattern_digit(pattern_num: usize, digit: usize) -> i8 {
    let index = ((digit + 1) / pattern_num) % 4;
    PATTERN_BASE[index]
}

pub fn solve(lines: &[String]) -> Solution {
    let digits: Vec<i8> = lines[0].chars().map(|c| (c as i8) - 48).collect();

    println!("{:?}", digits);

    fn phase_digit(digits: &Vec<i8>, n: usize) -> i8 {
        let pattern = (0..).map(move |i| pattern_digit(n + 1, i));

        // println!(
        //     "\n{} {:?} {:?}",
        //     n,
        //     digits,
        //     pattern(n).take(digits.len()).collect::<Vec<i8>>()
        // );
        let d = digits
            .iter()
            .zip(pattern)
            .map(|(a, b)| {
                let c = *a * b;
                // dbg!(a, b, c);
                c as i32
            })
            .sum::<i32>()
            .abs()
            % 10;
        // dbg!(d)
        d as i8
    }

    fn phase(digits: &Vec<i8>) -> Vec<i8> {
        (0..digits.len()).map(|i| phase_digit(digits, i)).collect()
    };

    println!("{:?}", phase(&digits));
    println!("{:?}", phase(&phase(&digits)));
    println!("{:?}", phase(&phase(&phase(&digits))));

    fn transform(digits: &Vec<i8>, phases: usize) -> Vec<i8> {
        (0..phases).fold(digits.clone(), |digs, _| phase(&digs))
    }

    let a_solution = transform(&digits, 100)
        .into_iter()
        .take(8)
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join("");
    let b_solution = "";
    (a_solution.to_string(), b_solution.to_string())
}
