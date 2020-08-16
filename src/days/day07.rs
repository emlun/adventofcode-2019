use crate::common::Solution;
use crate::intcode::IntcodeComputer;
use crate::util::Permutations;
use std::collections::VecDeque;

fn run(computer: IntcodeComputer, input: (u8, i64)) -> VecDeque<i64> {
    computer.run(vec![i64::from(input.0), input.1]).output
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

                comp1.input.push_back(*i1);
                comp2.input.push_back(*i2);
                comp3.input.push_back(*i3);
                comp4.input.push_back(*i4);
                comp5.input.push_back(*i5);

                comp1.input.push_back(0);

                while comp5.is_running() {
                    comp1.input.extend(comp5.output.drain(..));
                    comp2.input.extend(comp1.output.drain(..));
                    comp3.input.extend(comp2.output.drain(..));
                    comp4.input.extend(comp3.output.drain(..));
                    comp5.input.extend(comp4.output.drain(..));

                    comp1 = comp1.run(None);
                    comp2 = comp2.run(None);
                    comp3 = comp3.run(None);
                    comp4 = comp4.run(None);
                    comp5 = comp5.run(None);
                }

                let out = comp5.output.pop_front().unwrap();
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
