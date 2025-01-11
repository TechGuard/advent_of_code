use anyhow::*;
use itertools::Itertools;

pub static DAY: u32 = 01;
pub static EXAMPLE_INPUT: &str = "100756";

pub fn main(input: &str) -> Result<(String, String)> {
    let fuel: Vec<_> = input
        .lines()
        .map(|s| s.parse().map(calc_fuel))
        .try_collect()?;
    Ok((
        fuel.iter().sum::<i64>().to_string(),
        fuel.into_iter()
            .map(calc_fuel_requirement)
            .sum::<i64>()
            .to_string(),
    ))
}

fn calc_fuel(mass: i64) -> i64 {
    mass / 3 - 2
}

fn calc_fuel_requirement(mut mass: i64) -> i64 {
    let mut total = 0;
    while mass > 0 {
        total += mass;
        mass = mass / 3 - 2;
    }
    total
}
