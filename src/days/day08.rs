use crate::common::Solution;

pub fn solve(lines: &[String]) -> Solution {
    let mut digits = lines.iter().flat_map(|line| line.chars());
    let mut images: Vec<Vec<u8>> = Vec::new();
    let W = 25;
    let H = 6;
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

    (a_solution.to_string(), "bar".to_string())
}
