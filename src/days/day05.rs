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
        5 => {
            let ia = prog[eip + 1];
            let ib = prog[eip + 2];

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

            if a != 0 {
                (b as usize, prog)
            } else {
                (eip + 3, prog)
            }
        }
        6 => {
            let ia = prog[eip + 1];
            let ib = prog[eip + 2];

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

            if a == 0 {
                (b as usize, prog)
            } else {
                (eip + 3, prog)
            }
        }
        7 => {
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

            prog[ic] = if a < b { 1 } else { 0 };
            (eip + 4, prog)
        }
        8 => {
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

            prog[ic] = if a == b { 1 } else { 0 };
            (eip + 4, prog)
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

fn solve_a(program: Vec<i32>) -> Option<i32> {
    let (_, output) = run(program, &mut vec![1].into_iter());
    if output
        .iter()
        .enumerate()
        .all(|(i, o)| *o == 0 || i == output.len() - 1)
    {
        Some(output[output.len() - 1])
    } else {
        None
    }
}

fn solve_b(program: Vec<i32>) -> i32 {
    let (_, output) = run(program, &mut vec![5].into_iter());
    output[output.len() - 1]
}

pub fn solve(lines: &[String]) -> Solution {
    let program: Vec<i32> = lines[0].split(',').map(|s| s.parse().unwrap()).collect();
    (
        solve_a(program.clone())
            .map(|i| i.to_string())
            .unwrap_or("Failure!".to_string()),
        solve_b(program).to_string(),
    )
}
