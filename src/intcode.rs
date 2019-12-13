use std::convert::TryFrom;
use std::convert::TryInto;

type Word = i64;
type Memory = Vec<Word>;

#[derive(Clone)]
pub struct IntcodeComputer {
    eip: usize,
    pub prog: Memory,
    relbase: Word,
}

const OP_ADD: Word = 1;
const OP_MULTIPLY: Word = 2;
const OP_INPUT: Word = 3;
const OP_OUTPUT: Word = 4;
const OP_JUMP_NONZERO: Word = 5;
const OP_JUMP_ZERO: Word = 6;
const OP_LESS: Word = 7;
const OP_EQ: Word = 8;
const OP_RELBASE: Word = 9;
const OP_HALT: Word = 99;

impl IntcodeComputer {
    pub fn new(program: Vec<Word>) -> IntcodeComputer {
        IntcodeComputer {
            eip: 0,
            prog: program,
            relbase: 0,
        }
    }

    pub fn step(&mut self, input: &mut Option<Word>) -> Option<Word> {
        let instruction = self.prog[self.eip];
        let opcode = instruction % 100;
        let eip = self.eip;
        let relbase = self.relbase;
        let mut output = None;

        fn ensure_size(prog: &mut Memory, size: usize) {
            if size >= prog.len() {
                prog.append(&mut (0..=0).cycle().take(size - prog.len() + 1).collect());
            }
        };

        let get_addr = |prog: &mut Memory, offset: usize| -> usize {
            let parmode_pow = 10_i64.pow((offset + 1).try_into().unwrap());
            let out_addr = match (instruction / parmode_pow) % 10 {
                0 => usize::try_from(prog[eip + offset]).unwrap(),
                1 => eip + offset,
                2 => usize::try_from(relbase + prog[eip + offset]).unwrap(),
                _ => unreachable!(),
            };
            ensure_size(prog, out_addr);
            out_addr
        };

        let get_args = |prog: &mut Memory, num: usize| -> Memory {
            (1..=num)
                .map(|i| {
                    let addr = get_addr(prog, i);
                    prog[addr]
                })
                .collect()
        };

        self.eip = match opcode {
            OP_ADD => {
                let args = get_args(&mut self.prog, 2);
                let io = get_addr(&mut self.prog, 3);
                self.prog[io] = args[0] + args[1];
                self.eip + 4
            }

            OP_MULTIPLY => {
                let args = get_args(&mut self.prog, 2);
                let io = get_addr(&mut self.prog, 3);
                self.prog[io] = args[0] * args[1];
                self.eip + 4
            }

            OP_INPUT => {
                let io = get_addr(&mut self.prog, 1);
                if let Some(i) = input.take() {
                    self.prog[io] = i;
                    self.eip + 2
                } else {
                    self.eip
                }
            }

            OP_OUTPUT => {
                let args = get_args(&mut self.prog, 1);
                output = Some(args[0]);
                self.eip + 2
            }

            OP_JUMP_NONZERO => {
                let args = get_args(&mut self.prog, 2);
                if args[0] != 0 {
                    args[1] as usize
                } else {
                    self.eip + 3
                }
            }

            OP_JUMP_ZERO => {
                let args = get_args(&mut self.prog, 2);
                if args[0] == 0 {
                    args[1] as usize
                } else {
                    self.eip + 3
                }
            }

            OP_LESS => {
                let args = get_args(&mut self.prog, 2);
                let io = get_addr(&mut self.prog, 3);
                self.prog[io] = if args[0] < args[1] { 1 } else { 0 };
                self.eip + 4
            }

            OP_EQ => {
                let args = get_args(&mut self.prog, 2);
                let io = get_addr(&mut self.prog, 3);
                self.prog[io] = if args[0] == args[1] { 1 } else { 0 };
                self.eip + 4
            }

            OP_RELBASE => {
                let args = get_args(&mut self.prog, 1);
                self.relbase += args[0];
                self.eip + 2
            }

            OP_HALT => self.eip,
            _ => unreachable!(),
        };
        output
    }

    pub fn run<I>(mut self, input: I) -> Vec<Word>
    where
        I: IntoIterator<Item = Word>,
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
        initial_input: Option<Word>,
        initial_state: State,
        reducer: F,
    ) -> State
    where
        F: Fn(Option<Word>, State) -> (Option<Word>, State),
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
        self.prog[self.eip] == OP_HALT
    }
}

impl From<&[String]> for IntcodeComputer {
    fn from(lines: &[String]) -> IntcodeComputer {
        Self::new(parse_program(lines))
    }
}

pub fn parse_program(lines: &[String]) -> Vec<Word> {
    lines[0].split(',').map(|s| s.parse().unwrap()).collect()
}
