use crate::common::Solution;
use std::collections::HashSet;

type Point = (i64, i64, i64);

struct Moon {
    pos: Point,
    vel: Point,
}

fn add(p1: &Point, p2: &Point) -> Point {
    (p1.0 + p2.0, p1.1 + p2.1, p1.2 + p2.2)
}

fn sub(p1: &Point, p2: &Point) -> Point {
    (p1.0 - p2.0, p1.1 - p2.1, p1.2 - p2.2)
}

fn sign(i: i64) -> i64 {
    if i == 0 {
        0
    } else {
        i / i.abs()
    }
}

fn gravity(on: &Moon, from: &Moon) -> Point {
    let posdiff = sub(&from.pos, &on.pos);
    (sign(posdiff.0), sign(posdiff.1), sign(posdiff.2))
}

fn energy(moon: &Moon) -> i64 {
    let pot = moon.pos.0.abs() + moon.pos.1.abs() + moon.pos.2.abs();
    let kin = moon.vel.0.abs() + moon.vel.1.abs() + moon.vel.2.abs();
    pot * kin
}

fn step(mut moons: Vec<Moon>) -> Vec<Moon> {
    for mooni in 0..moons.len() {
        let mut acc = (0, 0, 0);
        for gravmoon in &moons {
            acc = add(&acc, &gravity(&moons[mooni], gravmoon));
        }
        moons[mooni].vel = add(&moons[mooni].vel, &acc);
    }
    for moon in &mut moons {
        moon.pos = add(&moon.pos, &moon.vel);
    }
    moons
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64, c: i64) -> i64 {
    let gcdab = gcd(a, b);
    let lcmab = (a / gcdab) * b;
    let gcdabc = gcd(lcmab, c);
    (lcmab / gcdabc) * c
}

pub fn solve(lines: &[String]) -> Solution {
    let mut x_states: HashSet<Vec<(i64, i64)>> = HashSet::new();
    let mut y_states: HashSet<Vec<(i64, i64)>> = HashSet::new();
    let mut z_states: HashSet<Vec<(i64, i64)>> = HashSet::new();

    let mut moons: Vec<Moon> = lines
        .iter()
        .map(|line| {
            let parts = line.split(',');
            let posv: Vec<i64> = parts
                .into_iter()
                .map(|part| {
                    part.split('=')
                        .nth(1)
                        .unwrap()
                        .split('>')
                        .next()
                        .unwrap()
                        .parse()
                        .unwrap()
                })
                .collect();
            Moon {
                pos: (posv[0], posv[1], posv[2]),
                vel: (0, 0, 0),
            }
        })
        .collect();

    let mut b_solution: Option<i64> = None;

    for _ in 0..1000 {
        moons = step(moons);
        x_states.insert(moons.iter().map(|moon| (moon.pos.0, moon.vel.0)).collect());
        y_states.insert(moons.iter().map(|moon| (moon.pos.1, moon.vel.1)).collect());
        z_states.insert(moons.iter().map(|moon| (moon.pos.2, moon.vel.2)).collect());
    }

    let a_solution: i64 = moons.iter().map(energy).sum();

    let mut x_cycle = None;
    let mut y_cycle = None;
    let mut z_cycle = None;

    for i in 1000.. {
        moons = step(moons);

        if x_cycle.is_none() {
            let x_state = moons.iter().map(|moon| (moon.pos.0, moon.vel.0)).collect();
            if x_states.contains(&x_state) {
                x_cycle = Some(i);
            }
            x_states.insert(x_state);
        }

        if y_cycle.is_none() {
            let y_state = moons.iter().map(|moon| (moon.pos.1, moon.vel.1)).collect();
            if y_states.contains(&y_state) {
                y_cycle = Some(i);
            }
            y_states.insert(y_state);
        }

        if z_cycle.is_none() {
            let z_state = moons.iter().map(|moon| (moon.pos.2, moon.vel.2)).collect();
            if z_states.contains(&z_state) {
                z_cycle = Some(i);
            }
            z_states.insert(z_state);
        }

        if let (Some(xc), Some(yc), Some(zc)) = (x_cycle, y_cycle, z_cycle) {
            b_solution = Some(lcm(xc, yc, zc));
            break;
        }
    }

    (a_solution.to_string(), b_solution.unwrap().to_string())
}
