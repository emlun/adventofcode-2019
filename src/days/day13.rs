use crate::common::Solution;
use crate::intcode::IntcodeComputer;
use crate::util::sign;
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

struct State {
    world: HashMap<Point, Tile>,
    state: u8,
    output_x: i64,
    output_y: i64,
    score: i64,
    paddle_x: i64,
    ball_x: i64,
}

impl State {
    fn new() -> State {
        State {
            world: HashMap::new(),
            state: 0,
            output_x: 0,
            output_y: 0,
            score: 0,
            paddle_x: 0,
            ball_x: 0,
        }
    }
}

fn print_state(state: &State) {
    let minx = *state.world.keys().map(|(x, _)| x).min().unwrap_or(&0);
    let maxx = *state.world.keys().map(|(x, _)| x).max().unwrap_or(&0);
    let miny = *state.world.keys().map(|(_, y)| y).min().unwrap_or(&0);
    let maxy = *state.world.keys().map(|(_, y)| y).max().unwrap_or(&0);

    println!(
        "\nScore: {}\n{}",
        state.score,
        (miny..=maxy)
            .rev()
            .rev()
            .map(|y| {
                (minx..=maxx)
                    .map(
                        |x| match *state.world.get(&(x, y)).unwrap_or(&Tile::Empty) {
                            Tile::Empty => " ",
                            Tile::Wall => "#",
                            Tile::Block => "B",
                            Tile::Paddle => "-",
                            Tile::Ball => "o",
                        },
                    )
                    .collect::<Vec<&str>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n")
    );
}

fn step_game(output: Option<i64>, expects_input: bool, mut state: State) -> (Option<i64>, State) {
    if let Some(out) = output {
        match state.state {
            0 => state.output_x = out,
            1 => state.output_y = out,
            2 => {
                if (state.output_x, state.output_y) == (-1, 0) {
                    state.score = out;
                } else {
                    let output_tile_id = match out {
                        0 => Tile::Empty,
                        1 => Tile::Wall,
                        2 => Tile::Block,
                        3 => {
                            state.paddle_x = state.output_x;
                            Tile::Paddle
                        }
                        4 => {
                            state.ball_x = state.output_x;
                            Tile::Ball
                        }
                        _ => unreachable!(),
                    };
                    state
                        .world
                        .insert((state.output_x, state.output_y), output_tile_id);
                }
            }
            _ => unreachable!(),
        };

        state.state = (state.state + 1) % 3;

        if ENABLE_OUTPUT {
            print_state(&state);
        }
    }

    let joystick = sign(state.ball_x - state.paddle_x);
    (
        if expects_input {
            println!("{}", joystick);
            Some(joystick)
        } else {
            None
        },
        state,
    )
}

fn solve_a(computer: IntcodeComputer) -> usize {
    computer
        .run_with_expect(Some(0), State::new(), step_game)
        .world
        .values()
        .filter(|tile| **tile == Tile::Block)
        .count()
}

fn solve_b(mut computer: IntcodeComputer) -> i64 {
    computer.prog[0] = 2;
    computer
        .run_with_expect(Some(0), State::new(), step_game)
        .score
}

pub fn solve(lines: &[String]) -> Solution {
    let computer: IntcodeComputer = lines.into();
    (
        solve_a(computer.clone()).to_string(),
        solve_b(computer).to_string(),
    )
}
