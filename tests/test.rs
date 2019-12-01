use adventofcode_2019::common::day_input_filename;
use adventofcode_2019::common::get_file_lines;
use adventofcode_2019::common::Solution;
use adventofcode_2019::days;

fn lookup(day: u8) -> Option<(fn(&Vec<String>) -> Solution, (&'static str, &'static str))> {
    match day {
        1 => Some((days::day01::solve, ("3262356", "4890664"))),
        // 2 => Some((days::day02::solve, ("", ""))),
        // 3 => Some((days::day03::solve, ("", ""))),
        // 4 => Some((days::day04::solve, ("", ""))),
        // 5 => Some((days::day05::solve, ("", ""))),
        // 6 => Some((days::day06::solve, ("", ""))),
        // 7 => Some((days::day07::solve, ("", ""))),
        // 8 => Some((days::day08::solve, ("", ""))),
        // 9 => Some((days::day09::solve, ("", ""))),
        // 10 => Some((days::day10::solve, ("", ""))),
        // 11 => Some((days::day11::solve, ("", ""))),
        // 12 => Some((days::day12::solve, ("", ""))),
        // 13 => Some((days::day13::solve, ("", ""))),
        // 14 => Some((days::day14::solve, ("", ""))),
        // 15 => Some((days::day15::solve, ("", ""))),
        // 16 => Some((days::day16::solve, ("", ""))),
        // 17 => Some((days::day17::solve, ("", ""))),
        // 18 => Some((days::day18::solve, ("", ""))),
        // 19 => Some((days::day19::solve, ("", ""))),
        // 20 => Some((days::day20::solve, ("", ""))),
        // 21 => Some((days::day21::solve, ("", ""))),
        // 22 => Some((days::day22::solve, ("", ""))),
        // 23 => Some((days::day23::solve, ("", ""))),
        // 24 => Some((days::day24::solve, ("", ""))),
        // 25 => Some((days::day25::solve, ("", ""))),
        day => {
            if day >= 1 && day <= 25 {
                None
            } else {
                panic!(format!("Unknown day: {}", day))
            }
        }
    }
}

#[test]
fn all_days_give_correct_output() -> Result<(), std::io::Error> {
    for day in 1..=25 {
        if let Some((solve, correct_solution)) = lookup(day) {
            let input_lines = get_file_lines(&day_input_filename(day))?;
            let solution = solve(&input_lines);
            assert_eq!(
                (solution.0.as_str(), solution.1.as_str()),
                correct_solution,
                "Incorrect solution for day {}",
                day
            );
        }
    }

    Ok(())
}
