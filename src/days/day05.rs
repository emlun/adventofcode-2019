use crate::common::Solution;
use crate::intcode::IntcodeComputer;

fn run(computer: IntcodeComputer, input: i64) -> Vec<i64> {
    computer.run(Some(input))
}

fn solve_a(computer: IntcodeComputer) -> Option<i64> {
    let output = run(computer, 1);
    if output
        .iter()
        .enumerate()
        .all(|(i, o)| *o == 0 || i == output.len() - 1)
    {
        Some(*output.last().unwrap())
    } else {
        None
    }
}

fn solve_b(computer: IntcodeComputer) -> i64 {
    *run(computer, 5).last().unwrap()
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
