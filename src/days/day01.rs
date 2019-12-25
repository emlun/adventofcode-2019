use crate::common::Solution;

fn base_fuel_mass(mass: i32) -> i32 {
    mass / 3 - 2
}

fn full_fuel_mass(mass: i32) -> i32 {
    let m = base_fuel_mass(mass);
    if m > 0 {
        m + full_fuel_mass(m)
    } else {
        0
    }
}

pub fn solve(lines: &[String]) -> Solution {
    let (a_solution, b_solution) =
        lines
            .iter()
            .map(|line| line.parse::<i32>().unwrap())
            .fold((0, 0), |(suma, sumb), i| {
                let base = base_fuel_mass(i);
                (suma + base, sumb + base + full_fuel_mass(base))
            });
    (a_solution.to_string(), b_solution.to_string())
}
