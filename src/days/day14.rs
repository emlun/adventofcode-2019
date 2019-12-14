use crate::common::Solution;
use std::collections::HashMap;

fn parse_qtyname(s: &str) -> (&str, i32) {
    let parts: Vec<&str> = s.trim().split(' ').map(&str::trim).collect();
    (parts[1], parts[0].parse().unwrap())
}

pub fn solve(lines: &[String]) -> Solution {
    let formulae: HashMap<&str, (i32, HashMap<&str, i32>)> = lines
        .iter()
        .map(|line| {
            let sides: Vec<&str> = line.split("=>").collect();
            let (out_name, out_qty) = parse_qtyname(sides[1]);
            let lhs: HashMap<&str, i32> = sides[0].split(',').map(parse_qtyname).collect();
            (out_name, (out_qty, lhs))
        })
        .collect();

    let mut ingredients: HashMap<&str, i32> = vec![("FUEL", 1)].into_iter().collect();

    loop {
        if let Some((ingredient, qty)) = ingredients
            .iter_mut()
            .filter(|(_, qty)| **qty > 0)
            .filter(|(k, _)| **k != "ORE")
            .next()
        {
            let (produced, metaingredients) = formulae.get(ingredient).unwrap();
            let times = *qty / produced + (if *qty % produced == 0 { 0 } else { 1 });
            *qty -= times * produced;

            for (metaingredient, needed) in metaingredients {
                ingredients.insert(
                    metaingredient,
                    ingredients.get(metaingredient).unwrap_or(&0) + (times * needed),
                );
            }
        } else {
            break;
        }
    }

    let a_solution: i32 = *ingredients.get("ORE").unwrap();
    let b_solution: i32 = 0;

    (a_solution.to_string(), b_solution.to_string())
}
