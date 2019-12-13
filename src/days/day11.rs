use crate::common::Solution;
use crate::intcode::IntcodeComputer;
use std::collections::HashMap;

type Point = (i64, i64);

fn solve_a(mut computer: IntcodeComputer) -> usize {
    let mut white_panels: HashMap<Point, bool> = HashMap::new();
    let mut pos: Point = (0, 0);
    let mut dir: Point = (0, 1);
    let mut state = 0;

    while computer.is_running() {
        let mut input: Option<i64> = Some(if *white_panels.get(&pos).unwrap_or(&false) {
            1
        } else {
            0
        });
        if let Some(out) = computer.step(&mut input).take() {
            match state {
                0 => {
                    white_panels.insert(pos, out == 1);
                }
                1 => {
                    dir = match out {
                        0 => (-dir.1, dir.0),
                        1 => (dir.1, -dir.0),
                        _ => unreachable!(),
                    };
                    pos = (pos.0 + dir.0, pos.1 + dir.1);
                }
                _ => unreachable!(),
            };

            state += 1;
            state %= 2;
        }
    }
    white_panels.values().count()
}

fn solve_b(mut computer: IntcodeComputer) -> String {
    let mut white_panels: HashMap<Point, bool> = HashMap::new();
    let mut pos: Point = (0, 0);
    let mut dir: Point = (0, 1);
    let mut state = 0;
    white_panels.insert(pos, true);

    while computer.is_running() {
        let mut input: Option<i64> = Some(if *white_panels.get(&pos).unwrap_or(&false) {
            1
        } else {
            0
        });
        if let Some(out) = computer.step(&mut input).take() {
            match state {
                0 => {
                    white_panels.insert(pos, out == 1);
                }
                1 => {
                    dir = match out {
                        0 => (-dir.1, dir.0),
                        1 => (dir.1, -dir.0),
                        _ => unreachable!(),
                    };
                    pos = (pos.0 + dir.0, pos.1 + dir.1);
                }
                _ => unreachable!(),
            };

            state += 1;
            state %= 2;
        }
    }

    let minx = *white_panels.keys().map(|(x, _)| x).min().unwrap();
    let maxx = *white_panels.keys().map(|(x, _)| x).max().unwrap();
    let miny = *white_panels.keys().map(|(_, y)| y).min().unwrap();
    let maxy = *white_panels.keys().map(|(_, y)| y).max().unwrap();

    format!(
        "\n{}",
        (miny..=maxy)
            .rev()
            .map(|y| {
                (minx..=maxx)
                    .map(|x| {
                        if *white_panels.get(&(x, y)).unwrap_or(&false) {
                            "#"
                        } else {
                            " "
                        }
                    })
                    .collect::<Vec<&str>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n")
    )
}

pub fn solve(lines: &[String]) -> Solution {
    let computer: IntcodeComputer = lines.into();
    (solve_a(computer.clone()).to_string(), solve_b(computer))
}
