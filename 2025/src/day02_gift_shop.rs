use anyhow::*;

pub static DAY: u32 = 02;
pub static EXAMPLE_INPUT: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124";

pub fn main(input: &str) -> Result<(i64, i64)> {
    let pairs = parse_input(input)?;
    let mut result1 = 0;
    let mut result2 = 0;

    for pair in pairs {
        for number in pair.0..=pair.1 {
            if part1(number) {
                result1 += number;
            }
            if part2(number) {
                result2 += number;
            }
        }
    }

    Ok((result1, result2))
}

fn part1(number: i64) -> bool {
    let str = number.to_string();
    if str.len() % 2 == 0 {
        let half = str.len() / 2;
        return str[0..half] == str[half..];
    }
    false
}

fn part2(number: i64) -> bool {
    let str = number.to_string();
    let len = str.len();
    for size in (1..=(len / 2)).rev() {
        if len % size == 0 {
            let mut offset = 0;
            let mut invalid = true;
            while offset + size < len {
                let end = offset + size;
                if str[offset..end] != str[end..end + size] {
                    invalid = false;
                    break;
                }
                offset += size;
            }
            if invalid {
                return true;
            }
        }
    }
    false
}

fn parse_input(input: &str) -> Result<Vec<(i64, i64)>> {
    input
        .lines()
        .next()
        .context("Missing input")?
        .split(',')
        .map(|str| {
            let mut parts = str.split('-');
            Ok((
                parts.next().context("Missing first number")?.parse()?,
                parts.next().context("Missing second number")?.parse()?,
            ))
        })
        .collect()
}
