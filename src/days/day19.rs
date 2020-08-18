use crate::common::Solution;
use crate::intcode::IntcodeComputer;

type Point = (usize, usize);

fn check(computer: &IntcodeComputer, (x, y): Point) -> bool {
    computer.clone().run(vec![x as i64, y as i64]).output[0] == 1
}

fn compute_minx(computer: &IntcodeComputer, prev_minx: usize, y: usize) -> usize {
    for x in prev_minx..(prev_minx + 10) {
        if check(computer, (x, y)) {
            return x;
        }
    }
    0
}

fn compute_maxx(computer: &IntcodeComputer, prev_maxx: usize, y: usize) -> usize {
    for x in prev_maxx.. {
        if !check(computer, (x, y)) {
            return x;
        }
    }
    unreachable!();
}

fn reduce_maxx(computer: &IntcodeComputer, maxx: usize, y: usize) -> usize {
    let mut result = maxx;
    for x in (0..maxx).rev() {
        if check(computer, (x, y)) {
            break;
        } else {
            result = x;
        }
    }
    result
}

fn solve_b(computer: IntcodeComputer) -> (usize, usize) {
    const DIM_WANTED: usize = 100;

    let mut maxx = 0;
    let mut minx = 0;

    let mut a_solution = 0;

    for y in 0..50 {
        minx = compute_minx(&computer, minx, y);
        maxx = reduce_maxx(&computer, compute_maxx(&computer, maxx, y), y);
        if maxx < minx {
            maxx = compute_maxx(&computer, minx + 1, y);
        }
        a_solution += std::cmp::min(50, maxx) - minx;
    }

    let k1: f64 = (maxx as f64) / 49_f64;
    let k2: f64 = (minx as f64) / 49_f64;

    let y_guess: usize = ((2_f64 * DIM_WANTED as f64) / (k1 - k2)).round() as usize;

    let mut y_min = 50;
    let mut y_max = y_guess * 2;
    let mut x_max = maxx;

    while y_max > y_min {
        let y = (y_max + y_min) / 2;
        let maxx_guess = (y as f64 * k1).round() as usize;
        let maxx = reduce_maxx(&computer, compute_maxx(&computer, maxx_guess, y), y);
        let x = maxx - DIM_WANTED;
        if check(&computer, (x, y + DIM_WANTED - 1)) {
            y_max = y;
            x_max = x;
        } else {
            y_min = y + 1;
        }
    }
    (a_solution, x_max * 10000 + y_max)
}

pub fn solve(lines: &[String]) -> Solution {
    let computer: IntcodeComputer = lines.into();
    let (a_solution, b_solution) = solve_b(computer);
    (a_solution.to_string(), b_solution.to_string())
}
