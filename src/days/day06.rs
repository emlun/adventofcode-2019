use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;

use crate::common::Solution;

fn solve_a(orbits: &HashMap<String, String>) -> u32 {
    let mut num_orbits: HashMap<String, u32> = HashMap::new();

    fn get_orbit_nums<'obt>(
        orbiter: &'obt String,
        orbits: &'obt HashMap<String, String>,
        num_orbits: &'obt mut HashMap<String, u32>,
    ) -> u32 {
        if let Some(num) = num_orbits.get(orbiter) {
            *num
        } else {
            let result = if let Some(orbitee) = orbits.get(orbiter) {
                1 + get_orbit_nums(orbitee, orbits, num_orbits)
            } else {
                0
            };
            num_orbits.insert(orbiter.clone(), result);
            result
        }
    };

    orbits
        .keys()
        .map(|orbitee| get_orbit_nums(&orbitee, &orbits, &mut num_orbits))
        .sum()
}

fn solve_b(
    pos: &String,
    target: &String,
    orbits: &HashMap<String, String>,
    orbiters: &HashMap<&String, HashSet<&String>>,
) -> Option<u32> {
    let mut queue: LinkedList<(&String, u32, &String)> = LinkedList::new();

    queue.push_back((orbits.get(pos).unwrap(), 1, pos));
    for orbiter in orbiters.get(pos).iter().map(|set| set.iter()).flatten() {
        queue.push_back((orbiter, 1, pos));
    }

    while !queue.is_empty() {
        let (pos, steps, prev) = queue.pop_front().unwrap();
        if pos == target {
            return Some(steps);
        } else {
            if let Some(orbitee) = orbits.get(pos) {
                if orbitee != prev {
                    queue.push_back((orbitee, steps + 1, pos));
                }
            }
            for orbiter in orbiters.get(pos).iter().map(|set| set.iter()).flatten() {
                if *orbiter != prev {
                    queue.push_back((orbiter, steps + 1, pos));
                }
            }
        }
    }
    None
}

pub fn solve(lines: &[String]) -> Solution {
    let orbits: HashMap<String, String> = lines
        .iter()
        .map(|line| {
            let mut splt = line.split(')');
            let a = splt.next().unwrap();
            let b = splt.next().unwrap();
            (b.to_string(), a.to_string())
        })
        .collect();

    let orbiters: HashMap<&String, HashSet<&String>> =
        orbits
            .iter()
            .fold(HashMap::new(), |mut result, (orbiter, orbitee)| {
                let nw = HashSet::new();
                let mut orbiters: HashSet<&String> = result.remove(orbitee).unwrap_or(nw);
                orbiters.insert(orbiter);
                result.insert(orbitee, orbiters);
                result
            });

    (
        solve_a(&orbits).to_string(),
        solve_b(
            orbits.get("YOU").unwrap(),
            orbits.get("SAN").unwrap(),
            &orbits,
            &orbiters,
        )
        .map(|b| b.to_string())
        .unwrap_or("Impossible".to_string())
        .to_string(),
    )
}
