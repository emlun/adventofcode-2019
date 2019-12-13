use crate::common::Solution;
use crate::intcode::IntcodeComputer;
use std::collections::HashMap;

type Point = (i64, i64);

fn run(computer: IntcodeComputer, white_panels: HashMap<Point, bool>) -> HashMap<Point, bool> {
    fn get_input(white_panels: &HashMap<Point, bool>, pos: &Point) -> Option<i64> {
        Some(if *white_panels.get(pos).unwrap_or(&false) {
            1
        } else {
            0
        })
    };

    computer
        .run_with(
            get_input(&white_panels, &(0, 0)),
            (white_panels, (0, 0), (0, 1), 0),
            |output: Option<i64>,
             (mut white_panels, mut pos, mut dir, mut state): (
                HashMap<Point, bool>,
                Point,
                Point,
                u8,
            )| {
                if let Some(out) = output {
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

                    state = (state + 1) % 2;
                }

                let input = get_input(&white_panels, &pos);

                (input, (white_panels, pos, dir, state))
            },
        )
        .0
}

fn solve_a(computer: IntcodeComputer) -> usize {
    run(computer, HashMap::new()).values().count()
}

fn solve_b(computer: IntcodeComputer) -> String {
    let mut white_panels = HashMap::new();
    white_panels.insert((0, 0), true);
    let white_panels = run(computer, white_panels);

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
