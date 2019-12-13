use crate::common::Solution;
use crate::util::sign;
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
    let mut moons: Vec<Moon> = lines
        .iter()
        .map(|line| {
            let posv: Vec<i64> = line
                .split(',')
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

    let mut x_states: HashSet<Vec<(i64, i64)>> = HashSet::new();
    let mut y_states: HashSet<Vec<(i64, i64)>> = HashSet::new();
    let mut z_states: HashSet<Vec<(i64, i64)>> = HashSet::new();

    for _ in 0..1000 {
        moons = step(moons);
        x_states.insert(moons.iter().map(|moon| (moon.pos.0, moon.vel.0)).collect());
        y_states.insert(moons.iter().map(|moon| (moon.pos.1, moon.vel.1)).collect());
        z_states.insert(moons.iter().map(|moon| (moon.pos.2, moon.vel.2)).collect());
    }

    let a_solution: i64 = moons.iter().map(energy).sum();
    let mut b_solution: Option<i64> = None;

    let mut x_period = None;
    let mut y_period = None;
    let mut z_period = None;

    for i in 1000.. {
        moons = step(moons);

        if x_period.is_none() {
            let x_state = moons.iter().map(|moon| (moon.pos.0, moon.vel.0)).collect();
            if x_states.contains(&x_state) {
                x_period = Some(i);
            }
            x_states.insert(x_state);
        }

        if y_period.is_none() {
            let y_state = moons.iter().map(|moon| (moon.pos.1, moon.vel.1)).collect();
            if y_states.contains(&y_state) {
                y_period = Some(i);
            }
            y_states.insert(y_state);
        }

        if z_period.is_none() {
            let z_state = moons.iter().map(|moon| (moon.pos.2, moon.vel.2)).collect();
            if z_states.contains(&z_state) {
                z_period = Some(i);
            }
            z_states.insert(z_state);
        }

        if let (Some(xp), Some(yp), Some(zp)) = (x_period, y_period, z_period) {
            b_solution = Some(lcm(xp, yp, zp));
            break;
        }
    }

    (a_solution.to_string(), b_solution.unwrap().to_string())
}
