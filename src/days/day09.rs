use crate::common::Solution;
use crate::intcode;

fn run(mut program: Vec<i64>, input: i64) -> Vec<i64> {
    let mut output: Vec<i64> = Vec::new();
    let mut eip = 0;
    let mut input_iter = vec![input].into_iter();
    let mut next_input = None;
    let mut relbase = 0;
    while program[eip] != 99 {
        if next_input.is_none() {
            next_input = input_iter.next();
        }
        let mut outpt = None;
        let out = intcode::step(eip, program, &mut relbase, &mut outpt, &mut next_input);
        if let Some(out) = outpt {
            output.push(out);
        }
        eip = out.0;
        program = out.1;
    }
    output
}

fn solve_a(program: Vec<i64>) -> i64 {
    let output = run(program, 1);
    assert_eq!(output.len(), 1, "{:?}", output);
    output[0]
}

fn solve_b(program: Vec<i64>) -> i64 {
    let output = run(program, 2);
    assert_eq!(output.len(), 1, "{:?}", output);
    output[0]
}

pub fn solve(lines: &[String]) -> Solution {
    let program = intcode::parse(lines);
    (
        solve_a(program.clone()).to_string(),
        solve_b(program).to_string(),
    )
}
