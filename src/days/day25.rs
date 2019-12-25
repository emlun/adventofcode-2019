use crate::common::Solution;
use crate::intcode::IntcodeComputer;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct State {
    state: u8,
    command_history: Vec<String>,
    input: VecDeque<char>,
    output: Vec<char>,
}

impl State {
    const WAITING: u8 = 0;
    const WRITING: u8 = 1;

    fn new() -> Self {
        Self {
            state: 0,
            command_history: Vec::new(),
            input: VecDeque::new(),
            output: Vec::new(),
        }
    }
}

fn next_commands(state: &State) -> (Vec<String>, Vec<String>) {
    let output: String = state.output.iter().collect();
    let mut words: VecDeque<&str> = output.split_whitespace().collect();

    let mut directions: Vec<String> = Vec::new();
    let mut items: Vec<String> = Vec::new();

    while words[0] != "Command?" {
        match words[0] {
            "Doors" => {
                debug_assert_eq!(words.pop_front(), Some("Doors"));
                debug_assert_eq!(words.pop_front(), Some("here"));
                debug_assert_eq!(words.pop_front(), Some("lead:"));
                while words[0] == "-" {
                    debug_assert_eq!(words.pop_front(), Some("-"));
                    directions.push(words.pop_front().unwrap().to_string());
                }
            }

            "Items" => {
                debug_assert_eq!(words.pop_front(), Some("Items"));
                debug_assert_eq!(words.pop_front(), Some("here:"));
                while words[0] == "-" {
                    debug_assert_eq!(words.pop_front(), Some("-"));
                    let mut item: Vec<&str> = Vec::new();
                    while words[0] != "-" && words[0] != "Command?" {
                        item.push(words.pop_front().unwrap());
                    }
                    items.push(item.join(" "));
                }
            }

            _ => {
                words.pop_front();
            }
        }
    }

    let move_commands: Vec<String> = directions;
    let mut item_commands: Vec<String> = items
        .into_iter()
        .map(|item| format!("take {}", item))
        .collect();
    (move_commands, item_commands)
}

fn solve_a(mut computer: IntcodeComputer) -> String {
    let mut computers = VecDeque::new();
    computers.push_back((computer, State::new()));
    while let Some((computer, state)) = computers.pop_front() {
        println!("Continuing computer: {:?}", state.command_history);

        let (computer, finish) =
            computer.run_with_halt_expect(None, state, |output, expects_input, mut state| {
                state
                    .output
                    .append(&mut output.into_iter().map(|c| c as u8 as char).collect());

                if expects_input {
                    match state.state {
                        State::WAITING => {
                            state.state = State::WRITING;
                            (None, state, false)
                        }

                        State::WRITING => {
                            let input = state.input.pop_front().map(|i| i as u8 as i64);
                            if input.is_none() {
                                (None, state, true)
                            } else if state.input.is_empty() {
                                state.state = State::WAITING;
                                (input, state, false)
                            } else {
                                (input, state, false)
                            }
                        }

                        _ => unreachable!(),
                    }
                } else {
                    (None, state, false)
                }
            });

        let (moves, items) = next_commands(&finish);
        println!("{}", finish.output.iter().collect::<String>());
        println!("{:?} {:?}", moves, items);

        for item_command in items.iter() {
            let mut take_state = State::new();
            take_state.command_history = finish.command_history.clone();
            take_state.command_history.push(item_command.clone());

            for move_command in moves.iter() {
                let mut move_state = take_state.clone();
                move_state.input = format!("{}\n{}\n", item_command, move_command)
                    .chars()
                    .collect();
                move_state.command_history.push(move_command.clone());
                computers.push_back((computer.clone(), move_state));
            }

            computers.push_back((computer.clone(), take_state));
        }

        for move_command in moves {
            let mut move_state = State::new();
            move_state.command_history = finish.command_history.clone();
            move_state.input = move_command.chars().collect();
            move_state.command_history.push(move_command);
            move_state.input.push_back('\n');
            computers.push_back((computer.clone(), move_state));
        }
    }

    "".to_string()
}

pub fn solve(lines: &[String]) -> Solution {
    let computer: IntcodeComputer = lines.into();
    let a_solution = solve_a(computer);
    // let b_solution = solve_b(&computer);
    let b_solution = "";
    (a_solution, b_solution.to_string())
}
