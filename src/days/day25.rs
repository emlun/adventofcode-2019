use crate::common::Solution;
use crate::intcode::IntcodeComputer;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::collections::VecDeque;

type Point = (i16, i16);

#[derive(Debug)]
struct State {
    pos: Point,
    items: BTreeSet<String>,
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
            pos: (0, 0),
            items: BTreeSet::new(),
            state: 0,
            command_history: Vec::new(),
            input: VecDeque::new(),
            output: Vec::new(),
        }
    }
}

fn next_commands(state: &State) -> Vec<String> {
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

    let mut commands: Vec<String> = Vec::new();
    for dir in directions {
        commands.push(dir);
    }
    for item in items {
        // commands.push(format!("take {}", item));
    }
    commands
}

fn solve_a(mut computer: IntcodeComputer) -> String {
    let mut computers = VecDeque::new();
    let mut visited: HashSet<(Point, BTreeSet<String>)> = HashSet::new();
    computers.push_back((computer, State::new()));
    while let Some((computer, state)) = computers.pop_front() {
        visited.insert((state.pos, state.items.clone()));
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

        let commands = next_commands(&finish);
        println!("{}", finish.output.iter().collect::<String>());
        println!("{:?}", commands);

        for command in commands {
            let mut new_state = State::new();
            new_state.input = command.chars().collect();
            new_state.input.push_back('\n');
            new_state.pos = match command.as_str() {
                "north" => (finish.pos.0, finish.pos.1 + 1),
                "south" => (finish.pos.0, finish.pos.1 - 1),
                "east" => (finish.pos.0 + 1, finish.pos.1),
                "west" => (finish.pos.0 - 1, finish.pos.1),
                _ => finish.pos,
            };
            if &command[0..4] == "take" {
                new_state.items.insert(command[5..].to_string());
            }
            new_state.command_history = finish.command_history.clone();
            new_state.command_history.push(command);
            if !visited.contains(&(new_state.pos, new_state.items.clone())) {
                computers.push_back((computer.clone(), new_state));
            }
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
