use crate::common::Solution;
use crate::intcode::IntcodeComputer;
use std::collections::HashMap;

type Point = (i64, i64);

fn solve_a(computer: IntcodeComputer) -> usize {
    let comp = &computer;
    let points: HashMap<Point, bool> = (0..50)
        .flat_map(|x| (0..50).map(move |y| ((x, y), comp.clone().run(vec![x, y])[0] == 1)))
        .collect();

    println!(
        "{}",
        (0..50)
            .map(|y| (0..50)
                .map(|x| if *points.get(&(x, y)).unwrap() {
                    "#"
                } else {
                    "."
                })
                .collect::<Vec<&str>>()
                .join(""))
            .collect::<Vec<String>>()
            .join("\n")
    );

    points.values().filter(|v| **v).count()
}

pub fn solve(lines: &[String]) -> Solution {
    let computer: IntcodeComputer = lines.into();
    let a_solution = solve_a(computer.clone());
    let b_solution = "";
    (a_solution.to_string(), b_solution.to_string())
}
