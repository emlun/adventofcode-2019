use crate::common::Solution;
use crate::intcode::IntcodeComputer;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::io::Read;

type Point = (i16, i16);

#[derive(Clone, Debug)]
struct State {
    pos: Point,
    items: BTreeSet<String>,
    state: u8,
    command_history: Vec<String>,
    commands: VecDeque<String>,
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
            commands: VecDeque::new(),
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

    while !words.is_empty() {
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
                    let item = item.join(" ");
                    match item.as_str() {
                        "infinite loop"
                        | "photons"
                        | "molten lava"
                        | "escape pod"
                        | "giant electromagnet" => {}
                        _ => {
                            // if state.items.len() < 1 || item == "dark matter" || item == "hypercube"
                            items.push(item);
                        }
                    };
                }
            }

            "Analyzing..." => {
                words.pop_front();
                debug_assert_eq!(words.pop_front(), Some("Doors"));
                debug_assert_eq!(words.pop_front(), Some("here"));
                debug_assert_eq!(words.pop_front(), Some("lead:"));
                debug_assert_eq!(words.pop_front(), Some("-"));
                directions.push(words.pop_front().unwrap().to_string());

                while words[0] != "==" && words[0] != "Command?" {
                    if words[0] == "you" && words[1] == "are" && words[2] == "ejected" {
                        return (vec!["eject".to_string(), directions[0].clone()], Vec::new());
                    }
                    words.pop_front();
                }
                println!("Made it through!?");
            }

            _ => {
                words.pop_front();
            }
        }
    }

    (directions, items)
}

fn cheat(computer: IntcodeComputer) {
    println!(
        "{}",
        computer
            .prog
            .iter()
            .flat_map(|i| char::try_from(*i as u8).into_iter())
            .collect::<String>()
    )
}

fn solve_a(computer: IntcodeComputer) -> String {
    let mut computers = VecDeque::new();
    let mut visited: HashSet<(Point, BTreeSet<String>)> = HashSet::new();
    computers.push_back((computer, State::new()));

    fn is_visited(visited: &HashSet<(Point, BTreeSet<String>)>, state: &State) -> bool {
        const SECURITY_POS: Point = (1, 3);
        const PRESSURE_POS: Point = (0, 3);
        visited.iter().any(|(pos, items)| {
            *pos == state.pos
                && !(*pos == SECURITY_POS || *pos == PRESSURE_POS)
                && items.is_superset(&state.items)
        })
    }

    while let Some((computer, state)) = computers.pop_front() {
        println!("state: {:?} {:?}", state.pos, state.items);
        println!("visited contains: {:?}", is_visited(&visited, &state));
        if is_visited(&visited, &state) {
            continue;
        }
        visited.insert((state.pos, state.items.clone()));
        println!("{}", computers.len());
        println!(
            "Resuming computer: {} {:?}",
            state.command_history.len(),
            state.command_history
        );

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
                            if state.input.is_empty() {
                                if let Some(command) = state.commands.pop_front() {
                                    state.input.append(&mut command.chars().collect());
                                    state.input.push_back('\n');
                                    state.command_history.push(command);
                                }
                            }
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

        if !moves.is_empty() && moves[0] == "eject" {
            println!("Tried items: {:?}", finish.items);
            let mut new_state = finish.clone();
            new_state.pos = match moves[1].as_str() {
                "north" => (finish.pos.0, finish.pos.1 + 1),
                "south" => (finish.pos.0, finish.pos.1 - 1),
                "east" => (finish.pos.0 + 1, finish.pos.1),
                "west" => (finish.pos.0 - 1, finish.pos.1),
                _ => unreachable!(),
            };

            for item in finish.items {
                let mut drop_state = new_state.clone();
                drop_state.items.remove(&item);
                drop_state.commands.push_back("inv".to_string());
                drop_state.commands.push_back(format!("drop {}", item));
                drop_state
                    .commands
                    .push_back(drop_state.command_history.last().unwrap().clone());

                if !is_visited(&visited, &drop_state) {
                    computers.push_back((computer.clone(), drop_state));
                }
            }
        } else {
            for item in items {
                for move_command in moves.iter() {
                    let mut new_state = finish.clone();
                    new_state.output.clear();
                    new_state.pos = match move_command.as_str() {
                        "north" => (finish.pos.0, finish.pos.1 + 1),
                        "south" => (finish.pos.0, finish.pos.1 - 1),
                        "east" => (finish.pos.0 + 1, finish.pos.1),
                        "west" => (finish.pos.0 - 1, finish.pos.1),
                        _ => finish.pos,
                    };
                    new_state.commands.push_back(format!("take {}", item));
                    new_state.commands.push_back(move_command.clone());
                    new_state.items.insert(item.clone());
                    if !is_visited(&visited, &new_state) {
                        computers.push_back((computer.clone(), new_state));
                    }
                }
            }

            for move_command in moves.iter() {
                let mut new_state = finish.clone();
                new_state.output.clear();
                new_state.pos = match move_command.as_str() {
                    "north" => (finish.pos.0, finish.pos.1 + 1),
                    "south" => (finish.pos.0, finish.pos.1 - 1),
                    "east" => (finish.pos.0 + 1, finish.pos.1),
                    "west" => (finish.pos.0 - 1, finish.pos.1),
                    _ => finish.pos,
                };
                new_state.commands.push_back(move_command.clone());
                if !is_visited(&visited, &new_state) {
                    computers.push_back((computer.clone(), new_state));
                }
            }
        }
    }

    "".to_string()
}

fn interact(computer: IntcodeComputer) -> String {
    computer.run_with_halt_expect(
        None,
        VecDeque::new(),
        |output, expects_input, mut input_queue| {
            if let Some(o) = output {
                print!("{}", o as u8 as char);
            }

            if expects_input {
                if input_queue.is_empty() {
                    let mut buf = [0; 100];
                    let mut len = 0;
                    while len == 0 || (buf[len - 1] as char) != '\n' {
                        let n = std::io::stdin().read(&mut buf[len..]).unwrap();
                        len += n;
                    }
                    for b in buf[0..len].iter() {
                        input_queue.push_back(*b as i64);
                    }
                }
                (input_queue.pop_front(), input_queue, false)
            } else {
                (None, input_queue, false)
            }
        },
    );

    "".to_string()
}

pub fn solve(lines: &[String]) -> Solution {
    let computer: IntcodeComputer = lines.into();
    // cheat(computer);
    interact(computer);
    // let a_solution = solve_a(computer);
    // let a_solution = solve_a(computer);
    // let b_solution = solve_b(&computer);
    let a_solution = "".to_string();
    let b_solution = "";
    (a_solution, b_solution.to_string())
}
