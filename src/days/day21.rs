use crate::common::Solution;
use crate::intcode::IntcodeComputer;
use std::collections::VecDeque;

struct State {
    output: Vec<i64>,
    script_input: VecDeque<char>,
    walk_input: VecDeque<char>,
}

fn simulate(computer: IntcodeComputer, run: bool, script: &str) -> i64 {
    let finish = computer.run_with_expect(
        None,
        State {
            output: Vec::new(),
            script_input: script.chars().collect(),
            walk_input: if run { "RUN\n" } else { "WALK\n" }.chars().collect(),
        },
        |output, expects_input, mut state| -> (Option<i64>, State) {
            if let Some(o) = output {
                state.output.push(o);
            }

            if expects_input {
                let input = state
                    .script_input
                    .pop_front()
                    .or_else(|| state.walk_input.pop_front());
                if input.is_some() {
                    (input.map(|i| i as u8 as i64), state)
                } else {
                    unreachable!()
                }
            } else {
                (None, state)
            }
        },
    );

    if finish.output.contains(&(b'D' as i64)) {
        println!("{:?}", finish.output);
        println!(
            "{}",
            finish
                .output
                .iter()
                .map(|i| *i as u8 as char)
                .collect::<String>()
        );
        -1
    } else {
        *finish.output.last().unwrap()
    }
}

fn solve_a(computer: IntcodeComputer) -> i64 {
    simulate(
        computer,
        false,
        "NOT C J
AND D J
NOT A T
OR T J
",
    )
}

fn solve_b(computer: IntcodeComputer) -> i64 {
    simulate(
        computer,
        true,
        "NOT A T
OR T J
NOT B T
OR T J
NOT C T
OR T J
AND E T
OR H T
AND T J
AND D J
",
    )
}

pub fn solve(lines: &[String]) -> Solution {
    let computer: IntcodeComputer = lines.into();
    let a_solution = solve_a(computer.clone());
    let b_solution = solve_b(computer);
    (a_solution.to_string(), b_solution.to_string())
}
