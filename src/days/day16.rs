use crate::common::Solution;

pub fn solve(lines: &[String]) -> Solution {
    let digits: Vec<u8> = lines[0].chars().map(|c| (c as u8) - 48).collect();

    println!("{:?}", digits);

    fn phase_digit(digits: &Vec<u8>, n: usize) -> u8 {
        let pattern = |n: usize| {
            vec![0, 1, 0, -1]
                .into_iter()
                .cycle()
                .flat_map(move |i| vec![i].into_iter().cycle().take(n + 1))
                .skip(1)
        };

        // println!(
        //     "\n{} {:?} {:?}",
        //     n,
        //     digits,
        //     pattern(n).take(digits.len()).collect::<Vec<i8>>()
        // );
        let d = digits
            .iter()
            .zip(pattern(n))
            .map(|(a, b)| {
                let c = *a as i32 * b;
                // dbg!(a, b, c);
                c
            })
            .sum::<i32>()
            .abs()
            % 10;
        // dbg!(d)
        d as u8
    }

    fn phase(digits: &Vec<u8>) -> Vec<u8> {
        (0..digits.len()).map(|i| phase_digit(digits, i)).collect()
    };

    println!("{:?}", phase(&digits));
    println!("{:?}", phase(&phase(&digits)));
    println!("{:?}", phase(&phase(&phase(&digits))));

    fn transform(digits: &Vec<u8>, phases: usize) -> Vec<u8> {
        (0..phases).fold(digits.clone(), |digs, _| phase(&digs))
    }

    let a_solution = transform(&digits, 100)
        .into_iter()
        .take(8)
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join("");
    let b_solution = "bar";
    (a_solution.to_string(), b_solution.to_string())
}
