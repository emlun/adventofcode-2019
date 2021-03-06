use crate::common::Solution;
use std::collections::HashMap;

fn parse_qtyname(s: &str) -> (&str, i64) {
    let parts: Vec<&str> = s.trim().split(' ').map(&str::trim).collect();
    (parts[1], parts[0].parse().unwrap())
}

fn produce_everything<'a>(
    ingredients: &mut HashMap<&'a str, i64>,
    formulae: &HashMap<&'a str, (i64, HashMap<&'a str, i64>)>,
) {
    while let Some((ingredient, needed)) = ingredients
        .iter_mut()
        .filter(|(_, needed)| **needed > 0)
        .find(|(k, _)| **k != "ORE")
    {
        let (producing, metaingredients) = formulae.get(ingredient).unwrap();
        let times = *needed / producing + (if *needed % producing == 0 { 0 } else { 1 });
        *needed -= times * producing;

        for (metaingredient, needed) in metaingredients {
            *ingredients.entry(metaingredient).or_insert(0) += times * needed;
        }
    }
}

pub fn solve(lines: &[String]) -> Solution {
    let formulae: HashMap<&str, (i64, HashMap<&str, i64>)> = lines
        .iter()
        .map(|line| {
            let sides: Vec<&str> = line.split("=>").collect();
            let (out_name, out_qty) = parse_qtyname(sides[1]);
            let lhs: HashMap<&str, i64> = sides[0].split(',').map(parse_qtyname).collect();
            (out_name, (out_qty, lhs))
        })
        .collect();

    let mut ingredients: HashMap<&str, i64> = vec![("FUEL", 1)].into_iter().collect();

    produce_everything(&mut ingredients, &formulae);
    let a_solution: i64 = *ingredients.get("ORE").unwrap();

    let mut b_solution: i64 = 1;
    loop {
        let ore_remaining = 1_000_000_000_000 - ingredients.get("ORE").unwrap();
        if ore_remaining >= 0 {
            let additional_fuel = std::cmp::max(1, ore_remaining / a_solution);
            *ingredients.get_mut("FUEL").unwrap() += additional_fuel;
            produce_everything(&mut ingredients, &formulae);

            if *ingredients.get("ORE").unwrap() <= 1_000_000_000_000 {
                b_solution += additional_fuel;
            }
        } else {
            break;
        }
    }

    (a_solution.to_string(), b_solution.to_string())
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn even_ingredients() {
        assert_eq!(
            solve(&[
                "100 ORE => 20 FOO".to_string(),
                "1 FOO => 1 FUEL".to_string()
            ]),
            ("100".to_string(), "200000000000".to_string())
        );
    }
}
