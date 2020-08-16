#![allow(clippy::naive_bytecount)]

use crate::common::Solution;

const W: usize = 25;
const H: usize = 6;

pub fn solve_b(layers: &[Vec<u8>]) -> String {
    let mut image: Vec<Vec<u8>> = vec![vec![2; W]; H];
    for (r, row) in image.iter_mut().enumerate() {
        for (c, col) in row.iter_mut().enumerate() {
            let rc = r * W + c;
            for layer in layers {
                if layer[rc] != 2 {
                    *col = layer[rc];
                    break;
                }
            }
        }
    }

    format!(
        "\n{}",
        image
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| match cell {
                        0 => ".",
                        1 => "#",
                        2 => " ",
                        _ => unreachable!(),
                    })
                    .collect::<Vec<&str>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n")
    )
}

pub fn solve(lines: &[String]) -> Solution {
    let mut digits = lines.iter().flat_map(|line| line.chars());
    let mut images: Vec<Vec<u8>> = Vec::new();
    while let Some(digit0) = digits.next() {
        let mut img = Vec::new();
        img.push(digit0.to_string().parse().unwrap());
        for _ in 1..W {
            img.push(digits.next().unwrap().to_string().parse().unwrap());
        }
        for _ in 1..H {
            for _ in 0..W {
                img.push(digits.next().unwrap().to_string().parse().unwrap());
            }
        }
        images.push(img);
    }

    let fewest_zeros = images
        .iter()
        .min_by_key(|img| img.iter().filter(|d| **d == 0).count())
        .unwrap();

    let num_ones = fewest_zeros.iter().filter(|d| **d == 1).count();
    let num_twos = fewest_zeros.iter().filter(|d| **d == 2).count();
    let a_solution = num_ones * num_twos;
    let b_solution = solve_b(&images);

    (a_solution.to_string(), b_solution)
}
