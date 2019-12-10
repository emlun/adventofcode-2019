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

pub fn solve(lines: &[String]) -> Solution {
    let map: HashSet<Point> = lines
        .iter()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(|(c, ch)| (r, c, ch))
                .collect::<Vec<(usize, usize, char)>>()
                .into_iter()
        })
        .filter(|(_, _, ch)| *ch == '#')
        .map(|(r, c, _)| (r.try_into().unwrap(), c.try_into().unwrap()))
        .collect();

    let mut max_vis = 0;
    let mut laser_pos: Point = (0, 0);
    let mut laser_rays: HashSet<Point> = HashSet::new();
    let mut asteroid_rays: HashMap<Point, Vec<Point>> = HashMap::new();

    fn normalize_dir(r: i64, c: i64) -> (i64, i64) {
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

    for (r0, c0) in &map {
        let mut recentered_map: HashSet<Point> =
            map.iter().map(|(r, c)| ((r - r0), (c - c0))).collect();
        recentered_map.remove(&(0, 0));
        let mut recentered_map: Vec<Point> = recentered_map.into_iter().collect();
        recentered_map.sort_by_key(|(r, c)| r.abs() + c.abs());

        let mut visible = 0;
        let mut blocked_rays: HashSet<(i64, i64)> = HashSet::new();
        let mut tmp_asteroid_rays: HashMap<Point, Vec<Point>> = HashMap::new();
        for (r, c) in recentered_map.iter() {
            let ray = normalize_dir(*r, *c);

            if !blocked_rays.contains(&ray) {
                visible += 1;
                tmp_asteroid_rays.insert(ray, Vec::new());
            }
            tmp_asteroid_rays.get_mut(&ray).unwrap().push((*r, *c));
            blocked_rays.insert(ray);
        }
        if visible > max_vis {
            max_vis = visible;
            laser_pos = (*r0, *c0);
            laser_rays = blocked_rays;
            asteroid_rays = tmp_asteroid_rays;
        }
    }

    fn ray_atan((r, c): &Point) -> f64 {
        ((*c as f64).atan2(-*r as f64) + 2.0 * std::f64::consts::PI) % (2.0 * std::f64::consts::PI)
    }

    let mut laser_rays: Vec<Point> = laser_rays.into_iter().collect();
    laser_rays.sort_by(|ray1, ray2| {
        let diff = ray_atan(ray1) - ray_atan(ray2);
        if diff < 0.0 {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    let mut asteroid_rays: Vec<Vec<Point>> = laser_rays
        .into_iter()
        .map(|ray| asteroid_rays.remove(&ray).unwrap())
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

    (max_vis.to_string(), b_solution.to_string())
}
