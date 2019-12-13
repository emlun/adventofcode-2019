use std::convert::TryFrom;
use std::convert::TryInto;

pub struct IntcodeComputer {
    eip: usize,
    prog: Vec<i64>,
    relbase: i64,
    output: Option<i64>,
    input: Option<i64>,
}

impl IntcodeComputer {
    pub fn step(&mut self) -> &mut Option<i64> {
        let instruction = self.prog[self.eip];
        let opcode = instruction % 100;
        let eip = self.eip;
        let relbase = self.relbase;

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
                if let Some(i) = self.input.take() {
                    self.prog[io] = i;
                    self.eip + 2
                } else {
                    self.eip
                }
            }
            4 => {
                let args = get_args(&mut self.prog, 1);
                if self.output.is_none() {
                    self.output.replace(args[0]);
                    self.eip + 2
                } else {
                    self.eip
                }
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
        &mut self.output
    }

    pub fn run(mut self) -> (Vec<i64>, Option<i64>) {
        while self.eip != 99 {
            self.step();
        }

        (self.prog, self.output)
    }
}
