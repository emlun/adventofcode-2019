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

macro_rules! test_day {
    ($name: ident, $day: literal, $sol_a: literal, $sol_b: literal) => {
        #[test]
        fn $name() -> Result<(), std::io::Error> {
            test_day($day, $sol_a, $sol_b)
        }
    };
}

test_day!(day01, 1, "3262356", "4890664");
test_day!(day02, 2, "3101844", "8478");
test_day!(day03, 3, "232", "6084");
test_day!(day04, 4, "1063", "686");
test_day!(day05, 5, "5577461", "7161591");
test_day!(day06, 6, "268504", "409");
test_day!(day07, 7, "77500", "22476942");
test_day!(
    day08,
    8,
    "1965",
    "
 ##  #### #  #   ## #   #
#  #    # # #     # #   #
#      #  ##      #  # # 
# ##  #   # #     #   #  
#  # #    # #  #  #   #  
 ### #### #  #  ##    #  "
);
test_day!(day09, 9, "3839402290", "35734");
test_day!(day10, 10, "269", "612");
test_day!(
    day11,
    11,
    "1932",
    "
 ####  ##  #  # #  #  ##    ## #### ###    
 #    #  # #  # # #  #  #    # #    #  #   
 ###  #    #### ##   #       # ###  #  #   
 #    # ## #  # # #  # ##    # #    ###    
 #    #  # #  # # #  #  # #  # #    # #    
 ####  ### #  # #  #  ###  ##  #### #  #   "
);
test_day!(day12, 12, "5937", "376203951569712");
test_day!(day13, 13, "200", "9803");
test_day!(day14, 14, "374457", "3568888");
test_day!(day15, 15, "336", "360");
test_day!(day16, 16, "59281788", "96062868");
test_day!(day17, 17, "5056", "942367");
test_day!(day18, 18, "4406", "1964");
test_day!(day19, 19, "217", "6840937");
test_day!(day20, 20, "620", "7366");
test_day!(day21, 21, "19350938", "1142986901");
test_day!(day22, 22, "4775", "37889219674304");
test_day!(day23, 23, "18513", "13286");
test_day!(day24, 24, "17863711", "1937");
test_day!(day25, 25, "35332", "-");
