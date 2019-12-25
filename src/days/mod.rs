pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

pub fn all_numbers() -> Vec<u8> {
    (1..=25).filter(|&day| get_solver(day).is_some()).collect()
}

pub fn get_solver(day: u8) -> Option<fn(&[String]) -> crate::common::Solution> {
    match day {
        1 => Some(day01::solve),
        2 => Some(day02::solve),
        3 => Some(day03::solve),
        4 => Some(day04::solve),
        5 => Some(day05::solve),
        6 => Some(day06::solve),
        7 => Some(day07::solve),
        8 => Some(day08::solve),
        9 => Some(day09::solve),
        10 => Some(day10::solve),
        11 => Some(day11::solve),
        12 => Some(day12::solve),
        13 => Some(day13::solve),
        14 => Some(day14::solve),
        15 => Some(day15::solve),
        16 => Some(day16::solve),
        17 => Some(day17::solve),
        18 => Some(day18::solve),
        19 => Some(day19::solve),
        20 => Some(day20::solve),
        21 => Some(day21::solve),
        22 => Some(day22::solve),
        23 => Some(day23::solve),
        24 => Some(day24::solve),
        25 => Some(day25::solve),
        _ => None,
    }
}
