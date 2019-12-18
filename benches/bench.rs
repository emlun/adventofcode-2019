#![feature(test)]
extern crate test;
use adventofcode_2019::common::day_input_filename;
use adventofcode_2019::common::get_file_lines;
use adventofcode_2019::common::Solution;
use adventofcode_2019::days;
use test::Bencher;

fn run_bench(day: u8, b: &mut Bencher) {
    let input_lines = get_file_lines(&day_input_filename(day)).unwrap();
    let solve = days::get_solver(day).unwrap();
    b.iter(|| solve(&input_lines));
}

#[bench]
fn day01(b: &mut Bencher) {
    run_bench(1, b);
}

#[bench]
fn day02(b: &mut Bencher) {
    run_bench(2, b);
}

#[bench]
fn day03(b: &mut Bencher) {
    run_bench(3, b);
}

#[bench]
fn day04(b: &mut Bencher) {
    run_bench(4, b);
}

#[bench]
fn day05(b: &mut Bencher) {
    run_bench(5, b);
}

#[bench]
fn day06(b: &mut Bencher) {
    run_bench(6, b);
}

#[bench]
fn day07(b: &mut Bencher) {
    run_bench(7, b);
}

#[bench]
fn day08(b: &mut Bencher) {
    run_bench(8, b);
}

#[bench]
fn day09(b: &mut Bencher) {
    run_bench(9, b);
}

#[bench]
fn day10(b: &mut Bencher) {
    run_bench(10, b);
}

#[bench]
fn day11(b: &mut Bencher) {
    run_bench(11, b);
}

#[bench]
fn day12(b: &mut Bencher) {
    run_bench(12, b);
}

#[bench]
fn day13(b: &mut Bencher) {
    run_bench(13, b);
}

#[bench]
fn day14(b: &mut Bencher) {
    run_bench(14, b);
}

#[bench]
fn day15(b: &mut Bencher) {
    run_bench(15, b);
}

#[bench]
fn day16(b: &mut Bencher) {
    run_bench(16, b);
}

#[bench]
fn day17(b: &mut Bencher) {
    run_bench(17, b);
}

#[bench]
fn day18(b: &mut Bencher) {
    run_bench(18, b);
}

#[bench]
fn days_all(b: &mut Bencher) {
    let solvers_and_inputs: Vec<(fn(&[String]) -> Solution, Vec<String>)> = days::all_numbers()
        .into_iter()
        .map(|day| {
            (
                days::get_solver(day).unwrap(),
                get_file_lines(&day_input_filename(day)).unwrap(),
            )
        })
        .collect();

    b.iter(|| {
        solvers_and_inputs
            .iter()
            .map(|(solver, input)| solver(&input))
            .collect::<Vec<Solution>>()
    })
}
