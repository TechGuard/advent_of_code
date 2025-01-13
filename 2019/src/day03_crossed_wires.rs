use anyhow::*;
use itertools::Itertools;
use std::collections::BTreeMap;

pub static DAY: u32 = 03;
pub static EXAMPLE_INPUT: &str = "\
R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83
";

pub fn main(input: &str) -> Result<(i64, i64)> {
    let mut wires = input.lines().map(|line| {
        line.split(",")
            .map(|s| {
                let dir = s
                    .chars()
                    .next()
                    .context("Invalid input, expected direction")?;
                let length: i64 = s
                    .get(1..)
                    .context("Invalid input, expected length")?
                    .parse()?;
                Ok(match dir {
                    'R' => Direction::Right(length),
                    'D' => Direction::Down(length),
                    'L' => Direction::Left(length),
                    'U' => Direction::Up(length),
                    _ => bail!("Invalid direction: {}", dir),
                })
            })
            .try_collect()
    });
    let wire0 = wires.next().context("Expected at least 1 wire")??;
    let wire1 = wires.next().context("Expected at least 2 wires")??;
    let mut visisted = BTreeMap::new();
    let mut intersections1 = Vec::new();
    let mut intersections2 = Vec::new();
    for_each_step(&wire0, |step, pos| {
        if !visisted.contains_key(&pos) {
            visisted.insert(pos, step);
        }
    });
    for_each_step(&wire1, |step, pos| {
        if let Some(other_step) = visisted.get(&pos) {
            intersections1.push(pos.0.abs() + pos.1.abs());
            intersections2.push(step + other_step);
        }
    });
    Ok((
        *intersections1.iter().min().unwrap(),
        *intersections2.iter().min().unwrap(),
    ))
}

#[derive(Debug)]
enum Direction {
    Right(i64),
    Down(i64),
    Left(i64),
    Up(i64),
}

fn for_each_step<F: FnMut(i64, (i64, i64))>(directions: &Vec<Direction>, mut callback: F) {
    let mut step = 0;
    let mut pos = (0, 0);
    for direction in directions {
        let (offset, length) = match *direction {
            Direction::Right(length) => ((0, 1), length),
            Direction::Down(length) => ((1, 0), length),
            Direction::Left(length) => ((0, -1), length),
            Direction::Up(length) => ((-1, 0), length),
        };
        for _ in 0..length {
            step += 1;
            pos = (pos.0 + offset.0, pos.1 + offset.1);
            callback(step, pos);
        }
    }
}
