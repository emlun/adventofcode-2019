use crate::common::Solution;

fn step(
    eip: usize,
    mut prog: Vec<i32>,
    output: &mut Vec<i32>,
    input: &mut dyn Iterator<Item = i32>,
) -> (usize, Vec<i32>) {
    let instruction = prog[eip];
    let opcode = instruction % 100;

    let get_args = |num: usize| {
        let mut result = Vec::new();

        let mut parmode_pow = 100;
        for i in 1..=num {
            let iarg = prog[eip + i];
            let arg = if (instruction / parmode_pow) % 10 == 0 {
                prog[iarg as usize]
            } else {
                iarg
            };
            result.push(arg);
            parmode_pow *= 10;
        }

        result
    };

    let eip = match opcode {
        1 => {
            let args = get_args(2);
            let io = prog[eip + 3] as usize;
            prog[io] = args[0] + args[1];
            eip + 4
        }
        2 => {
            let args = get_args(2);
            let io = prog[eip + 3] as usize;
            prog[io] = args[0] * args[1];
            eip + 4
        }
        3 => {
            let io = prog[eip + 1] as usize;
            prog[io] = input.next().unwrap();
            eip + 2
        }
        4 => {
            let args = get_args(1);
            output.push(args[0]);
            eip + 2
        }
        5 => {
            let args = get_args(2);
            if args[0] != 0 {
                args[1] as usize
            } else {
                eip + 3
            }
        }
        6 => {
            let args = get_args(2);
            if args[0] == 0 {
                args[1] as usize
            } else {
                eip + 3
            }
        }
        7 => {
            let args = get_args(2);
            let io = prog[eip + 3] as usize;
            prog[io] = if args[0] < args[1] { 1 } else { 0 };
            eip + 4
        }
        8 => {
            let args = get_args(2);
            let io = prog[eip + 3] as usize;
            prog[io] = if args[0] == args[1] { 1 } else { 0 };
            eip + 4
        }
        _ => unreachable!(),
    };
    (eip, prog)
}

fn run(mut program: Vec<i32>, input: (u8, i32)) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    let mut eip = 0;
    let input_iter = &mut vec![i32::from(input.0), input.1].into_iter();
    while program[eip] != 99 {
        let out = step(eip, program, &mut output, input_iter);
        eip = out.0;
        program = out.1;
    }
    output
}

fn solve_a(program: Vec<i32>) -> Option<i32> {
    let mut max_output = None;

    for i0 in 0..=4 {
        for i1 in 0..=4 {
            if i0 != i1 {
                for i2 in 0..=4 {
                    if i2 != i0 && i2 != i1 {
                        for i3 in 0..=4 {
                            if i3 != i0 && i3 != i1 && i3 != i2 {
                                for i4 in 0..=4 {
                                    if i4 != i0 && i4 != i1 && i4 != i2 && i4 != i3 {
                                        let output0 = run(program.clone(), (i0, 0));
                                        let output1 = run(program.clone(), (i1, output0[0]));
                                        let output2 = run(program.clone(), (i2, output1[0]));
                                        let output3 = run(program.clone(), (i3, output2[0]));
                                        let output4 = run(program.clone(), (i4, output3[0]));
                                        if output4[0] > max_output.unwrap_or(output4[0] - 1) {
                                            max_output = Some(output4[0]);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    max_output
}

pub fn solve(lines: &[String]) -> Solution {
    let program: Vec<i32> = lines[0].split(',').map(|s| s.parse().unwrap()).collect();
    (
        solve_a(program.clone())
            .map(|i| i.to_string())
            .unwrap_or_else(|| "Failure!".to_string()),
        "foo".to_string(),
    )
}
