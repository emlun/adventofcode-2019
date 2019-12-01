use crate::common::Solution;

fn base_fuel_mass(mass: i64) -> i64 {
    mass / 3 - 2
}

fn full_fuel_mass(mass: i64) -> i64 {
    let m = base_fuel_mass(mass);
    if m > 0 {
        m + full_fuel_mass(m)
    } else {
        0
    }
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let module_masses: Vec<i64> = lines
        .iter()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    let a_solution: i64 = module_masses.iter().map(|i| base_fuel_mass(*i)).sum();
    let b_solution: i64 = module_masses.iter().map(|i| full_fuel_mass(*i)).sum();
    (a_solution.to_string(), b_solution.to_string())
}
