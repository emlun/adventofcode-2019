use crate::common::Solution;
use crate::intcode;

fn run(mut program: Vec<i64>, input: i64) -> Vec<i64> {
    let mut outputs: Vec<i64> = Vec::new();
    let mut input: Option<i64> = Some(input);
    let mut output: Option<i64> = None;
    let mut eip = 0;
    while program[eip] != 99 {
        let out = intcode::step(eip, program, &mut 0, &mut output, &mut input);
        eip = out.0;
        program = out.1;
        if let Some(o) = output.take() {
            outputs.push(o);
        }
    }
    outputs
}

fn solve_a(program: Vec<i64>) -> Option<i64> {
    let output = run(program, 1);
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

fn solve_b(program: Vec<i64>) -> i64 {
    *run(program, 5).last().unwrap()
}

pub fn solve(lines: &[String]) -> Solution {
    let program = intcode::parse(lines);
    (
        solve_a(program.clone())
            .map(|i| i.to_string())
            .unwrap_or_else(|| "Failure!".to_string()),
        solve_b(program).to_string(),
    )
}
