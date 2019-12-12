use crate::common::Solution;

type Point = (i64, i64, i64);

#[derive(Debug)]
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

pub fn solve(lines: &[String]) -> Solution {
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
    for _ in 0..1000 {
        moons = step(moons);
    }
    let a_solution: i64 = moons.iter().map(energy).sum();
    (a_solution.to_string(), "bar".to_string())
}
