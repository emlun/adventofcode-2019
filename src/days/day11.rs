use crate::common::Solution;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::convert::TryInto;

type Point = (i64, i64);

fn step(
    eip: usize,
    mut prog: Vec<i64>,
    relbase: &mut i64,
    output: &mut Option<i64>,
    input: &mut Option<i64>,
) -> (usize, Vec<i64>) {
    let instruction = prog[eip];
    let opcode = instruction % 100;

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
            2 => usize::try_from(*relbase + prog[eip + offset]).unwrap(),
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

    let eip = match opcode {
        1 => {
            let args = get_args(&mut prog, 2);
            let io = get_addr(&mut prog, 3);
            prog[io] = args[0] + args[1];
            eip + 4
        }
        2 => {
            let args = get_args(&mut prog, 2);
            let io = get_addr(&mut prog, 3);
            prog[io] = args[0] * args[1];
            eip + 4
        }
        3 => {
            let io = get_addr(&mut prog, 1);
            if let Some(i) = input.take() {
                prog[io] = i;
                eip + 2
            } else {
                eip
            }
        }
        4 => {
            let args = get_args(&mut prog, 1);
            if output.is_none() {
                output.replace(args[0]);
                eip + 2
            } else {
                eip
            }
        }
        5 => {
            let args = get_args(&mut prog, 2);
            if args[0] != 0 {
                args[1] as usize
            } else {
                eip + 3
            }
        }
        6 => {
            let args = get_args(&mut prog, 2);
            if args[0] == 0 {
                args[1] as usize
            } else {
                eip + 3
            }
        }
        7 => {
            let args = get_args(&mut prog, 2);
            let io = get_addr(&mut prog, 3);
            prog[io] = if args[0] < args[1] { 1 } else { 0 };
            eip + 4
        }
        8 => {
            let args = get_args(&mut prog, 2);
            let io = get_addr(&mut prog, 3);
            prog[io] = if args[0] == args[1] { 1 } else { 0 };
            eip + 4
        }
        9 => {
            let args = get_args(&mut prog, 1);
            *relbase += args[0];
            eip + 2
        }
        99 => eip,
        _ => unreachable!(),
    };
    (eip, prog)
}

fn run(mut program: Vec<i64>, input: i64) -> Vec<i64> {
    let mut output: Vec<i64> = Vec::new();
    let mut eip = 0;
    let mut input_iter = vec![input].into_iter();
    let mut next_input = None;
    let mut relbase = 0;
    while program[eip] != 99 {
        if next_input.is_none() {
            next_input = input_iter.next();
        }
        let mut outpt = None;
        let out = step(eip, program, &mut relbase, &mut outpt, &mut next_input);
        if let Some(out) = outpt {
            output.push(out);
        }
        eip = out.0;
        program = out.1;
    }
    output
}

fn solve_a(mut program: Vec<i64>) -> usize {
    let mut white_panels: HashMap<Point, bool> = HashMap::new();
    let mut pos: Point = (0, 0);
    let mut dir: Point = (0, 1);
    let mut eip = 0;
    let mut relbase = 0;
    let mut state = 0;

    loop {
        let mut output: Option<i64> = None;
        let mut input: Option<i64> = Some(if *white_panels.get(&pos).unwrap_or(&false) {
            1
        } else {
            0
        });
        let (eip2, prog2) = step(eip, program, &mut relbase, &mut output, &mut input);
        eip = eip2;
        program = prog2;
        // dbg!(eip, &white_panels, input, output);
        if let Some(out) = output {
            match state {
                0 => {
                    white_panels.insert(pos, out == 1);
                }
                1 => {
                    dir = match out {
                        0 => (dir.0 * 0 + dir.1 * -1, dir.0 * 1 + dir.1 * 0),
                        1 => (dir.0 * 0 + dir.1 * 1, dir.0 * -1 + dir.1 * 0),
                        _ => unreachable!(),
                    };
                    pos = (pos.0 + dir.0, pos.1 + dir.1);
                }
                _ => unreachable!(),
            };

            state += 1;
            state %= 2;
        }
        if program[eip] == 99 {
            break;
        }
    }
    white_panels.values().count()
}

fn solve_b(program: Vec<i64>) -> i64 {
    0
}

pub fn solve(lines: &[String]) -> Solution {
    let program: Vec<i64> = lines[0].split(',').map(|s| s.parse().unwrap()).collect();
    (
        solve_a(program.clone()).to_string(),
        solve_b(program).to_string(),
    )
}
