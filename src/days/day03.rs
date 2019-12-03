use crate::common::Solution;
use std::collections::HashSet;

type Point = (i64, i64);

fn parse_wire(desc: &str) -> HashSet<Point> {
    let mut points: HashSet<Point> = HashSet::new();
    let mut pos: Point = (0, 0);
    for step in desc.split(',') {
        let dir = match step.chars().nth(0).unwrap() {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => unreachable!(),
        };
        let len: u32 = step[1..step.len()].parse().unwrap();
        for _i in 0..len {
            pos = (pos.0 + dir.0, pos.1 + dir.1);
            points.insert(pos);
        }
    }
    points
}

pub fn solve(lines: &[String]) -> Solution {
    let wire1 = parse_wire(&lines[0]);
    let wire2 = parse_wire(&lines[1]);
    let a_solution: i64 = wire1
        .intersection(&wire2)
        .filter(|(x, y)| (x, y) != (&0, &0))
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap();
    let b_solution: i64 = 0;
    (a_solution.to_string(), b_solution.to_string())
}
