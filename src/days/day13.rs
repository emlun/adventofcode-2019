use crate::common::Solution;
use crate::intcode;
use std::collections::HashMap;

type Point = (i64, i64);

const ENABLE_OUTPUT: bool = false;

#[derive(Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

fn run(mut program: Vec<i64>) -> (usize, i64) {
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

    loop {
        let mut output: Option<i64> = None;
        let mut input: Option<i64> = Some(joystick);
        let (eip2, prog2) = intcode::step(eip, program, &mut relbase, &mut output, &mut input);
        eip = eip2;
        program = prog2;
        if let Some(out) = output {
            match state {
                0 => output_x = out,
                1 => output_y = out,
                2 => {
                    if (output_x, output_y) == (-1, 0) {
                        score = out;
                    } else {
                        let output_tile_id = match out {
                            0 => Tile::Empty,
                            1 => Tile::Wall,
                            2 => Tile::Block,
                            3 => {
                                paddle_x = output_x;
                                Tile::Paddle
                            }
                            4 => {
                                ball_x = output_x;
                                Tile::Ball
                            }
                            _ => unreachable!(),
                        };
                        world.insert((output_x, output_y), output_tile_id);
                    }
                }
                _ => unreachable!(),
            };

            state = (state + 1) % 3;

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

    (
        world.values().filter(|tile| **tile == Tile::Block).count(),
        score,
    )
}

fn sign(i: i64) -> i64 {
    if i == 0 {
        0
    } else {
        i / i.abs()
    }
}

fn solve_a(program: Vec<i64>) -> usize {
    run(program).0
}

fn solve_b(mut program: Vec<i64>) -> i64 {
    program[0] = 2;
    run(program).1
}

pub fn solve(lines: &[String]) -> Solution {
    let program = intcode::parse(lines);
    (
        solve_a(program.clone()).to_string(),
        solve_b(program).to_string(),
    )
}
