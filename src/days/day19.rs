use crate::common::Solution;
use crate::intcode::IntcodeComputer;
use std::collections::HashMap;

const ENABLE_OUTPUT: bool = false;
const DIM_WANTED: usize = 100;

type Point = (usize, usize);

#[derive(Debug)]
struct Image {
    dim: usize,
    minx: Vec<usize>,
    maxx: Vec<usize>,
    miny: Vec<usize>,
    maxy: Vec<usize>,
}

impl Image {
    fn new() -> Image {
        Image {
            dim: 1,
            minx: vec![0],
            maxx: vec![1],
            miny: vec![0],
            maxy: vec![1],
        }
    }

    fn contains(&self, &(x, y): &Point) -> bool {
        x >= self.minx[y] && x < self.maxx[y] && y >= self.miny[x] && y < self.maxy[x]
    }
}

fn check(computer: &IntcodeComputer, (x, y): Point) -> bool {
    computer.clone().run(vec![x as i64, y as i64])[0] == 1
}

fn expand(mut image: Image, computer: &IntcodeComputer) -> Image {
    let d = image.dim - 1;

    image.minx.push(image.minx[d]);
    image.maxx.push(image.maxx[d]);
    image.miny.push(image.miny[d]);
    image.maxy.push(image.maxy[d]);

    for x in image.minx[d]..(image.minx[d] + 10) {
        let y = image.dim;
        if check(computer, (x, y)) {
            image.minx[image.dim] = x;
            break;
        }
    }
    for x in std::cmp::max(image.minx[image.dim], image.maxx[d])..(image.maxx[d] + 10) {
        let y = image.dim;
        if !check(computer, (x, y)) {
            image.maxx[image.dim] = x;
            break;
        }
    }

    for y in image.miny[d]..(image.miny[d] + 10) {
        let x = image.dim;
        if check(computer, (x, y)) {
            image.miny[image.dim] = y;
            break;
        }
    }

    for y in std::cmp::max(image.miny[image.dim], image.maxy[d])..(image.maxy[d] + 10) {
        let x = image.dim;
        if !check(computer, (x, y)) {
            image.maxy[image.dim] = y;
            break;
        }
    }

    image.dim += 1;
    image
}

fn solve_a(computer: IntcodeComputer) -> usize {
    let comp = &computer;
    let points: HashMap<(i64, i64), bool> = (0..50)
        .flat_map(|x| (0..50).map(move |y| ((x, y), comp.clone().run(vec![x, y])[0] == 1)))
        .collect();

    println!(
        "{}",
        (0..50)
            .map(|y| (0..50)
                .map(|x| if *points.get(&(x, y)).unwrap() {
                    "#"
                } else if x == y {
                    "+"
                } else {
                    "."
                })
                .collect::<Vec<&str>>()
                .join(""))
            .collect::<Vec<String>>()
            .join("\n")
    );

    points.values().filter(|v| **v).count()
}

fn solve_b(computer: IntcodeComputer) -> usize {
    let mut image: Image = Image::new();

    for dim in 1.. {
        image = expand(image, &computer);
        let y = image.dim - 1;
        let maxx = image.maxx[y];
        if maxx >= DIM_WANTED {
            let x = maxx - DIM_WANTED;
            if image.contains(&(x, y)) {
                let maxy = image.maxy[x];
                if maxy >= y + DIM_WANTED {
                    if ENABLE_OUTPUT {
                        let wy = maxy - y;
                        for _ in dim..(y + wy + 5) {
                            image = expand(image, &computer);
                        }
                        println!();
                        println!(
                            "{}",
                            (0..image.dim)
                                .map(|yy| (0..image.dim)
                                    .map(|xx| if image.contains(&(xx, yy)) {
                                        if xx >= x && yy >= y && xx < maxx && yy < maxy {
                                            "O"
                                        } else {
                                            "#"
                                        }
                                    } else if xx == yy {
                                        "+"
                                    } else {
                                        "."
                                    })
                                    .collect::<Vec<&str>>()
                                    .join(""))
                                .collect::<Vec<String>>()
                                .join("\n")
                        );

                        println!("{} {} {}", x, y, image.dim);
                    }
                    return x * 10000 + y;
                }
            }
        }
    }

    0
}

pub fn solve(lines: &[String]) -> Solution {
    let computer: IntcodeComputer = lines.into();
    let a_solution = solve_a(computer.clone());
    let b_solution = solve_b(computer);
    (a_solution.to_string(), b_solution.to_string())
}
