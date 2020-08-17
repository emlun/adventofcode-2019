use crate::common::Solution;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;

type Point = (i64, i64);

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn normalize((r, c): Point) -> Point {
    if c == 0 {
        if r == 0 {
            unreachable!();
        } else {
            (r / r.abs(), 0)
        }
    } else if r == 0 {
        (0, c / c.abs())
    } else {
        let d = gcd(r.abs(), c.abs());
        (r / d, c / d)
    }
}

fn ray_atan((r, c): &Point) -> f64 {
    ((*c as f64).atan2(-*r as f64) + 2.0 * std::f64::consts::PI) % (2.0 * std::f64::consts::PI)
}

pub fn solve(lines: &[String]) -> Solution {
    let map: Vec<Point> = lines
        .iter()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '#')
                .map(|(c, ch)| (r, c, ch))
                .collect::<Vec<(usize, usize, char)>>()
                .into_iter()
        })
        .map(|(r, c, _)| (r.try_into().unwrap(), c.try_into().unwrap()))
        .collect();

    let (laser_pos, asteroid_rays): (Point, HashMap<Point, Vec<Point>>) = map
        .iter()
        .map(|(r0, c0)| {
            let mut recentered_map: HashSet<Point> =
                map.iter().map(|(r, c)| ((r - r0), (c - c0))).collect();
            recentered_map.remove(&(0, 0));
            let mut recentered_map: Vec<Point> = recentered_map.into_iter().collect();
            recentered_map.sort_by_key(|(r, c)| r.abs() + c.abs());

            let asteroid_rays: HashMap<Point, Vec<Point>> =
                recentered_map
                    .into_iter()
                    .fold(HashMap::new(), |mut result, pos| {
                        let ray = normalize(pos);
                        result.entry(ray).or_insert_with(Vec::new).push(pos);
                        result
                    });
            ((*r0, *c0), asteroid_rays)
        })
        .max_by_key(|(_, rays)| rays.len())
        .unwrap();
    let a_solution = asteroid_rays.len();

    let mut asteroid_rays: Vec<(Point, Vec<Point>)> = asteroid_rays
        .into_iter()
        .collect::<Vec<(Point, Vec<Point>)>>();
    asteroid_rays.sort_by(|(dir1, _), (dir2, _)| {
        if ray_atan(dir1) - ray_atan(dir2) < 0.0 {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    let mut asteroid_rays: Vec<Vec<Point>> = asteroid_rays
        .into_iter()
        .map(|(_, asteroids)| asteroids)
        .collect();

    let mut num = 0;
    let mut i = 0;
    let mut b_solution = 0;
    while !asteroid_rays.is_empty() {
        let ast = asteroid_rays[i].remove(0);
        let orig_ast = (ast.0 + laser_pos.0, ast.1 + laser_pos.1);
        i += 1;
        num += 1;
        if num == 200 {
            b_solution = orig_ast.0 + orig_ast.1 * 100;
        }
        if i >= asteroid_rays.len() {
            i -= asteroid_rays.len();
            asteroid_rays = asteroid_rays
                .into_iter()
                .filter(|v| !v.is_empty())
                .collect();
        }
    }

    (a_solution.to_string(), b_solution.to_string())
}
