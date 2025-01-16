use anyhow::*;
use itertools::Itertools;
use std::collections::BTreeMap;

pub static DAY: u32 = 14;
pub static EXAMPLE_INPUT: &str = "\
9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL
";

pub fn main(input: &str) -> Result<(usize, usize)> {
    let mut reactions = Vec::new();
    for line in input.lines() {
        let (before, after) = line
            .split(" => ")
            .collect_tuple()
            .context("invalid reaction")?;
        let input = before
            .split(", ")
            .map(|s| parse_chemical(s))
            .try_collect()?;
        reactions.push(Reaction {
            input,
            output: parse_chemical(after)?,
        });
    }

    let minimum_ore = ore_for_fuel(&reactions, 1)?;
    let mut min_fuel = 1000000000000 / minimum_ore;
    let mut max_fuel = min_fuel * 2;

    // Simple binary search
    let maximum_fuel = loop {
        let fuel = (min_fuel + max_fuel) / 2;
        let ore = ore_for_fuel(&reactions, fuel)?;
        if ore < 1000000000000 {
            if fuel == min_fuel {
                break fuel;
            } else {
                min_fuel = fuel;
            }
        } else {
            max_fuel = fuel;
        }
    };

    Ok((minimum_ore, maximum_fuel))
}

fn ore_for_fuel(reactions: &Vec<Reaction>, fuel: usize) -> Result<usize> {
    let mut chemicals_to_produce = vec![(fuel, "FUEL")];
    let mut residual_chemicals = BTreeMap::new();
    let mut ore = 0;

    while let Some((mut required_quantity, chemical)) = chemicals_to_produce.pop() {
        if chemical == "ORE" {
            ore += required_quantity;
            continue;
        }

        // Use up residual chemicals
        if let Some(residual) = residual_chemicals.get_mut(&chemical) {
            if *residual < required_quantity {
                required_quantity -= *residual;
                *residual = 0;
            } else {
                *residual -= required_quantity;
                continue;
            }
        }

        // Find reaction that produces this chemical
        let reaction = reactions
            .iter()
            .find(|r| r.output.1 == chemical)
            .with_context(|| anyhow!("Missing required chemical: {}", chemical))?;

        // Round up to match minimum required quantity
        let output_quantity = reaction.output.0;
        let required_reactions = (required_quantity + output_quantity - 1) / output_quantity;

        // Add reaction input to chemicals to produce
        for (quantity, chemical) in &reaction.input {
            chemicals_to_produce.push((required_reactions * quantity, chemical));
        }

        // Store residual chemicals
        let residual_quantity = (required_reactions * output_quantity) - required_quantity;
        *residual_chemicals.entry(chemical).or_default() += residual_quantity;
    }
    Ok(ore)
}

fn parse_chemical(s: &str) -> Result<(usize, &str)> {
    let (quantity, chemical) = s
        .split_ascii_whitespace()
        .collect_tuple()
        .context("Invalid chemical")?;
    Ok((quantity.parse()?, chemical))
}

#[derive(Debug, Clone)]
struct Reaction<'a> {
    input: Vec<(usize, &'a str)>,
    output: (usize, &'a str),
}
