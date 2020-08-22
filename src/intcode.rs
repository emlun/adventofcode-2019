use std::collections::VecDeque;

type Word = i64;
type Memory = Vec<Word>;

#[derive(Clone)]
pub struct IntcodeComputer {
    pub eip: usize,
    pub prog: Memory,
    relbase: Word,
    pub input: VecDeque<Word>,
    pub output: VecDeque<Word>,
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
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }

    pub fn step(&mut self) {
        let instruction = self.prog[self.eip];
        let opcode = instruction % 100;
        let eip = self.eip;
        let relbase = self.relbase;

        let get_addr = |prog: &mut Memory, offset: usize| -> usize {
            let parmode_pow = match offset {
                1 => 100,
                2 => 1000,
                3 => 10000,
                _ => unreachable!(),
            };
            let out_addr = match (instruction / parmode_pow) % 10 {
                0 => (prog[eip + offset]) as usize,
                1 => eip + offset,
                2 => (relbase + prog[eip + offset]) as usize,
                _ => unreachable!(),
            };
            if out_addr >= prog.len() {
                prog.resize(out_addr + 1, 0);
            }
            out_addr
        };

        let get_arg = |prog: &mut Memory, arg_num: usize| -> Word {
            let addr = get_addr(prog, arg_num);
            prog[addr]
        };

        self.eip = match opcode {
            OP_ADD => {
                let io = get_addr(&mut self.prog, 3);
                self.prog[io] = get_arg(&mut self.prog, 1) + get_arg(&mut self.prog, 2);
                self.eip + 4
            }

            OP_MULTIPLY => {
                let io = get_addr(&mut self.prog, 3);
                self.prog[io] = get_arg(&mut self.prog, 1) * get_arg(&mut self.prog, 2);
                self.eip + 4
            }

            OP_INPUT => {
                let io = get_addr(&mut self.prog, 1);
                if let Some(i) = self.input.pop_front() {
                    self.prog[io] = i;
                    self.eip + 2
                } else {
                    self.eip
                }
            }

            OP_OUTPUT => {
                self.output.push_back(get_arg(&mut self.prog, 1));
                self.eip + 2
            }

            OP_JUMP_NONZERO => {
                if get_arg(&mut self.prog, 1) != 0 {
                    get_arg(&mut self.prog, 2) as usize
                } else {
                    self.eip + 3
                }
            }

            OP_JUMP_ZERO => {
                if get_arg(&mut self.prog, 1) == 0 {
                    get_arg(&mut self.prog, 2) as usize
                } else {
                    self.eip + 3
                }
            }

            OP_LESS => {
                let io = get_addr(&mut self.prog, 3);
                self.prog[io] = if get_arg(&mut self.prog, 1) < get_arg(&mut self.prog, 2) {
                    1
                } else {
                    0
                };
                self.eip + 4
            }

            OP_EQ => {
                let io = get_addr(&mut self.prog, 3);
                self.prog[io] = if get_arg(&mut self.prog, 1) == get_arg(&mut self.prog, 2) {
                    1
                } else {
                    0
                };
                self.eip + 4
            }

            OP_RELBASE => {
                self.relbase += get_arg(&mut self.prog, 1);
                self.eip + 2
            }

            OP_HALT => self.eip,
            _ => unreachable!(),
        };
    }

    pub fn run<I>(mut self, input: I) -> Self
    where
        I: IntoIterator<Item = Word>,
    {
        self.run_mut(input);
        self
    }

    pub fn run_mut<I>(&mut self, input: I) -> &mut Self
    where
        I: IntoIterator<Item = Word>,
    {
        self.input.extend(input);
        while self.is_running() && !(self.input.is_empty() && self.expects_input()) {
            self.step();
        }
        self
    }

    pub fn run_with_halt_expect<State, F>(
        mut self,
        initial_input: Option<Word>,
        initial_state: State,
        reducer: F,
    ) -> (IntcodeComputer, State)
    where
        F: Fn(Option<Word>, bool, State) -> (Option<Word>, State, bool),
    {
        self.input.extend(initial_input);
        let mut state = initial_state;
        while self.is_running() {
            self.step();
            let stepout = reducer(self.output.pop_front(), self.expects_input(), state);
            self.input.clear();
            self.input.extend(stepout.0);
            state = stepout.1;
            if stepout.2 {
                break;
            }
        }
        (self, state)
    }

    pub fn is_running(&self) -> bool {
        self.prog[self.eip] != OP_HALT
    }

    pub fn expects_input(&self) -> bool {
        self.prog[self.eip] % 100 == OP_INPUT
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
