use std::collections::HashMap;

use crate::common::Solution;

fn solve_a(orbits: &HashMap<&str, &str>) -> u32 {
    let mut num_orbits: HashMap<&str, u32> = HashMap::new();

    fn get_orbit_nums<'obt>(
        child: &'obt str,
        orbits: &'obt HashMap<&str, &str>,
        num_orbits: &mut HashMap<&'obt str, u32>,
    ) -> u32 {
        num_orbits.get(child).copied().unwrap_or_else(|| {
            let result = orbits
                .get(child)
                .map(|parent| 1 + get_orbit_nums(parent, orbits, num_orbits))
                .unwrap_or(0);
            num_orbits.insert(child, result);
            result
        })
    };

    orbits
        .keys()
        .map(|child| get_orbit_nums(&child, &orbits, &mut num_orbits))
        .sum()
}

fn solve_b(orbits: &HashMap<&str, &str>) -> Option<usize> {
    let mut steps_from_you: HashMap<&str, usize> = HashMap::new();
    let mut steps_from_san: HashMap<&str, usize> = HashMap::new();

    let mut seq_from_you = std::iter::successors(Some(&"YOU"), |pos| orbits.get(*pos))
        .skip(1)
        .enumerate();
    let mut seq_from_san = std::iter::successors(Some(&"SAN"), |pos| orbits.get(*pos))
        .skip(1)
        .enumerate();

    loop {
        let next_you = seq_from_you.next();
        if let Some((steps1, pos)) = next_you {
            if let Some(steps2) = steps_from_san.get(pos) {
                return Some(steps1 + steps2);
            }
            steps_from_you.insert(pos, steps1);
        }

        let next_san = seq_from_san.next();
        if let Some((steps1, pos)) = next_san {
            if let Some(steps2) = steps_from_you.get(pos) {
                return Some(steps1 + steps2);
            }
            steps_from_san.insert(pos, steps1);
        }

        if (next_you, next_san) == (None, None) {
            return None;
        }
    }
}

pub fn solve(lines: &[String]) -> Solution {
    let orbits: HashMap<&str, &str> = lines
        .iter()
        .map(|line| {
            let mut splt = line.split(')');
            let a = splt.next().unwrap();
            let b = splt.next().unwrap();
            (b, a)
        })
        .collect();

    (
        solve_a(&orbits).to_string(),
        solve_b(&orbits)
            .map(|b| b.to_string())
            .unwrap_or_else(|| "Impossible".to_string()),
    )
}
