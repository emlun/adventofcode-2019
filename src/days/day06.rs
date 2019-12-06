use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;

use crate::common::Solution;

fn solve_a(orbits: &HashMap<String, String>) -> u32 {
    let mut num_orbits: HashMap<&str, u32> = HashMap::new();

    fn get_orbit_nums<'obt>(
        child: &'obt str,
        orbits: &'obt HashMap<String, String>,
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

fn solve_b(
    orbits: &HashMap<String, String>,
    adjacent: &HashMap<&str, HashSet<&str>>,
) -> Option<u32> {
    let mut queue: LinkedList<(&str, u32, &str)> = LinkedList::new();
    let pos = orbits.get("YOU").unwrap();
    let target = orbits.get("SAN").unwrap();
    queue.push_back((pos, 0, pos));

    loop {
        if let Some((pos, steps, prev)) = queue.pop_front() {
            if pos == target {
                return Some(steps);
            } else {
                for neighbor in adjacent
                    .get(pos)
                    .iter()
                    .flat_map(|set| set.iter())
                    .filter(|neighbor| **neighbor != prev)
                {
                    queue.push_back((neighbor, steps + 1, pos));
                }
            }
        } else {
            return None;
        }
    }
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

    let adjacent: HashMap<&str, HashSet<&str>> =
        orbits
            .iter()
            .fold(HashMap::new(), |mut result, (child, parent)| {
                let nw = HashSet::new();
                let mut adjacent = result.remove(parent.as_str()).unwrap_or(nw);
                adjacent.insert(child);
                if let Some(parent) = orbits.get(parent) {
                    adjacent.insert(parent);
                }
                result.insert(parent, adjacent);
                result
            });

    (
        solve_a(&orbits).to_string(),
        solve_b(&orbits, &adjacent)
            .map(|b| b.to_string())
            .unwrap_or_else(|| "Impossible".to_string())
            .to_string(),
    )
}
