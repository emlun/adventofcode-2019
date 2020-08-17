use std::collections::HashMap;
use std::collections::HashSet;

use crate::common::Solution;

type Point = (i64, i64);

fn norm(p: &&&Point) -> u64 {
    (p.0.abs() as u64) + (p.1.abs() as u64)
}

fn parse_wire(desc: &str) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    let mut pos: Point = (0, 0);
    for step in desc.split(',') {
        let dir = match step.chars().next().unwrap() {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => unreachable!(),
        };
        let len: u32 = step[1..].parse().unwrap();
        for _i in 0..len {
            pos = (pos.0 + dir.0, pos.1 + dir.1);
            points.push(pos);
        }
    }
    points
}

pub fn solve(lines: &[String]) -> Solution {
    let wire1 = parse_wire(&lines[0]);
    let wire2 = parse_wire(&lines[1]);

    let wire1_set: HashSet<&Point> = wire1.iter().collect();
    let wire2_set: HashSet<&Point> = wire2.iter().collect();

    let wire1_inv: HashMap<&Point, usize> = wire1.iter().enumerate().map(|(i, p)| (p, i)).collect();
    let wire2_inv: HashMap<&Point, usize> = wire2.iter().enumerate().map(|(i, p)| (p, i)).collect();

    let intersections: HashSet<&&Point> = wire1_set.intersection(&wire2_set).collect();

    let a_solution: u64 = intersections.iter().map(norm).min().unwrap();
    let b_solution: usize = 2 + intersections
        .iter()
        .map(|p| wire1_inv[*p] + wire2_inv[*p])
        .min()
        .unwrap();
    (a_solution.to_string(), b_solution.to_string())
}
