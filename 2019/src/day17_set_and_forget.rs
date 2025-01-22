use crate::utils::intcode;
use anyhow::*;
use itertools::Itertools;
use std::collections::BTreeSet;

pub static DAY: u32 = 17;
pub static EXAMPLE_INPUT: &str = "";

pub fn main(input: &str) -> Result<(i64, i64)> {
    let data = intcode::parse_data(input)?;
    let (map, start) = build_map(data.clone())?;
    Ok((count_intersections(&map), collect_dust(&map, data, start)?))
}

fn build_map(data: Vec<i64>) -> Result<(BTreeSet<Pos>, Pos)> {
    let camera_output: String = intcode::execute_until_end(data, [])?
        .into_iter()
        .map(|x| char::from_u32(x.try_into()?).context("invalid character"))
        .try_collect()?;
    let mut map = BTreeSet::new();
    let mut start = (0, 0);
    for (y, line) in camera_output.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '^' {
                start = (y as _, x as _);
            } else if c == '#' {
                map.insert((y as _, x as _));
            }
        }
    }
    Ok((map, start))
}

fn count_intersections(map: &BTreeSet<Pos>) -> i64 {
    let mut result = 0;
    for &(y, x) in map {
        if map.contains(&(y + 1, x))
            && map.contains(&(y - 1, x))
            && map.contains(&(y, x + 1))
            && map.contains(&(y, x - 1))
        {
            result += y * x;
        }
    }
    result
}

fn collect_dust(map: &BTreeSet<Pos>, mut data: Vec<i64>, start: Pos) -> Result<i64> {
    let movement_instructions = build_movement_instructions(map, start);
    let patterns = find_repeating_instructions(&movement_instructions, &vec![])
        .context("cannot find repeatinng instruction set")?;
    let pattern_instructions = build_pattern_instructions(&movement_instructions, &patterns);

    let mut robot_instructions = Vec::new();

    // Main routine
    let mut add_comma = false;
    for instruction in pattern_instructions {
        if add_comma {
            robot_instructions.push(',' as _);
        } else {
            add_comma = true;
        }
        robot_instructions.push((('A' as usize) + instruction) as _);
    }
    robot_instructions.push('\n' as _); // new line

    // Functions
    for pattern in patterns {
        add_comma = false;
        for instruction in pattern {
            if add_comma {
                robot_instructions.push(',' as _);
            } else {
                add_comma = true;
            }
            match instruction {
                Movement::Forward(steps) => {
                    for c in steps.to_string().chars() {
                        robot_instructions.push(c as _);
                    }
                }
                Movement::TurnLeft => robot_instructions.push('L' as _),
                Movement::TurnRight => robot_instructions.push('R' as _),
            }
        }
        robot_instructions.push('\n' as _);
    }

    robot_instructions.push('n' as _); // decline video feed
    robot_instructions.push('\n' as _); // new line

    // Execute robot with all instructions and return last output
    data[0] = 2;
    let output = intcode::execute_until_end(data, robot_instructions)?;
    Ok(*output.last().context("robot did not return output")?)
}

fn build_pattern_instructions(
    movement_instructions: &Vec<Movement>,
    patterns: &Vec<Vec<Movement>>,
) -> Vec<usize> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < movement_instructions.len() {
        // Try to find pattern at for index i
        for (pattern_index, pattern) in patterns.iter().enumerate() {
            let mut matches = true;
            for j in 0..pattern.len() {
                if movement_instructions[i + j] != pattern[j] {
                    matches = false;
                    break;
                }
            }
            // Found pattern
            if matches {
                i += pattern.len();
                result.push(pattern_index);
                break;
            }
        }
    }
    result
}

fn find_repeating_instructions(
    movement_instructions: &Vec<Movement>,
    patterns: &Vec<Vec<Movement>>,
) -> Option<Vec<Vec<Movement>>> {
    // Stop when we found 3 patterns
    if patterns.len() == 3 {
        // Only return valid result if there are no remaining instructions
        if movement_instructions.is_empty() {
            return Some(patterns.iter().cloned().collect_vec());
        } else {
            return None;
        }
    }

    for i in 1..movement_instructions.len() {
        for j in 0..(movement_instructions.len() - i) {
            if movement_instructions[j] != movement_instructions[i + j] {
                break;
            }
            // For each repeating pattern:
            // 1. Remove the pattern from the instructions
            // 2. Try to find more patterns in the remaining instructions
            // 3. End when 3 patterns have been found
            if j > 0 {
                let pattern_length = j + 1;
                let remaining_instructions = remove_pattern(movement_instructions, pattern_length);
                let mut patterns = patterns.clone();
                patterns.push(
                    movement_instructions
                        .iter()
                        .cloned()
                        .take(pattern_length)
                        .collect_vec(),
                );

                // Continue searching for more patterns
                let result = find_repeating_instructions(&remaining_instructions, &patterns);
                if result.is_some() {
                    return result;
                }
            }
        }
    }
    None
}

fn remove_pattern(movement_instructions: &Vec<Movement>, pattern_length: usize) -> Vec<Movement> {
    let mut new_instructions = Vec::new();
    let mut i = pattern_length;
    while i < movement_instructions.len() {
        // For each index:
        // 1. Check if its part of the pattern
        // 2. If it is, skip the section
        // 3. If it's not, add it to the new instruction list
        if (movement_instructions.len() - i) >= pattern_length {
            let mut matches = true;
            for j in 0..pattern_length {
                if movement_instructions[j] != movement_instructions[i + j] {
                    matches = false;
                    break;
                }
            }
            if matches {
                i += pattern_length;
                continue;
            }
        }

        new_instructions.push(movement_instructions[i]);
        i += 1;
    }
    new_instructions
}

fn build_movement_instructions(map: &BTreeSet<Pos>, mut pos: Pos) -> Vec<Movement> {
    let mut path = Vec::new();
    let mut direction_forward = Direction::North;
    loop {
        // Move forward
        let mut pos_forward = direction_forward.update_pos(pos);
        if map.contains(&pos_forward) {
            // Keep going forward until it can't
            let mut steps = 0;
            while {
                steps += 1;
                pos = pos_forward;
                pos_forward = direction_forward.update_pos(pos);
                map.contains(&pos_forward)
            } {}
            path.push(Movement::Forward(steps));
        }

        let direction_left = match direction_forward {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };

        let direction_right = match direction_forward {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };

        // Turn left
        let pos_left = direction_left.update_pos(pos);
        if map.contains(&pos_left) {
            path.push(Movement::TurnLeft);
            direction_forward = direction_left;
            continue;
        }

        // Turn right
        let pos_right = direction_right.update_pos(pos);
        if map.contains(&pos_right) {
            path.push(Movement::TurnRight);
            direction_forward = direction_right;
            continue;
        }

        // End of path
        break path;
    }
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn update_pos(&self, pos: (i64, i64)) -> (i64, i64) {
        match *self {
            Direction::North => (pos.0 - 1, pos.1),
            Direction::East => (pos.0, pos.1 + 1),
            Direction::South => (pos.0 + 1, pos.1),
            Direction::West => (pos.0, pos.1 - 1),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Movement {
    Forward(i64),
    TurnLeft,
    TurnRight,
}

type Pos = (i64, i64);
