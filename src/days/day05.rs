use crate::common::Solution;

fn step(
    eip: usize,
    mut prog: Vec<i32>,
    output: &mut Vec<i32>,
    input: &mut dyn Iterator<Item = i32>,
) -> (usize, Vec<i32>) {
    let instruction = prog[eip];
    let opcode = instruction % 100;
    let parmodes = instruction / 100;
    match opcode {
        1 => {
            let ia = prog[eip + 1];
            let ib = prog[eip + 2];
            let ic = prog[eip + 3] as usize;

            let a = if parmodes % 10 == 0 {
                prog[ia as usize]
            } else {
                ia
            };
            let b = if (parmodes / 10) % 10 == 0 {
                prog[ib as usize]
            } else {
                ib
            };

            prog[ic] = a + b;
            (eip + 4, prog)
        }
        2 => {
            let ia = prog[eip + 1];
            let ib = prog[eip + 2];
            let ic = prog[eip + 3] as usize;

            let a = if parmodes % 10 == 0 {
                prog[ia as usize]
            } else {
                ia
            };
            let b = if (parmodes / 10) % 10 == 0 {
                prog[ib as usize]
            } else {
                ib
            };

            prog[ic] = a * b;
            (eip + 4, prog)
        }
        3 => {
            let ia = prog[eip + 1] as usize;
            prog[ia] = input.next().unwrap();
            (eip + 2, prog)
        }
        4 => {
            let ia = prog[eip + 1];
            let a = if parmodes % 10 == 0 {
                prog[ia as usize]
            } else {
                ia
            };
            output.push(a);
            (eip + 2, prog)
        }
        _ => unreachable!(),
    }
}

fn run(mut program: Vec<i32>, input: &mut dyn Iterator<Item = i32>) -> (Vec<i32>, Vec<i32>) {
    let mut output: Vec<i32> = Vec::new();
    let mut eip = 0;
    while program[eip] != 99 {
        let out = step(eip, program, &mut output, input);
        eip = out.0;
        program = out.1;
    }
    (program, output)
}

fn solve_a(mut program: Vec<i32>, mut input: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let (prog2, output) = run(program, &mut input.into_iter());
    println!("{:?}", prog2);
    println!("{:?}", output);
    if output
        .iter()
        .enumerate()
        .all(|(i, o)| *o == 0 || i == output.len() - 1)
    {
        println!("Success!");
    } else {
        println!("Failure!");
    }
    (prog2, output)
}

#[allow(clippy::unreadable_literal)]
const B_OUTPUT_TARGET: usize = 19690720;

pub fn solve(lines: &[String]) -> Solution {
    // println!("{:?}", solve_a(vec![1002, 4, 3, 4, 33], vec![42]));
    let program: Vec<i32> = lines[0].split(',').map(|s| s.parse().unwrap()).collect();
    (
        solve_a(program.clone(), vec![1]).0[0].to_string(),
        "foo".to_string(),
    )
}
