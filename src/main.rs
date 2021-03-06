use std::path::Path;

use clap::App;
use clap::Arg;

mod common;
mod crate_info;
mod days;
mod intcode;
mod util;

use crate::common::day_input_filename;
use crate::common::get_file_lines;
use crate::crate_info::crate_author;
use crate::crate_info::crate_description;
use crate::crate_info::crate_name;
use crate::crate_info::crate_version;
use crate::intcode::IntcodeComputer;

fn main() -> Result<(), std::io::Error> {
    let cli = App::new(crate_name())
        .version(crate_version())
        .about(crate_description())
        .author(crate_author())
        .arg(
            Arg::with_name("day")
                .takes_value(true)
                .help(r#"Day number (1 - 25) to run, or "intcode" to parse and run an Intcode program. If omitted, all days are run."#)
        )
        .arg(
            Arg::with_name("input-file")
                .takes_value(true)
                .help(r#"When <day> is not "intcode": Path to a file containing input for the chosen day. Use "-" for standard input; omit to use "./inputs/day<day>.in". When <day> is "intcode": Path to a file containing the Intcode program to run. Use "-" or omit for standard input. Input is read from the first line of standard input in the same format as an Intcode program; when program is also read from standard input, the program is read from the first line and the input from the second."#)
        );

    let matches = cli.get_matches();

    if let Some(day) = matches.value_of("day") {
        if day == "intcode" {
            run_intcode(matches.value_of("input-file"))
        } else {
            run_day(
                day.parse::<u8>()
                    .unwrap_or_else(|_| panic!(format!("Invalid day number: {}", day))),
                matches.value_of("input-file").map(Path::new),
            )
        }
    } else {
        run_all_days()
    }
}

fn run_day(day: u8, input_path: Option<&Path>) -> Result<(), std::io::Error> {
    println!();
    println!("=== Day {: >2} ===", day);

    let day_func = days::get_solver(day).unwrap_or_else(|| panic!(format!("Unknown day: {}", day)));
    let lines = input_path
        .map(get_file_lines)
        .unwrap_or_else(|| get_file_lines(&day_input_filename(day)))?;
    let solution = day_func(&lines);

    println!("A: {}", solution.0);
    println!("B: {}", solution.1);

    Ok(())
}

fn run_all_days() -> Result<(), std::io::Error> {
    for day in days::all_numbers() {
        run_day(day, None)?
    }
    Ok(())
}

fn run_intcode(input_file: Option<&str>) -> Result<(), std::io::Error> {
    let lines = get_file_lines(Path::new(input_file.unwrap_or("-")))?;
    let inputs: Vec<i64> = if input_file == None || input_file == Some("-") {
        lines
            .get(1)
            .map(|line| {
                line.split(',')
                    .map(|s| s.parse())
                    .collect::<Result<Vec<i64>, std::num::ParseIntError>>()
                    .expect(
                        "Invalid integer in intcode program input (second line of standard input)",
                    )
            })
            .unwrap_or_default()
    } else {
        get_file_lines(Path::new("-"))?
            .get(0)
            .map(|line| {
                line.split(',')
                    .map(|s| s.parse())
                    .collect::<Result<Vec<i64>, std::num::ParseIntError>>()
                    .expect("Invalid integer in intcode program input (standard input)")
            })
            .unwrap_or_default()
    };
    let computer: IntcodeComputer = lines[0..1].into();
    let computer = computer.run(inputs);
    if computer.expects_input() {
        Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "Not enough input",
        ))
    } else {
        println!(
            "{}",
            computer
                .output
                .into_iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
        Ok(())
    }
}
