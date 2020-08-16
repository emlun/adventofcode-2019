use crate::common::Solution;
use crate::intcode::IntcodeComputer;

fn solve_a(computer: IntcodeComputer) -> Option<i64> {
    let output = computer.run(Some(1)).output;
    if output
        .iter()
        .enumerate()
        .all(|(i, o)| *o == 0 || i == output.len() - 1)
    {
        Some(*output.back().unwrap())
    } else {
        None
    }
}

fn solve_b(computer: IntcodeComputer) -> i64 {
    *computer.run(Some(5)).output.back().unwrap()
}

pub fn solve(lines: &[String]) -> Solution {
    let computer: IntcodeComputer = lines.into();
    (
        solve_a(computer.clone())
            .map(|i| i.to_string())
            .unwrap_or_else(|| "Failure!".to_string()),
        solve_b(computer).to_string(),
    )
}
