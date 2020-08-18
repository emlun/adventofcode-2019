use crate::common::Solution;
use crate::util::sign;

#[derive(Clone, Eq, PartialEq)]
struct Moon {
    pos: i64,
    vel: i64,
}

fn gravity(on: &Moon, from: &Moon) -> i64 {
    sign(from.pos - on.pos)
}

fn energy(x: &Moon, y: &Moon, z: &Moon) -> i64 {
    let pot = x.pos.abs() + y.pos.abs() + z.pos.abs();
    let kin = x.vel.abs() + y.vel.abs() + z.vel.abs();
    pot * kin
}

fn step(mut moons: Vec<Moon>) -> Vec<Moon> {
    for mooni in 0..moons.len() {
        moons[mooni].vel += moons
            .iter()
            .map(|gravmoon| gravity(&moons[mooni], gravmoon))
            .sum::<i64>();
    }
    for moon in &mut moons {
        moon.pos += moon.vel;
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
    let mut moons_x: Vec<Moon> = Vec::new();
    let mut moons_y: Vec<Moon> = Vec::new();
    let mut moons_z: Vec<Moon> = Vec::new();
    for line in lines {
        let mut pos = line.split(',').map(|part| {
            part.split('=')
                .nth(1)
                .unwrap()
                .split('>')
                .next()
                .unwrap()
                .parse()
                .unwrap()
        });

        moons_x.push(Moon {
            pos: pos.next().unwrap(),
            vel: 0,
        });

        moons_y.push(Moon {
            pos: pos.next().unwrap(),
            vel: 0,
        });

        moons_z.push(Moon {
            pos: pos.next().unwrap(),
            vel: 0,
        });
    }

    let initial_x_state: Vec<Moon> = moons_x.clone();
    let initial_y_state: Vec<Moon> = moons_y.clone();
    let initial_z_state: Vec<Moon> = moons_z.clone();

    for _ in 0..1000 {
        moons_x = step(moons_x);
        moons_y = step(moons_y);
        moons_z = step(moons_z);
    }

    let a_solution: i64 = moons_x
        .iter()
        .zip(moons_y.iter())
        .zip(moons_z.iter())
        .map(|((x, y), z)| energy(x, y, z))
        .sum();

    let mut x_period = None;
    let mut y_period = None;
    let mut z_period = None;

    for i in 1001.. {
        moons_x = step(moons_x);

        if initial_x_state == moons_x {
            x_period = Some(i);
            break;
        }
    }

    for i in 1001.. {
        moons_y = step(moons_y);

        if initial_y_state == moons_y {
            y_period = Some(i);
            break;
        }
    }

    for i in 1001.. {
        moons_z = step(moons_z);

        if initial_z_state == moons_z {
            z_period = Some(i);
            break;
        }
    }

    if let (Some(xp), Some(yp), Some(zp)) = (x_period, y_period, z_period) {
        (a_solution.to_string(), lcm(xp, yp, zp).to_string())
    } else {
        unreachable!();
    }
}
