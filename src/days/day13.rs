use crate::common::Solution;
use crate::intcode::IntcodeComputer;
use crate::util::sign;
use std::collections::HashSet;

fn solve_a(mut computer: IntcodeComputer) -> usize {
    computer.run_mut(None);

    let mut blocks = HashSet::new();

    while !computer.output.is_empty() {
        let pos = (
            computer.output.pop_front().unwrap(),
            computer.output.pop_front().unwrap(),
        );

        if pos == (-1, 0) {
            computer.output.pop_front();
        } else if computer.output.pop_front().unwrap() == 2 {
            blocks.insert(pos);
        }
    }

    blocks.len()
}

fn solve_b(mut computer: IntcodeComputer) -> i64 {
    computer.prog[0] = 2;

    let mut ball_x = 0;
    let mut paddle_x = 0;
    let mut score = 0;

    while computer.is_running() {
        let joystick = sign(ball_x - paddle_x);
        computer.run_mut(Some(joystick));

        while !computer.output.is_empty() {
            let (x, y) = (
                computer.output.pop_front().unwrap(),
                computer.output.pop_front().unwrap(),
            );

            if (x, y) == (-1, 0) {
                score = computer.output.pop_front().unwrap();
            } else {
                match computer.output.pop_front().unwrap() {
                    3 => {
                        paddle_x = x;
                    }
                    4 => {
                        ball_x = x;
                    }
                    _ => {}
                }
            }
        }
    }

    score
}

pub fn solve(lines: &[String]) -> Solution {
    let computer: IntcodeComputer = lines.into();
    (
        solve_a(computer.clone()).to_string(),
        solve_b(computer).to_string(),
    )
}

mod pretty {
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
        score: i64,
        paddle_x: i64,
        ball_x: i64,
    }

    impl State {
        fn new() -> State {
            State {
                world: HashMap::new(),
                score: 0,
                paddle_x: 0,
                ball_x: 0,
            }
        }
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    fn play_game(mut computer: IntcodeComputer) -> State {
        let mut state: State = State::new();
        computer.run_mut(Some(0));

        while computer.is_running() {
            let joystick = sign(state.ball_x - state.paddle_x);
            computer.run_mut(Some(joystick));

            while !computer.output.is_empty() {
                let (x, y) = (
                    computer.output.pop_front().unwrap(),
                    computer.output.pop_front().unwrap(),
                );

                if (x, y) == (-1, 0) {
                    state.score = computer.output.pop_front().unwrap();
                } else {
                    let output_tile_id = match computer.output.pop_front().unwrap() {
                        0 => Tile::Empty,
                        1 => Tile::Wall,
                        2 => Tile::Block,
                        3 => {
                            state.paddle_x = x;
                            Tile::Paddle
                        }
                        4 => {
                            state.ball_x = x;
                            Tile::Ball
                        }
                        _ => unreachable!(),
                    };
                    state.world.insert((x, y), output_tile_id);
                }
            }

            if ENABLE_OUTPUT {
                print_state(&state);
            }
        }

        state
    }
}
