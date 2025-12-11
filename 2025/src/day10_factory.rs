use anyhow::*;
use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    vec,
};

pub static DAY: u32 = 10;
pub static EXAMPLE_INPUT: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    diagrams: Vec<Vec<usize>>,
    joltage: Vec<i64>,
}

pub fn main(input: &str) -> Result<(i64, String)> {
    let machines = parse_input(input)?;
    Ok((
        machines.iter().map(|m| solve_lights(m)).sum(),
        "Not implemented".into(), // machines.iter().map(|m| solve_joltage(m)).sum(),
    ))
}

fn solve_lights(machine: &Machine) -> i64 {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_back((0, vec![false; machine.lights.len()]));

    while let Some((result, values)) = queue.pop_front() {
        if values == machine.lights {
            return result;
        }

        for diagram in &machine.diagrams {
            let mut values = values.clone();
            for &button in diagram {
                values[button] = !values[button];
            }
            if seen.insert(values.clone()) {
                queue.push_back((result + 1, values));
            }
        }
    }
    unreachable!("no answer")
}

// todo: part2
fn solve_joltage(machine: &Machine) -> i64 {
    println!("{:?}", machine);

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_back((Vec::new(), machine.joltage.clone()));

    while let Some((result, values)) = queue.pop_front() {
        if values.iter().all(|&x| x == 0) {
            let counts = result.iter().counts();
            for (k, v) in counts {
                print!("{}{} ", "abcdefg".chars().nth(*k).unwrap(), v);
            }
            println!();
            return result.len() as i64;
        }

        for diagram in &machine.diagrams {
            let mut values = values.clone();
            for &button in diagram {
                values[button] -= 1;
            }

            if values.iter().any(|&value| value < 0) {
                continue;
            }

            if seen.insert(values.clone()) {
                let mut result = result.clone();
                result.push(machine.diagrams.iter().position(|x| x == diagram).unwrap());
                queue.push_back((result, values));
            }
        }
    }
    unreachable!("no answer")
}

fn parse_input(input: &str) -> Result<Vec<Machine>> {
    input
        .lines()
        .map(|line| {
            let mut sections = line.split_ascii_whitespace().peekable();

            let lights = sections
                .next()
                .context("missing lights")?
                .trim_matches(['[', ']'])
                .chars()
                .map(|c| c == '#')
                .collect();

            let mut diagrams = Vec::new();
            while sections.peek().is_some_and(|str| str.starts_with('(')) {
                let diagram = sections
                    .next()
                    .context("missing diagrams")?
                    .trim_matches(['(', ')'])
                    .split(',')
                    .map(|str| Ok(str.parse()?))
                    .collect::<Result<_>>()?;
                diagrams.push(diagram);
            }

            let joltage = sections
                .next()
                .context("missing joltage")?
                .trim_matches(['{', '}'])
                .split(',')
                .map(|str| Ok(str.parse()?))
                .collect::<Result<_>>()?;

            Ok(Machine {
                lights,
                diagrams,
                joltage,
            })
        })
        .collect()
}
