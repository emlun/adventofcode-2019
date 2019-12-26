use crate::common::Solution;
use crate::intcode::IntcodeComputer;
use std::collections::VecDeque;
use std::io::Read;

type Point = (i8, i8);
type Direction = (i8, i8);

#[derive(Clone, Debug)]
struct State {
    state: u8,
    pos: Vec<Point>,
    unexplored_pos: Vec<Vec<Point>>,
    items: Vec<String>,
    path_to_security: Vec<Direction>,
    security_pos: Option<Point>,
    next_commands: VecDeque<String>,
    unlock_attempt: u32,
    solution: Option<String>,
}

impl State {
    const COLLECT: u8 = 0;
    const NAVIGATE: u8 = 1;
    const UNLOCK: u8 = 2;
    const DONE: u8 = 3;

    fn new() -> Self {
        Self {
            state: Self::COLLECT,
            pos: vec![(0, 0)],
            unexplored_pos: Vec::new(),
            items: Vec::new(),
            path_to_security: Vec::new(),
            security_pos: None,
            next_commands: VecDeque::new(),
            unlock_attempt: 0,
            solution: None,
        }
    }

    fn current_pos(&self) -> Point {
        self.pos[self.pos.len() - 1]
    }

    fn backtrack_dir(&self) -> Direction {
        let pos = self.current_pos();
        let prev = self.pos[self.pos.len() - 2];
        (prev.0 - pos.0, prev.1 - pos.1)
    }

    fn backtrack_move(&self) -> &'static str {
        dir_to_move(self.backtrack_dir())
    }

    fn backtrack(mut self) -> Self {
        self.unexplored_pos.pop();
        self.next_commands
            .push_back(self.backtrack_move().to_string());
        let prev_pos = self.current_pos();
        self.pos.pop();

        if self.security_pos.is_some() {
            let pos = self.current_pos();

            if !self.path_to_security.is_empty()
                && self.path_to_security[self.path_to_security.len() - 1] == pos
            {
                self.path_to_security.pop();
            } else {
                self.path_to_security.push(prev_pos);
            }
        }
        self
    }

    fn take(mut self, item: String) -> Self {
        self.next_commands.push_back(format!("take {}", item));
        self.items.push(item);
        self
    }

    fn explore(mut self, doors: Vec<String>) -> Self {
        if self.unexplored_pos.iter().all(|poss| poss.is_empty()) {
            self.state = Self::NAVIGATE;
            self.navigate()
        } else {
            let unlen = self.unexplored_pos.len() - 1;
            while let Some(unexplored) = self.unexplored_pos[unlen].pop() {
                if self.pos.len() > 1 && unexplored == self.pos[self.pos.len() - 2] {
                    continue;
                }

                let pos = self.current_pos();
                let move_dir = (unexplored.0 - pos.0, unexplored.1 - pos.1);
                let move_command = dir_to_move(move_dir).to_string();
                if doors.contains(&move_command) {
                    if self.security_pos.is_some() {
                        self.path_to_security.push(pos);
                    }

                    self.next_commands.push_back(move_command);
                    self.pos.push(unexplored);

                    let mut next_unexplored = Vec::new();
                    for next_dpos in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                        let next = (unexplored.0 + next_dpos.0, unexplored.1 + next_dpos.1);
                        next_unexplored.push(next);
                    }
                    self.unexplored_pos.push(next_unexplored);

                    return self;
                }
            }
            self.backtrack()
        }
    }

    fn navigate(mut self) -> Self {
        while let Some(step) = self.path_to_security.pop() {
            let pos = self.current_pos();
            if self.pos.len() > 2 && self.pos[self.pos.len() - 2] == step {
                self.pos.pop();
            } else {
                self.pos.push(step);
            }
            let dir = (step.0 - pos.0, step.1 - pos.1);
            let move_command = dir_to_move(dir);
            self.next_commands.push_back(move_command.to_string());
        }
        self.unlock_attempt = (1 << self.items.len()) - 1;
        self.state = Self::UNLOCK;
        self
    }

    fn unlock(mut self, room: Room) -> Self {
        if let Some(solution) = room.solution {
            self.solution = Some(solution);
            self.state = Self::DONE;
        } else {
            let next_attempt = if self.unlock_attempt == (1 << self.items.len()) - 1 {
                self.unlock_attempt
            } else {
                self.unlock_attempt - 1
            };

            let pos = self.current_pos();
            let prev_pos = self.pos[self.pos.len() - 2];
            let move_command = room
                .doors
                .into_iter()
                .find(|door| move_to_point(pos, door) != prev_pos)
                .unwrap();

            for i in 0..self.items.len() {
                let mask = 1 << i;
                match (self.unlock_attempt & mask > 0, next_attempt & mask > 0) {
                    (true, false) => self
                        .next_commands
                        .push_back(format!("drop {}", self.items[i])),
                    (false, true) => self
                        .next_commands
                        .push_back(format!("take {}", self.items[i])),
                    _ => {}
                }
            }
            self.unlock_attempt -= 1;
            self.next_commands.push_back(move_command);
        }
        self
    }
}

fn dir_to_move(dir: Direction) -> &'static str {
    match dir {
        (0, 1) => "north",
        (1, 0) => "east",
        (0, -1) => "south",
        (-1, 0) => "west",
        _ => unreachable!(),
    }
}

struct Room {
    name: String,
    doors: Vec<String>,
    items: Vec<String>,
    solution: Option<String>,
}

fn parse_room(output: String) -> Room {
    let mut words: VecDeque<&str> = output.split_whitespace().collect();

    let mut name: String = "".to_string();
    let mut doors: Vec<String> = Vec::new();
    let mut items: Vec<String> = Vec::new();
    let mut solution = None;

    while !words.is_empty() {
        match words[0] {
            "==" => {
                let mut nm = vec![];
                words.pop_front();
                while let Some(word) = words.pop_front() {
                    if word == "==" {
                        name = nm.join(" ");
                        break;
                    } else {
                        nm.push(word);
                    }
                }
            }

            "Doors" => {
                words.pop_front();
                words.pop_front();
                words.pop_front();
                doors.clear();
                while words[0] == "-" {
                    words.pop_front();
                    doors.push(words.pop_front().unwrap().to_string());
                }
            }

            "Items" => {
                words.pop_front();
                words.pop_front();
                while words[0] == "-" {
                    words.pop_front();
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
                            items.push(item);
                        }
                    };
                }
            }

            "\"Analysis" => {
                words.pop_front();
                assert_eq!(words.pop_front(), Some("complete!"));
                assert_eq!(words.pop_front(), Some("You"));
                assert_eq!(words.pop_front(), Some("may"));
                assert_eq!(words.pop_front(), Some("proceed.\""));
                while let Some(word) = words.pop_front() {
                    if word == "typing" {
                        solution = words.pop_front().map(|s| s.to_string());
                        break;
                    }
                }
            }

            _ => {
                words.pop_front();
            }
        }
    }

    Room {
        name,
        doors,
        items,
        solution,
    }
}

fn update(mut state: State, output: String) -> State {
    state.next_commands.clear();
    match state.state {
        State::COLLECT => {
            let room = parse_room(output);
            if room.name == "Security Checkpoint" {
                state.security_pos = Some(state.current_pos());
                state.backtrack()
            } else {
                for item in room.items {
                    state = state.take(item);
                }

                state.explore(room.doors)
            }
        }

        State::NAVIGATE => state.navigate(),
        State::UNLOCK => state.unlock(parse_room(output)),
        _ => unreachable!(),
    }
}

fn move_to_point(pos: Point, move_command: &str) -> Point {
    let dir = match move_command {
        "north" => (0, 1),
        "east" => (1, 0),
        "south" => (0, -1),
        "west" => (-1, 0),
        _ => unreachable!(),
    };
    (pos.0 + dir.0, pos.1 + dir.1)
}

fn initialize(mut state: State, output: String) -> State {
    let room = parse_room(output);
    for item in room.items {
        state = state.take(item);
    }
    state.unexplored_pos.push(
        room.doors
            .iter()
            .map(|door| move_to_point((0, 0), door))
            .collect(),
    );
    state.explore(room.doors)
}

fn solve_a(mut computer: IntcodeComputer) -> String {
    let mut state = State::new();

    let o = computer.run_until_more_input_required(None);
    computer = o.0;
    let first_output: String = o.1.into_iter().map(|i| i as u8 as char).collect();
    // println!("{}", first_output);
    state = initialize(state, first_output);

    loop {
        let mut output: String = "".to_string();

        while let Some(cmd) = state.next_commands.pop_front() {
            // println!("pos: {:?}", state.pos);
            // println!("items: {:?}", state.items);
            // println!("unexplored: {:?}", state.unexplored_pos);
            // println!("Command: {}", cmd);
            let input: Vec<i64> = format!("{}\n", cmd)
                .chars()
                .map(|c| c as u8 as i64)
                .collect();
            let o = computer.run_until_more_input_required(input);
            computer = o.0;
            output = o.1.into_iter().map(|i| i as u8 as char).collect();
            // println!("{}", output);
        }

        state = update(state, output);

        if state.solution.is_some() {
            return state.solution.unwrap();
        }
    }
}

#[allow(dead_code)]
fn interact(computer: IntcodeComputer) {
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
}

pub fn solve(lines: &[String]) -> Solution {
    let computer: IntcodeComputer = lines.into();
    // interact(computer);
    let a_solution = solve_a(computer);
    (a_solution, "-".to_string())
}
