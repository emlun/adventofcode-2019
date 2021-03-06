use crate::common::Solution;
use std::cmp::Ordering;

fn parse_wire(desc: &str) -> Vec<LineSegment> {
    let mut points: Vec<LineSegment> = Vec::new();
    let mut pos = (0, 0);
    let mut tot_len = 0;
    for step in desc.split(',') {
        let dir = match step.chars().next().unwrap() {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => unreachable!(),
        };
        let len: i32 = step[1..].parse().unwrap();
        let pos_off_origin = if pos == (0, 0) { dir } else { pos };
        let end = (pos.0 + dir.0 * len, pos.1 + dir.1 * len);
        points.push(LineSegment {
            x_start: std::cmp::min(pos_off_origin.0, end.0),
            x_end: std::cmp::max(pos_off_origin.0, end.0),
            y_start: std::cmp::min(pos_off_origin.1, end.1),
            y_end: std::cmp::max(pos_off_origin.1, end.1),

            walk_pos: pos,
            walk_len: tot_len,
        });
        pos = end;
        tot_len += len;
    }
    points
}

struct LineSegment {
    x_start: i32,
    x_end: i32,
    y_start: i32,
    y_end: i32,

    walk_pos: (i32, i32),
    walk_len: i32,
}

impl LineSegment {
    fn intersection(&self, other: &LineSegment) -> Option<Intersection> {
        if self.x_start == self.x_end
            && other.x_start == other.x_end
            && self.x_start == other.x_start
        {
            let y_start = std::cmp::max(self.y_start, other.y_start);
            let y_end = std::cmp::min(self.y_end, other.y_end);
            if y_end >= y_start {
                unimplemented!("Parallel line segments are not supported")
            } else {
                None
            }
        } else if self.y_start == self.y_end
            && other.y_start == other.y_end
            && self.y_start == other.y_start
        {
            let x_start = std::cmp::max(self.x_start, other.x_start);
            let x_end = std::cmp::min(self.x_end, other.x_end);
            if x_end >= x_start {
                unimplemented!("Parallel line segments are not supported")
            } else {
                None
            }
        } else if self.x_start <= other.x_start
            && self.x_end >= other.x_start
            && other.y_start <= self.y_start
            && other.y_end >= self.y_start
        {
            let x = other.x_start;
            let y = self.y_start;
            Some(Intersection {
                x,
                y,
                walk_len: self.walk_len
                    + other.walk_len
                    + (x - self.walk_pos.0).abs()
                    + (y - other.walk_pos.1).abs(),
            })
        } else if other.x_start <= self.x_start
            && other.x_end >= self.x_start
            && self.y_start <= other.y_start
            && self.y_end >= other.y_start
        {
            let x = self.x_start;
            let y = other.y_start;
            Some(Intersection {
                x,
                y,
                walk_len: self.walk_len
                    + other.walk_len
                    + (x - other.walk_pos.0).abs()
                    + (y - self.walk_pos.1).abs(),
            })
        } else {
            None
        }
    }
}

struct Intersection {
    x: i32,
    y: i32,
    walk_len: i32,
}

fn binary_search<F, T>(list: &[T], test: F) -> usize
where
    F: Fn(&T) -> Ordering,
{
    let mut i_min = 0;
    let mut i_max = list.len();
    while i_min < i_max {
        let i = (i_min + i_max) / 2;
        match test(&list[i]) {
            Ordering::Less => i_max = i - 1,
            Ordering::Greater => i_min = i + 1,
            Ordering::Equal => {
                return i;
            }
        };
    }
    i_min
}

pub fn solve(lines: &[String]) -> Solution {
    let wire1 = parse_wire(&lines[0]);
    let wire2 = parse_wire(&lines[1]);

    let (mut wire1_x, mut wire1_y): (Vec<&LineSegment>, Vec<&LineSegment>) =
        wire1.iter().partition(|seg| seg.x_start == seg.x_end);

    wire1_x.sort_by_key(|seg| seg.x_start);
    wire1_y.sort_by_key(|seg| seg.y_start);

    let mut a_solution = wire2.last().unwrap().walk_len;
    let mut b_solution = a_solution;

    for seg2 in wire2 {
        if seg2.x_start == seg2.x_end {
            let wire1_i_min = binary_search(&wire1_y, |seg| seg2.y_start.cmp(&seg.y_start));
            for seg1 in wire1_y
                .iter()
                .skip(wire1_i_min)
                .take_while(|seg| seg.y_start <= seg2.y_end)
            {
                if let Some(isct) = seg1.intersection(&seg2) {
                    a_solution = std::cmp::min(a_solution, isct.x.abs() + isct.y.abs());
                    b_solution = std::cmp::min(b_solution, isct.walk_len);
                }
            }
        } else {
            let wire1_i_min = binary_search(&wire1_x, |seg| seg2.x_start.cmp(&seg.x_start));
            for seg1 in wire1_x
                .iter()
                .skip(wire1_i_min)
                .take_while(|seg| seg.x_start <= seg2.x_end)
            {
                if let Some(isct) = seg1.intersection(&seg2) {
                    a_solution = std::cmp::min(a_solution, isct.x.abs() + isct.y.abs());
                    b_solution = std::cmp::min(b_solution, isct.walk_len);
                }
            }
        }
    }

    (a_solution.to_string(), b_solution.to_string())
}
