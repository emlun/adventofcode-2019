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

fn step(moons: &mut Vec<Moon>) {
    for mooni in 0..moons.len() {
        moons[mooni].vel += moons
            .iter()
            .map(|gravmoon| gravity(&moons[mooni], gravmoon))
            .sum::<i64>();
    }
    for moon in moons {
        moon.pos += moon.vel;
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize, c: usize) -> usize {
    let gcdab = gcd(a, b);
    let lcmab = (a / gcdab) * b;
    let gcdabc = gcd(lcmab, c);
    (lcmab / gcdabc) * c
}

fn find_period(initial_state: Vec<Moon>, mut moons: Vec<Moon>) -> usize {
    for i in 1001.. {
        step(&mut moons);

        if moons.iter().all(|moon| moon.vel == 0) {
            if moons == initial_state {
                return i;
            } else {
                return i * 2;
            }
        }
    }
    unreachable!();
}

pub fn solve(lines: &[String]) -> Solution {
    let mut moons: Vec<Vec<Moon>> = vec![Vec::new(); 3];
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

        for m in moons.iter_mut() {
            m.push(Moon {
                pos: pos.next().unwrap(),
                vel: 0,
            });
        }
    }

    let initial_states = moons.clone();

    for _ in 0..1000 {
        for m in moons.iter_mut() {
            step(m);
        }
    }

    let a_solution: i64 = moons[0]
        .iter()
        .zip(moons[1].iter())
        .zip(moons[2].iter())
        .map(|((x, y), z)| energy(x, y, z))
        .sum();

    let periods: Vec<usize> = initial_states
        .into_iter()
        .zip(moons.into_iter())
        .map(|(i, s)| find_period(i, s))
        .collect();

    (
        a_solution.to_string(),
        lcm(periods[0], periods[1], periods[2]).to_string(),
    )
}
