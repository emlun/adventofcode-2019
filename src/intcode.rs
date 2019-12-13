use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Clone)]
pub struct IntcodeComputer {
    eip: usize,
    pub prog: Vec<i64>,
    relbase: i64,
}

impl IntcodeComputer {
    pub fn new(program: Vec<i64>) -> IntcodeComputer {
        IntcodeComputer {
            eip: 0,
            prog: program,
            relbase: 0,
        }
    }

    pub fn step(&mut self, input: &mut Option<i64>) -> Option<i64> {
        let instruction = self.prog[self.eip];
        let opcode = instruction % 100;
        let eip = self.eip;
        let relbase = self.relbase;
        let mut output = None;

        fn ensure_size(prog: &mut Vec<i64>, size: usize) {
            if size >= prog.len() {
                prog.append(&mut (0..=0).cycle().take(size - prog.len() + 1).collect());
            }
        };

        let get_addr = |prog: &mut Vec<i64>, offset: usize| -> usize {
            let parmode_pow = 10i64.pow((offset + 1).try_into().unwrap());
            let out_addr = match (instruction / parmode_pow) % 10 {
                0 => usize::try_from(prog[eip + offset]).unwrap(),
                1 => eip + offset,
                2 => usize::try_from(relbase + prog[eip + offset]).unwrap(),
                _ => unreachable!(),
            };
            ensure_size(prog, out_addr);
            out_addr
        };

        let get_args = |prog: &mut Vec<i64>, num: usize| -> Vec<i64> {
            (1..=num)
                .map(|i| {
                    let addr = get_addr(prog, i);
                    prog[addr]
                })
                .collect()
        };

        self.eip = match opcode {
            1 => {
                let args = get_args(&mut self.prog, 2);
                let io = get_addr(&mut self.prog, 3);
                self.prog[io] = args[0] + args[1];
                self.eip + 4
            }
            2 => {
                let args = get_args(&mut self.prog, 2);
                let io = get_addr(&mut self.prog, 3);
                self.prog[io] = args[0] * args[1];
                self.eip + 4
            }
            3 => {
                let io = get_addr(&mut self.prog, 1);
                if let Some(i) = input.take() {
                    self.prog[io] = i;
                    self.eip + 2
                } else {
                    self.eip
                }
            }
            4 => {
                let args = get_args(&mut self.prog, 1);
                output = Some(args[0]);
                self.eip + 2
            }
            5 => {
                let args = get_args(&mut self.prog, 2);
                if args[0] != 0 {
                    args[1] as usize
                } else {
                    self.eip + 3
                }
            }
            6 => {
                let args = get_args(&mut self.prog, 2);
                if args[0] == 0 {
                    args[1] as usize
                } else {
                    self.eip + 3
                }
            }
            7 => {
                let args = get_args(&mut self.prog, 2);
                let io = get_addr(&mut self.prog, 3);
                self.prog[io] = if args[0] < args[1] { 1 } else { 0 };
                self.eip + 4
            }
            8 => {
                let args = get_args(&mut self.prog, 2);
                let io = get_addr(&mut self.prog, 3);
                self.prog[io] = if args[0] == args[1] { 1 } else { 0 };
                self.eip + 4
            }
            9 => {
                let args = get_args(&mut self.prog, 1);
                self.relbase += args[0];
                self.eip + 2
            }
            99 => self.eip,
            _ => unreachable!(),
        };
        output
    }

    pub fn run<I>(mut self, input: I) -> Vec<i64>
    where
        I: IntoIterator<Item = i64>,
    {
        let mut outputs = Vec::new();
        let mut inputs = input.into_iter();
        let mut next_input = inputs.next();
        while self.is_running() {
            if next_input.is_none() {
                next_input = inputs.next();
            }
            if let Some(o) = self.step(&mut next_input) {
                outputs.push(o);
            };
        }
        outputs
    }

    pub fn run_with<State, F>(
        mut self,
        initial_input: Option<i64>,
        initial_state: State,
        reducer: F,
    ) -> State
    where
        F: Fn(Option<i64>, State) -> (Option<i64>, State),
    {
        let mut input = initial_input;
        let mut state = initial_state;
        while self.is_running() {
            let output = self.step(&mut input);
            let stepout = reducer(output, state);
            input = stepout.0;
            state = stepout.1;
        }
        state
    }

    pub fn is_running(&self) -> bool {
        !self.is_halted()
    }

    pub fn is_halted(&self) -> bool {
        self.prog[self.eip] == 99
    }
}

impl From<&[String]> for IntcodeComputer {
    fn from(lines: &[String]) -> IntcodeComputer {
        Self::new(parse_program(lines))
    }
}

pub fn parse_program(lines: &[String]) -> Vec<i64> {
    lines[0].split(',').map(|s| s.parse().unwrap()).collect()
}
