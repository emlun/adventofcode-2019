use crate::common::Solution;
use crate::intcode::IntcodeComputer;
use crate::util::Permutations;

fn run(computer: IntcodeComputer, input: (u8, i64)) -> Vec<i64> {
    computer.run(vec![i64::from(input.0), input.1])
}

fn solve_a(computer: IntcodeComputer) -> i64 {
    let mut max_output = None;

    for perm in Permutations::from(0..=4) {
        match perm.as_slice() {
            [i0, i1, i2, i3, i4] => {
                let output0 = run(computer.clone(), (*i0, 0));
                let output1 = run(computer.clone(), (*i1, output0[0]));
                let output2 = run(computer.clone(), (*i2, output1[0]));
                let output3 = run(computer.clone(), (*i3, output2[0]));
                let output4 = run(computer.clone(), (*i4, output3[0]));
                if output4[0] > max_output.unwrap_or(output4[0] - 1) {
                    max_output = Some(output4[0]);
                }
            }
            _ => unreachable!(),
        }
    }

    max_output.unwrap()
}

fn solve_b(computer: IntcodeComputer) -> i64 {
    let mut max_output: Option<i64> = None;

    for perm in Permutations::from(5..=9) {
        match perm.as_slice() {
            [i1, i2, i3, i4, i5] => {
                let mut comp1 = computer.clone();
                let mut comp2 = computer.clone();
                let mut comp3 = computer.clone();
                let mut comp4 = computer.clone();
                let mut comp5 = computer.clone();

                let mut input1: Option<i64> = Some(*i1);
                let mut input2: Option<i64> = Some(*i2);
                let mut input3: Option<i64> = Some(*i3);
                let mut input4: Option<i64> = Some(*i4);
                let mut input5: Option<i64> = Some(*i5);

                let mut output1: Option<i64> = None;
                let mut output2: Option<i64> = None;
                let mut output3: Option<i64> = None;
                let mut output4: Option<i64> = None;
                let mut output5: Option<i64> = Some(0);

                while comp5.is_running() {
                    if output1.is_none() {
                        output1 = comp1
                            .step(if input1.is_some() {
                                &mut input1
                            } else {
                                &mut output5
                            })
                            .take();
                    }
                    if output2.is_none() {
                        output2 = comp2
                            .step(if input2.is_some() {
                                &mut input2
                            } else {
                                &mut output1
                            })
                            .take();
                    }
                    if output3.is_none() {
                        output3 = comp3
                            .step(if input3.is_some() {
                                &mut input3
                            } else {
                                &mut output2
                            })
                            .take();
                    }
                    if output4.is_none() {
                        output4 = comp4
                            .step(if input4.is_some() {
                                &mut input4
                            } else {
                                &mut output3
                            })
                            .take();
                    }
                    if output5.is_none() {
                        output5 = comp5
                            .step(if input5.is_some() {
                                &mut input5
                            } else {
                                &mut output4
                            })
                            .take();
                    }
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
    let computer: IntcodeComputer = lines.into();
    (
        solve_a(computer.clone()).to_string(),
        solve_b(computer).to_string(),
    )
}
