use anyhow::*;
use itertools::Itertools;

pub static DAY: u32 = 05;
pub static EXAMPLE_INPUT: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

pub fn main(input: &str) -> Result<(i64, i64)> {
    let (first, second) = input.split("\n\n").next_tuple().context("invalid input")?;

    let ranges = first
        .lines()
        .map(|line| {
            let mut parts = line.split('-');
            Ok((
                parts.next().context("missing first number")?.parse()?,
                parts.next().context("missing second number")?.parse()?,
            ))
        })
        .collect::<Result<Vec<(i64, i64)>>>()?;

    let ids = second
        .lines()
        .map(|x| Ok(x.parse()?))
        .collect::<Result<Vec<i64>>>()?;

    Ok((part1(&ranges, &ids), part2(ranges)))
}

fn part1(ranges: &Vec<(i64, i64)>, ids: &Vec<i64>) -> i64 {
    let mut result = 0;
    for id in ids {
        for (left, right) in ranges {
            if left <= id && id <= right {
                result += 1;
                break;
            }
        }
    }
    result
}

fn part2(mut ranges: Vec<(i64, i64)>) -> i64 {
    ranges.sort();

    for i in 0..(ranges.len() - 1) {
        let j = i + 1;
        if ranges[i].1 < ranges[j].0 {
            continue; // no overlap, continue!
        }
        // split ranges so they don't overlap
        if ranges[i].1 > ranges[j].1 {
            ranges[j].1 = ranges[i].1;
        }
        ranges[i].1 = ranges[j].0 - 1;
    }

    let mut result = 0;
    for (left, right) in ranges {
        if left <= right {
            result += right - left + 1;
        }
    }
    result
}
