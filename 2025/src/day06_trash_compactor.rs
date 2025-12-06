use anyhow::*;
use itertools::Itertools;

pub static DAY: u32 = 06;
pub static EXAMPLE_INPUT: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

struct Problem {
    numbers: Vec<i64>,
    operator: Operator,
}

pub fn main(input: &str) -> Result<(i64, i64)> {
    Ok((part1(input)?, part2(input)?))
}

fn solve(problems: Vec<Problem>) -> i64 {
    let mut result = 0;
    for problem in problems {
        let solution = match problem.operator {
            Operator::Add => problem.numbers.iter().fold(0, |acc, x| acc + x),
            Operator::Multiply => problem.numbers.iter().fold(1, |acc, x| acc * x),
        };
        result += solution;
    }
    result
}

fn parse_input(input: &str) -> Result<(Vec<&str>, Vec<Operator>)> {
    let mut lines = input.lines();
    let count = lines.clone().count() - 1;

    let input = lines.by_ref().take(count).collect_vec();

    let operators = lines
        .next()
        .context("missing operators")?
        .split_ascii_whitespace()
        .map(|str| match str {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Multiply),
            _ => bail!("invalid operator"),
        })
        .collect::<Result<Vec<Operator>>>()?;

    Ok((input, operators))
}

fn part1(input: &str) -> Result<i64> {
    let (input, operators) = parse_input(input)?;

    let input = input
        .into_iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|str| Ok(str.parse()?))
                .collect()
        })
        .collect::<Result<Vec<Vec<i64>>>>()?;

    let columns = input.first().context("invalid input")?.len();
    let rows = input.len();

    let mut operators = operators.iter();
    let mut problems = Vec::with_capacity(columns);

    for x in 0..columns {
        let mut numbers = Vec::with_capacity(rows);
        for y in 0..rows {
            numbers.push(input[y][x]);
        }
        problems.push(Problem {
            numbers,
            operator: *operators.next().context("not enough operators")?,
        });
    }

    Ok(solve(problems))
}

fn part2(input: &str) -> Result<i64> {
    let (input, operators) = parse_input(input)?;

    let input = input
        .into_iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let input_width = input
        .iter()
        .map(|line| line.len())
        .max()
        .context("invalid input")?;

    let rows = input.len();
    let mut cursor = input_width;

    let mut problems = Vec::new();
    let mut operators = operators.iter().rev();

    while cursor > 0 {
        let mut numbers = Vec::new();
        while cursor > 0 {
            cursor -= 1;

            // Read each row right to left and build number
            let mut number = 0;
            for y in 0..rows {
                if let Some(c) = input[y].get(cursor) {
                    if let Some(digit) = c.to_digit(10) {
                        number *= 10;
                        number += digit as i64;
                    }
                }
            }

            if number == 0 {
                // No number found in any of the rows, done parsing this column
                break;
            }

            numbers.push(number);
        }

        // If any numbers were found, add to problem list
        if !numbers.is_empty() {
            problems.push(Problem {
                numbers,
                operator: *operators.next().context("not enough operators")?,
            });
        }
    }

    Ok(solve(problems))
}
