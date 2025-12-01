use anyhow::*;

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

pub fn main(input: &str) -> Result<(i32, i32)> {
    let mut dial = 50;
    let mut result1 = 0;
    let mut result2 = 0;

    for line in input.lines() {
        let mut chars = line.chars();
        let dir = match chars.next() {
            Some('L') => -1,
            Some('R') => 1,
            _ => bail!("Invalid direction"),
        };
        let amount: i32 = chars.as_str().parse()?;

        let begin_at_zero = dial == 0;
        dial += dir * amount;

        if dial <= 0 {
            result2 += dial.abs() / 100;
            if !begin_at_zero {
                result2 += 1;
            }
        } else if dial > 99 {
            result2 += dial / 100;
        }

        dial = dial.rem_euclid(100);
        if dial == 0 {
            result1 += 1;
        }
    }

    Ok((result1, result2))
}
