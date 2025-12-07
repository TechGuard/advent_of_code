use anyhow::*;
use std::collections::HashMap;

pub static DAY: u32 = 07;
pub static EXAMPLE_INPUT: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

pub fn main(input: &str) -> Result<(i64, i64)> {
    let (start, splitters) = parse_input(input);

    let mut beams = HashMap::new();
    beams.insert(start, 1);

    let mut result1 = 0;
    for x in splitters {
        if let Some(count) = beams.remove(&x) {
            *beams.entry(x - 1).or_default() += count;
            *beams.entry(x + 1).or_default() += count;
            result1 += 1;
        }
    }

    Ok((result1, beams.values().sum()))
}

fn parse_input(input: &str) -> (i64, Vec<i64>) {
    let mut start = 0;
    let mut splitters = Vec::new();
    for line in input.lines() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = x as i64;
            } else if c == '^' {
                splitters.push(x as i64);
            }
        }
    }
    (start, splitters)
}
