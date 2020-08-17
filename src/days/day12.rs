use crate::common::Solution;
use crate::util::sign;

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

    let initial_x_state: Vec<(i64, i64)> =
        moons.iter().map(|moon| (moon.pos.0, moon.vel.0)).collect();
    let initial_y_state: Vec<(i64, i64)> =
        moons.iter().map(|moon| (moon.pos.1, moon.vel.1)).collect();
    let initial_z_state: Vec<(i64, i64)> =
        moons.iter().map(|moon| (moon.pos.2, moon.vel.2)).collect();

    for _ in 0..1000 {
        moons = step(moons);
    }

    let a_solution: i64 = moons.iter().map(energy).sum();
    let mut b_solution: Option<i64> = None;

    let mut x_period = None;
    let mut y_period = None;
    let mut z_period = None;

    for i in 1001.. {
        moons = step(moons);

        if x_period.is_none() {
            let x_state = moons.iter().map(|moon| (moon.pos.0, moon.vel.0));
            if initial_x_state.iter().zip(x_state).all(|(a, b)| *a == b) {
                x_period = Some(i);
            }
        }

        if y_period.is_none() {
            let y_state = moons.iter().map(|moon| (moon.pos.1, moon.vel.1));
            if initial_y_state.iter().zip(y_state).all(|(a, b)| *a == b) {
                y_period = Some(i);
            }
        }

        if z_period.is_none() {
            let z_state = moons.iter().map(|moon| (moon.pos.2, moon.vel.2));
            if initial_z_state.iter().zip(z_state).all(|(a, b)| *a == b) {
                z_period = Some(i);
            }
        }

        if let (Some(xp), Some(yp), Some(zp)) = (x_period, y_period, z_period) {
            b_solution = Some(lcm(xp, yp, zp));
            break;
        }
    }

    (a_solution.to_string(), b_solution.unwrap().to_string())
}
