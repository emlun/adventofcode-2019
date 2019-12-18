use adventofcode_2019::common::day_input_filename;
use adventofcode_2019::common::get_file_lines;
use adventofcode_2019::common::Solution;
use adventofcode_2019::days;

fn lookup(day: u8) -> Option<(fn(&[String]) -> Solution, (&'static str, &'static str))> {
    match day {
        1 => Some((days::day01::solve, ("3262356", "4890664"))),
        2 => Some((days::day02::solve, ("3101844", "8478"))),
        3 => Some((days::day03::solve, ("232", "6084"))),
        4 => Some((days::day04::solve, ("1063", "686"))),
        5 => Some((days::day05::solve, ("5577461", "7161591"))),
        6 => Some((days::day06::solve, ("268504", "409"))),
        7 => Some((days::day07::solve, ("77500", "22476942"))),
        8 => Some((
            days::day08::solve,
            (
                "1965",
                "
.##..####.#..#...##.#...#
#..#....#.#.#.....#.#...#
#......#..##......#..#.#.
#.##..#...#.#.....#...#..
#..#.#....#.#..#..#...#..
.###.####.#..#..##....#..",
            ),
        )),
        9 => Some((days::day09::solve, ("3839402290", "35734"))),
        10 => Some((days::day10::solve, ("269", "612"))),
        11 => Some((
            days::day11::solve,
            (
                "1932",
                "
 ####  ##  #  # #  #  ##    ## #### ###    
 #    #  # #  # # #  #  #    # #    #  #   
 ###  #    #### ##   #       # ###  #  #   
 #    # ## #  # # #  # ##    # #    ###    
 #    #  # #  # # #  #  # #  # #    # #    
 ####  ### #  # #  #  ###  ##  #### #  #   ",
            ),
        )),
        12 => Some((days::day12::solve, ("5937", "376203951569712"))),
        13 => Some((days::day13::solve, ("200", "9803"))),
        14 => Some((days::day14::solve, ("374457", "3568888"))),
        15 => Some((days::day15::solve, ("336", "360"))),
        16 => Some((days::day16::solve, ("59281788", "96062868"))),
        17 => Some((days::day17::solve, ("5056", "942367"))),
        18 => Some((days::day18::solve, ("4406", ""))),
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
        if let Some((solve, (correct_a, correct_b))) = lookup(day) {
            let input_lines = get_file_lines(&day_input_filename(day))?;
            let (solution_a, solution_b) = solve(&input_lines);
            assert_eq!(
                solution_a.as_str(),
                correct_a,
                "Incorrect solution for day {}a",
                day
            );
            assert_eq!(
                solution_b.as_str(),
                correct_b,
                "Incorrect solution for day {}b",
                day
            );
        }
    }

    Ok(())
}
