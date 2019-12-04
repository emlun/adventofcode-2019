use crate::common::Solution;

fn satisfies_conditions(password: u32) -> bool {
    let s = password.to_string();
    let mut two_same: bool = false;
    for i in 1..s.len() {
        if s.chars().nth(i - 1) == s.chars().nth(i) {
            two_same = true;
            break;
        }
    }

    let mut increasing: bool = true;
    for i in 1..s.len() {
        let i1 = (s.chars().nth(i - 1).unwrap() as u8 - 0x30);
        let i2 = (s.chars().nth(i).unwrap() as u8 - 0x30);
        if i1 > i2 {
            increasing = false;
            break;
        }
    }

    // println!("{} {} {}", password, two_same, increasing);

    return two_same && increasing;
}

pub fn solve(lines: &[String]) -> Solution {
    let bounds = lines[0]
        .split('-')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();
    let low_bound: u32 = bounds[0];
    let high_bound: u32 = bounds[1];

    let mut count = 0;
    for password in low_bound..=high_bound {
        if satisfies_conditions(password) {
            count += 1;
        }
    }

    println!("{}", satisfies_conditions(111111));
    println!("{}", satisfies_conditions(223450));
    println!("{}", satisfies_conditions(123789));

    let a_solution = count;
    let b_solution = "bar";
    (a_solution.to_string(), b_solution.to_string())
}
