use std::collections::HashMap;

use crate::common::Solution;

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

    let orbit_nums: u32 = orbits
        .keys()
        .map(|orbitee| get_orbit_nums(&orbitee, &orbits, &mut num_orbits))
        .sum();

    println!("{:?}", orbit_nums);

    (orbit_nums.to_string(), "bar".to_string())
}
