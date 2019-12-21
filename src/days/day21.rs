use crate::common::Solution;
use crate::intcode::IntcodeComputer;

fn simulate(computer: IntcodeComputer, run: bool, script: &str) -> i64 {
    let walk_input = if run { "RUN\n" } else { "WALK\n" };
    let input = script
        .chars()
        .chain(walk_input.chars())
        .map(|i| i as u8 as i64);
    let output = computer.run(input);

    if output.contains(&(b'D' as i64)) {
        println!("{:?}", output);
        println!(
            "{}",
            output.iter().map(|i| *i as u8 as char).collect::<String>()
        );
        -1
    } else {
        *output.last().unwrap()
    }
}

fn solve_a(computer: IntcodeComputer) -> i64 {
    simulate(
        computer,
        false,
        // !A || (!C && D)
        // (must jump now) || ((jump coming up) && (can land))
        "NOT A J
NOT C T
AND D T
OR T J
",
    )
}

fn solve_b(computer: IntcodeComputer) -> i64 {
    simulate(
        computer,
        true,
        // (!A || !B || !C) && D && (E || H) = !(A && B && C) && D && (E || H)
        // (must jump) && (can land) && ((can walk after landing) || (can jump immediately after landing))
        "OR A J
AND B J
AND C J
NOT J J
AND D J
OR E T
OR H T
AND T J
",
    )
}

pub fn solve(lines: &[String]) -> Solution {
    let computer: IntcodeComputer = lines.into();
    let a_solution = solve_a(computer.clone());
    let b_solution = solve_b(computer);
    (a_solution.to_string(), b_solution.to_string())
}
