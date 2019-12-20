use adventofcode_2019::common::day_input_filename;
use adventofcode_2019::common::get_file_lines;
use adventofcode_2019::days;

fn test_day(day: u8, correct_a: &str, correct_b: &str) -> Result<(), std::io::Error> {
    let solve = days::get_solver(day).unwrap();
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

    Ok(())
}

#[test]
fn day01() -> Result<(), std::io::Error> {
    test_day(1, "3262356", "4890664")
}

#[test]
fn day02() -> Result<(), std::io::Error> {
    test_day(2, "3101844", "8478")
}

#[test]
fn day03() -> Result<(), std::io::Error> {
    test_day(3, "232", "6084")
}

#[test]
fn day04() -> Result<(), std::io::Error> {
    test_day(4, "1063", "686")
}

#[test]
fn day05() -> Result<(), std::io::Error> {
    test_day(5, "5577461", "7161591")
}

#[test]
fn day06() -> Result<(), std::io::Error> {
    test_day(6, "268504", "409")
}

#[test]
fn day07() -> Result<(), std::io::Error> {
    test_day(7, "77500", "22476942")
}

#[test]
fn day08() -> Result<(), std::io::Error> {
    test_day(
        8,
        "1965",
        "
.##..####.#..#...##.#...#
#..#....#.#.#.....#.#...#
#......#..##......#..#.#.
#.##..#...#.#.....#...#..
#..#.#....#.#..#..#...#..
.###.####.#..#..##....#..",
    )
}

#[test]
fn day09() -> Result<(), std::io::Error> {
    test_day(9, "3839402290", "35734")
}

#[test]
fn day10() -> Result<(), std::io::Error> {
    test_day(10, "269", "612")
}

#[test]
fn day11() -> Result<(), std::io::Error> {
    test_day(
        11,
        "1932",
        "
 ####  ##  #  # #  #  ##    ## #### ###    
 #    #  # #  # # #  #  #    # #    #  #   
 ###  #    #### ##   #       # ###  #  #   
 #    # ## #  # # #  # ##    # #    ###    
 #    #  # #  # # #  #  # #  # #    # #    
 ####  ### #  # #  #  ###  ##  #### #  #   ",
    )
}

#[test]
fn day12() -> Result<(), std::io::Error> {
    test_day(12, "5937", "376203951569712")
}

#[test]
fn day13() -> Result<(), std::io::Error> {
    test_day(13, "200", "9803")
}

#[test]
fn day14() -> Result<(), std::io::Error> {
    test_day(14, "374457", "3568888")
}

#[test]
fn day15() -> Result<(), std::io::Error> {
    test_day(15, "336", "360")
}

#[test]
fn day16() -> Result<(), std::io::Error> {
    test_day(16, "59281788", "96062868")
}

#[test]
fn day17() -> Result<(), std::io::Error> {
    test_day(17, "5056", "942367")
}

#[test]
fn day18() -> Result<(), std::io::Error> {
    test_day(18, "4406", "1964")
}

#[test]
fn day19() -> Result<(), std::io::Error> {
    test_day(19, "217", "6840937")
}
