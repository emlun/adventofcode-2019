use crate::common::Solution;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::convert::TryInto;

type Point = (i64, i64);

const ENABLE_OUTPUT: bool = false;

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

#[derive(Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

fn solve_a(mut program: Vec<i64>) -> String {
    let mut world: HashMap<Point, Tile> = HashMap::new();
    let mut pos: Point = (0, 0);
    let mut dir: Point = (0, 1);
    let mut eip = 0;
    let mut relbase = 0;
    let mut state = 0;
    let mut output_x = 0;
    let mut output_y = 0;

    loop {
        let mut output: Option<i64> = None;
        let mut input: Option<i64> = None;
        let (eip2, prog2) = step(eip, program, &mut relbase, &mut output, &mut input);
        eip = eip2;
        program = prog2;
        if let Some(out) = output {
            match state {
                0 => {
                    output_x = out;
                }
                1 => {
                    output_y = out;
                }
                2 => {
                    let output_tile_id = match out {
                        0 => Tile::Empty,
                        1 => Tile::Wall,
                        2 => Tile::Block,
                        3 => Tile::Paddle,
                        4 => Tile::Ball,
                        _ => unreachable!(),
                    };
                    world.insert((output_x, output_y), output_tile_id);
                }
                _ => unreachable!(),
            };

            state += 1;
            state %= 3;
        }
        if program[eip] == 99 {
            break;
        }
    }

    world
        .values()
        .filter(|tile| **tile == Tile::Block)
        .count()
        .to_string()
}

fn sign(i: i64) -> i64 {
    if i == 0 {
        0
    } else {
        i / i.abs()
    }
}

fn solve_b(mut program: Vec<i64>) -> String {
    let mut world: HashMap<Point, Tile> = HashMap::new();
    let mut eip = 0;
    let mut relbase = 0;
    let mut state = 0;
    let mut output_x = 0;
    let mut output_y = 0;
    let mut joystick = 0;
    let mut score = 0;
    let mut paddle_x = 0;
    let mut ball_x = 0;

    program[0] = 2;

    loop {
        let mut output: Option<i64> = None;
        let mut input: Option<i64> = Some(joystick);
        let (eip2, prog2) = step(eip, program, &mut relbase, &mut output, &mut input);
        eip = eip2;
        program = prog2;
        if let Some(out) = output {
            match state {
                0 => {
                    output_x = out;
                }
                1 => {
                    output_y = out;
                }
                2 => {
                    if (output_x, output_y) == (-1, 0) {
                        score = out;
                    } else {
                        let output_tile_id = match out {
                            0 => Tile::Empty,
                            1 => Tile::Wall,
                            2 => Tile::Block,
                            3 => Tile::Paddle,
                            4 => Tile::Ball,
                            _ => unreachable!(),
                        };
                        if output_tile_id == Tile::Paddle {
                            paddle_x = output_x;
                        }
                        if output_tile_id == Tile::Ball {
                            ball_x = output_x;
                        }
                        world.insert((output_x, output_y), output_tile_id);
                    }
                }
                _ => unreachable!(),
            };

            state += 1;
            state %= 3;

            if ENABLE_OUTPUT {
                let minx = *world.keys().map(|(x, _)| x).min().unwrap_or(&0);
                let maxx = *world.keys().map(|(x, _)| x).max().unwrap_or(&0);
                let miny = *world.keys().map(|(_, y)| y).min().unwrap_or(&0);
                let maxy = *world.keys().map(|(_, y)| y).max().unwrap_or(&0);

                println!(
                    "\n{}\n{}",
                    score,
                    (miny..=maxy)
                        .rev()
                        .map(|y| {
                            (minx..=maxx)
                                .map(|x| match *world.get(&(x, y)).unwrap_or(&Tile::Empty) {
                                    Tile::Empty => " ",
                                    Tile::Wall => "#",
                                    Tile::Block => "B",
                                    Tile::Paddle => "-",
                                    Tile::Ball => "o",
                                    _ => unreachable!(),
                                })
                                .collect::<Vec<&str>>()
                                .join("")
                        })
                        .collect::<Vec<String>>()
                        .join("\n")
                );
            }

            joystick = sign(ball_x - paddle_x);
        }
        if program[eip] == 99 {
            break;
        }
    }

    score.to_string()
}

pub fn solve(lines: &[String]) -> Solution {
    let program: Vec<i64> = lines[0].split(',').map(|s| s.parse().unwrap()).collect();
    (solve_a(program.clone()).to_string(), solve_b(program))
}
