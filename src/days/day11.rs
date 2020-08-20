use crate::common::Solution;
use crate::intcode::IntcodeComputer;
use std::collections::HashMap;

type Point = (i64, i64);

fn run(
    mut computer: IntcodeComputer,
    mut white_panels: HashMap<Point, bool>,
) -> HashMap<Point, bool> {
    let mut pos = (0, 0);
    let mut dir = (0, 1);

    while computer.is_running() {
        computer.run_mut(Some(i64::from(*white_panels.get(&pos).unwrap_or(&false))));

        if let Some(out) = computer.output.pop_front() {
            white_panels.insert(pos, out == 1);
        }
        if let Some(out) = computer.output.pop_front() {
            dir = match out {
                0 => (-dir.1, dir.0),
                1 => (dir.1, -dir.0),
                _ => unreachable!(),
            };
            pos = (pos.0 + dir.0, pos.1 + dir.1);
        }
    }

    white_panels
}

fn solve_a(computer: IntcodeComputer) -> usize {
    run(computer, HashMap::new()).len()
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
