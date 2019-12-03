use crate::common::Solution;

fn step(eip: usize, mut prog: Vec<usize>) -> (usize, Vec<usize>) {
    let a = prog[eip + 1];
    let b = prog[eip + 2];
    let o = prog[eip + 3];
    match prog[eip] {
        1 => prog[o] = prog[a] + prog[b],
        2 => prog[o] = prog[a] * prog[b],
        _ => unreachable!(),
    }
    (eip + 4, prog)
}

fn run(mut program: Vec<usize>) -> usize {
    let mut eip = 0;
    while program[eip] != 99 {
        let out = step(eip, program);
        eip = out.0;
        program = out.1;
    }
    program[0]
}

fn solve_a(mut program: Vec<usize>) -> usize {
    program[1] = 12;
    program[2] = 2;
    run(program)
}

#[allow(clippy::unreadable_literal)]
const B_OUTPUT_TARGET: usize = 19690720;

fn solve_b(program: Vec<usize>) -> usize {
    for noun in 0..program.len() {
        for verb in 0..program.len() {
            let mut prog = program.clone();
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
    let program: Vec<usize> = lines[0].split(',').map(|s| s.parse().unwrap()).collect();
    (
        solve_a(program.clone()).to_string(),
        solve_b(program).to_string(),
    )
}
