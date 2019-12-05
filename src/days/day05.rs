use crate::common::Solution;

fn step(
    eip: usize,
    mut prog: Vec<i32>,
    output: &mut Vec<i32>,
    input: &mut dyn Iterator<Item = i32>,
) -> (usize, Vec<i32>) {
    let instruction = prog[eip];
    let opcode = instruction % 100;

    let get_args = |num: usize| {
        let mut result = Vec::new();

        let mut parmode_pow = 100;
        for i in 1..=num {
            let iarg = prog[eip + i];
            let arg = if (instruction / parmode_pow) % 10 == 0 {
                prog[iarg as usize]
            } else {
                iarg
            };
            result.push(arg);
            parmode_pow *= 10;
        }

        result
    };

    let eip = match opcode {
        1 => {
            let args = get_args(2);
            let io = prog[eip + 3] as usize;
            prog[io] = args[0] + args[1];
            eip + 4
        }
        2 => {
            let args = get_args(2);
            let io = prog[eip + 3] as usize;
            prog[io] = args[0] * args[1];
            eip + 4
        }
        3 => {
            let io = prog[eip + 1] as usize;
            prog[io] = input.next().unwrap();
            eip + 2
        }
        4 => {
            let args = get_args(1);
            output.push(args[0]);
            eip + 2
        }
        5 => {
            let args = get_args(2);
            if args[0] != 0 {
                args[1] as usize
            } else {
                eip + 3
            }
        }
        6 => {
            let args = get_args(2);
            if args[0] == 0 {
                args[1] as usize
            } else {
                eip + 3
            }
        }
        7 => {
            let args = get_args(2);
            let io = prog[eip + 3] as usize;
            prog[io] = if args[0] < args[1] { 1 } else { 0 };
            eip + 4
        }
        8 => {
            let args = get_args(2);
            let io = prog[eip + 3] as usize;
            prog[io] = if args[0] == args[1] { 1 } else { 0 };
            eip + 4
        }
        _ => unreachable!(),
    };
    (eip, prog)
}

fn run(mut program: Vec<i32>, input: i32) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    let mut eip = 0;
    while program[eip] != 99 {
        let out = step(eip, program, &mut output, &mut vec![input].into_iter());
        eip = out.0;
        program = out.1;
    }
    output
}

fn solve_a(program: Vec<i32>) -> Option<i32> {
    let output = run(program, 1);
    if output
        .iter()
        .enumerate()
        .all(|(i, o)| *o == 0 || i == output.len() - 1)
    {
        Some(*output.last().unwrap())
    } else {
        None
    }
}

fn solve_b(program: Vec<i32>) -> i32 {
    *run(program, 5).last().unwrap()
}

pub fn solve(lines: &[String]) -> Solution {
    let program: Vec<i32> = lines[0].split(',').map(|s| s.parse().unwrap()).collect();
    (
        solve_a(program.clone())
            .map(|i| i.to_string())
            .unwrap_or_else(|| "Failure!".to_string()),
        solve_b(program).to_string(),
    )
}
