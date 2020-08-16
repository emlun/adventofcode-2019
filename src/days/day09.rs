use crate::common::Solution;
use crate::intcode::IntcodeComputer;

fn solve_a(computer: IntcodeComputer) -> i64 {
    let output = computer.run(Some(1)).output;
    assert_eq!(output.len(), 1, "{:?}", output);
    output[0]
}

fn solve_b(computer: IntcodeComputer) -> i64 {
    let output = computer.run(Some(2)).output;
    assert_eq!(output.len(), 1, "{:?}", output);
    output[0]
}

pub fn solve(lines: &[String]) -> Solution {
    let computer: IntcodeComputer = lines.into();
    (
        solve_a(computer.clone()).to_string(),
        solve_b(computer).to_string(),
    )
}
