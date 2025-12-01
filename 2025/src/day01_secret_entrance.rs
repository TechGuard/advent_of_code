use anyhow::*;
use itertools::Itertools;

pub static DAY: u32 = 01;
pub static EXAMPLE_INPUT: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

pub fn main(input: &str) -> Result<(String, String)> {
    let mut dial = 50;
    let mut result1 = 0;

    for line in input.lines() {
        let mut chars = line.chars();
        let dir = match chars.next() {
            Some('L') => -1,
            Some('R') => 1,
            _ => bail!("Invalid direction"),
        };
        let amount: i32 = chars.as_str().parse()?;

        dial += dir * amount;
        dial %= 100;

        if dial == 0 {
            result1 += 1;
        }
    }

    Ok((result1.to_string(), "Not implemented".into()))
}
