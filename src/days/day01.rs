use crate::common::Solution;

pub fn solve(lines: &Vec<String>) -> Solution {
    let a_solution: i64 = lines
        .iter()
        .map(|line| line.parse::<i64>().unwrap() / 3 - 2)
        .sum();
    (a_solution.to_string(), "".to_string())
}
