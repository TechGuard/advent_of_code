use anyhow::*;

pub static DAY: u32 = 03;
pub static EXAMPLE_INPUT: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

pub fn main(input: &str) -> Result<(i64, i64)> {
    let banks = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).context("invalid character"))
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?;

    Ok((solve(&banks, 2), solve(&banks, 12)))
}

fn solve(banks: &Vec<Vec<u32>>, count: usize) -> i64 {
    let mut result = 0;
    for bank in banks {
        let mut joltage = 0;
        let mut left = 0;

        for remaining in (0..count).rev() {
            let mut num = 0;

            for i in left..(bank.len() - remaining) {
                if num < bank[i] {
                    num = bank[i];
                    left = i;
                }
            }

            joltage *= 10;
            joltage += num as i64;

            left += 1;
        }

        result += joltage;
    }
    result
}
