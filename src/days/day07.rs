use crate::common::Solution;
use crate::intcode;
use crate::util::Permutations;

fn run(mut program: Vec<i64>, input: (u8, i64)) -> Vec<i64> {
    let mut output: Vec<i64> = Vec::new();
    let mut eip = 0;
    let inpt = vec![i64::from(input.0), input.1];
    let mut input_iter = inpt.into_iter();
    let mut next_input = None;
    while program[eip] != 99 {
        if next_input.is_none() {
            next_input = input_iter.next();
        }
        let mut outpt = None;
        let out = intcode::step(eip, program, &mut 0, &mut outpt, &mut next_input);
        if let Some(out) = outpt {
            output.push(out);
        }
        eip = out.0;
        program = out.1;
    }
    output
}

fn solve_a(program: Vec<i64>) -> i64 {
    let mut max_output = None;

    for perm in Permutations::from(0..=4) {
        match perm.as_slice() {
            [i0, i1, i2, i3, i4] => {
                let output0 = run(program.clone(), (*i0, 0));
                let output1 = run(program.clone(), (*i1, output0[0]));
                let output2 = run(program.clone(), (*i2, output1[0]));
                let output3 = run(program.clone(), (*i3, output2[0]));
                let output4 = run(program.clone(), (*i4, output3[0]));
                if output4[0] > max_output.unwrap_or(output4[0] - 1) {
                    max_output = Some(output4[0]);
                }
            }
            _ => unreachable!(),
        }
    }

    max_output.unwrap()
}

fn solve_b(program: Vec<i64>) -> i64 {
    let mut max_output: Option<i64> = None;

    for perm in Permutations::from(5..=9) {
        match perm.as_slice() {
            [i1, i2, i3, i4, i5] => {
                let mut eip1 = 0;
                let mut eip2 = 0;
                let mut eip3 = 0;
                let mut eip4 = 0;
                let mut eip5 = 0;

                let mut prog1 = program.clone();
                let mut prog2 = program.clone();
                let mut prog3 = program.clone();
                let mut prog4 = program.clone();
                let mut prog5 = program.clone();

                let mut input: Option<i64> = Some(*i1);
                let mut output1: Option<i64> = Some(*i2);
                let mut output2: Option<i64> = Some(*i3);
                let mut output3: Option<i64> = Some(*i4);
                let mut output4: Option<i64> = Some(*i5);
                let mut output5: Option<i64> = Some(0);

                while prog5[eip5] != 99 {
                    let out1 = intcode::step(
                        eip1,
                        prog1,
                        &mut 0,
                        &mut output1,
                        if input.is_some() {
                            &mut input
                        } else {
                            &mut output5
                        },
                    );
                    eip1 = out1.0;
                    prog1 = out1.1;

                    let out2 = intcode::step(eip2, prog2, &mut 0, &mut output2, &mut output1);
                    eip2 = out2.0;
                    prog2 = out2.1;

                    let out3 = intcode::step(eip3, prog3, &mut 0, &mut output3, &mut output2);
                    eip3 = out3.0;
                    prog3 = out3.1;

                    let out4 = intcode::step(eip4, prog4, &mut 0, &mut output4, &mut output3);
                    eip4 = out4.0;
                    prog4 = out4.1;

                    let out5 = intcode::step(eip5, prog5, &mut 0, &mut output5, &mut output4);
                    eip5 = out5.0;
                    prog5 = out5.1;
                }

                let out = output5.take().unwrap();
                if out > max_output.unwrap_or(out - 1) {
                    max_output = Some(out);
                }
            }
            _ => unreachable!(),
        }
    }

    max_output.unwrap()
}

pub fn solve(lines: &[String]) -> Solution {
    let program = intcode::parse(lines);
    (
        solve_a(program.clone()).to_string(),
        solve_b(program).to_string(),
    )
}
