use crate::common::Solution;

fn step(iep: usize, mut prog: Vec<usize>) -> (usize, Vec<usize>) {
    let op = prog[iep];
    let a = prog[iep + 1];
    let b = prog[iep + 2];
    let o = prog[iep + 3];
    match op {
        1 => prog[o] = prog[a] + prog[b],
        2 => prog[o] = prog[a] * prog[b],
        _ => unreachable!(),
    }
    (iep + 4, prog)
}

fn run(mut program: Vec<usize>) -> usize {
    let mut iep = 0;
    // println!("{} : {:?}", iep, program);

    while program[iep] != 99 {
        let out = step(iep, program);
        iep = out.0;
        program = out.1;
        // println!("{} : {:?}", iep, program);
    }

    program[0]
}

fn solve_a(mut program: Vec<usize>) -> usize {
    let mut iep = 0;
    println!("{} : {:?}", iep, program);

    program[1] = 12;
    program[2] = 2;

    run(program)
}

fn solve_b(program: Vec<usize>) -> usize {
    for noun in 0..program.len() {
        println!("noun: {}", noun);
        for verb in 0..program.len() {
            let mut prog = program.clone();
            prog[1] = noun;
            prog[2] = verb;
            let output = run(prog);
            if output == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    panic!("No solution found!");
}

pub fn solve(lines: &[String]) -> Solution {
    let program: Vec<usize> = lines[0].split(',').map(|s| s.parse().unwrap()).collect();

    let a_solution = solve_a(program.clone());
    let b_solution = solve_b(program.clone());

    (a_solution.to_string(), b_solution.to_string())
}
