use anyhow::*;
use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    vec,
};
use z3::{Optimize, ast::Int};

pub static DAY: u32 = 10;
pub static EXAMPLE_INPUT: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<i64>,
}

pub fn main(input: &str) -> Result<(i64, i64)> {
    let machines = parse_input(input)?;
    Ok((
        machines.iter().map(|m| solve_lights(m)).sum(),
        machines.iter().map(|m| solve_joltage(m)).sum(),
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

        for button in &machine.buttons {
            let mut values = values.clone();
            for &index in button {
                values[index] = !values[index];
            }
            if seen.insert(values.clone()) {
                queue.push_back((result + 1, values));
            }
        }
    }
    unreachable!("no answer")
}

fn solve_joltage(machine: &Machine) -> i64 {
    /*
        Example (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}

        Each button affects a set of joltages

          (3) = 0a + 0a + 0a + 1a
        (1,3) = 0b + 1b + 0b + 1b
          (2) = 0c + 0c + 1c + 0c
        (2,3) = 0d + 0d + 1d + 1d
        (0,2) = 1e + 0e + 1e + 0e
        (0,1) = 1f + 1f + 0f + 0f

        Each joltage is a sum of all buttons pressed that affect it

        3 = 1e + 2f
        5 = 3b + 2f
        4 = 3d + 1e
        7 = 1a + 3b + 3d
    */

    let solver = Optimize::new();

    // Create button variables a..f
    let buttons = (0..machine.buttons.len())
        .map(|i| Int::new_const(format!("button{}", i)))
        .collect_vec();

    // Each button is greater or equal to 0
    for button in &buttons {
        solver.assert(&button.ge(0));
    }

    // For each joltage create a sum of all buttons that affects it
    let mut joltage = vec![Int::from_i64(0); machine.joltage.len()];
    for button in 0..machine.buttons.len() {
        for index in machine.buttons[button].iter().cloned() {
            joltage[index] += buttons[button].clone();
        }
    }

    // Assert that each joltage is equal to the expected outcome
    for index in 0..machine.joltage.len() {
        solver.assert(&joltage[index].eq(machine.joltage[index]));
    }

    // Solve for the smallest possible amount of buttons
    solver.minimize(&Int::add(&buttons));
    solver.check(&[]);

    // The answer is the sum of all buttons
    let model = solver.get_model().unwrap();
    let mut answer = 0;
    for button in buttons {
        answer += model.get_const_interp(&button).unwrap().as_i64().unwrap();
    }
    answer
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

            let mut buttons = Vec::new();
            while sections.peek().is_some_and(|str| str.starts_with('(')) {
                let button = sections
                    .next()
                    .context("missing buttons")?
                    .trim_matches(['(', ')'])
                    .split(',')
                    .map(|str| Ok(str.parse()?))
                    .collect::<Result<_>>()?;
                buttons.push(button);
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
                buttons,
                joltage,
            })
        })
        .collect()
}
