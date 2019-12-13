use crate::common::Solution;
use crate::intcode;
use std::convert::TryInto;

fn run(mut program: Vec<i64>) -> i64 {
    let mut eip = 0;
    while program[eip] != 99 {
        let out = intcode::step(eip, program, &mut 0, &mut None, &mut None);
        eip = out.0;
        program = out.1;
    }
    program[0]
}

fn solve_a(mut program: Vec<i64>) -> i64 {
    program[1] = 12;
    program[2] = 2;
    run(program)
}

#[allow(clippy::unreadable_literal)]
const B_OUTPUT_TARGET: i64 = 19690720;

fn solve_b(program: Vec<i64>) -> i64 {
    for noun in 0..program.len() {
        for verb in 0..program.len() {
            let mut prog = program.clone();
            let noun = noun.try_into().unwrap();
            let verb = verb.try_into().unwrap();
            prog[1] = noun;
            prog[2] = verb;
            if run(prog) == B_OUTPUT_TARGET {
                return 100 * noun + verb;
            }
        }
    }
    panic!("No solution found!");
}

pub fn solve(lines: &[String]) -> Solution {
    let program = intcode::parse(lines);
    (
        solve_a(program.clone()).to_string(),
        solve_b(program).to_string(),
    )
}
