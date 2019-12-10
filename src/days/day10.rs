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

    let mut visibility: HashMap<Point, usize> = HashMap::new();
    let mut max_vis = 0;

    for (r0, c0) in &map {
        let mut recentered_map: HashSet<Point> =
            map.iter().map(|(r, c)| ((r - r0), (c - c0))).collect();
        recentered_map.remove(&(0, 0));
        let mut recentered_map: Vec<Point> = recentered_map.into_iter().collect();
        recentered_map.sort_by_key(|(r, c)| r.abs() + c.abs());

        let mut visible = 0;
        let mut blocked_rays: HashSet<(i64, i64)> = HashSet::new();
        for (r, c) in recentered_map {
            let ray = if c == 0 {
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
            };

            if !blocked_rays.contains(&ray) {
                visible += 1;
            }
            blocked_rays.insert(ray);
        }
        max_vis = std::cmp::max(max_vis, visible);
    }

    (max_vis.to_string(), "bar".to_string())
}
