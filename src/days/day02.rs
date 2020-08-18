use crate::common::Solution;
use crate::intcode::parse_program;
use crate::intcode::IntcodeComputer;

fn solve_a(mut computer: IntcodeComputer) -> i64 {
    computer.prog[1] = 12;
    computer.prog[2] = 2;
    computer.run(None).prog[0]
}

#[allow(clippy::unreadable_literal)]
const B_OUTPUT_TARGET: i64 = 19690720;

fn solve_b(program: Vec<i64>) -> i64 {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut prog = program.clone();
            prog[1] = noun;
            prog[2] = verb;
            if IntcodeComputer::new(prog).run(None).prog[0] == B_OUTPUT_TARGET {
                return 100 * noun + verb;
            }
        }
    }
    panic!("No solution found!");
}

pub fn solve(lines: &[String]) -> Solution {
    let program = parse_program(lines);
    (
        solve_a(IntcodeComputer::new(program.clone())).to_string(),
        solve_b(program).to_string(),
    )
}
