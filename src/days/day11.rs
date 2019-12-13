use crate::common::Solution;
use crate::intcode;
use std::collections::HashMap;

type Point = (i64, i64);

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
        let (eip2, prog2) = intcode::step(eip, program, &mut relbase, &mut output, &mut input);
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
        if program[eip] == 99 {
            break;
        }
    }
    white_panels.values().count()
}

fn solve_b(mut program: Vec<i64>) -> String {
    let mut white_panels: HashMap<Point, bool> = HashMap::new();
    let mut pos: Point = (0, 0);
    let mut dir: Point = (0, 1);
    let mut eip = 0;
    let mut relbase = 0;
    let mut state = 0;
    white_panels.insert(pos, true);

    loop {
        let mut output: Option<i64> = None;
        let mut input: Option<i64> = Some(if *white_panels.get(&pos).unwrap_or(&false) {
            1
        } else {
            0
        });
        let (eip2, prog2) = intcode::step(eip, program, &mut relbase, &mut output, &mut input);
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
        if program[eip] == 99 {
            break;
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
    let program = intcode::parse(lines);
    (solve_a(program.clone()).to_string(), solve_b(program))
}
