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

pub fn solve(lines: &[String]) -> Solution {
    let mut program: Vec<usize> = lines[0].split(',').map(|s| s.parse().unwrap()).collect();
    let mut iep = 0;

    println!("{} : {:?}", iep, program);

    program[1] = 12;
    program[2] = 2;

    while program[iep] != 99 {
        let out = step(iep, program);
        iep = out.0;
        program = out.1;
        println!("{} : {:?}", iep, program);
    }

    (program[0].to_string(), "bar".to_string())
}
