use anyhow::*;
use itertools::Itertools;

pub static DAY: u32 = 02;
pub static EXAMPLE_INPUT: &str = "1,9,10,3,2,3,11,0,99,30,40,50";

pub fn main(input: &str) -> Result<(String, String)> {
    let data: Vec<usize> = input
        .lines()
        .next()
        .context("Invalid input")?
        .split(",")
        .map(|s| s.parse())
        .try_collect()?;
    Ok((
        execute(data.clone(), 12, 2)?.to_string(),
        ans2(data)?.to_string(),
    ))
}

fn execute(mut data: Vec<usize>, noun: usize, verb: usize) -> Result<usize> {
    let mut ip = 0;
    *get_checked_mut(&mut data, 1)? = noun;
    *get_checked_mut(&mut data, 2)? = verb;
    loop {
        let opcode = get_checked(&data, ip)?;
        if opcode == 99 {
            return Ok(get_checked(&data, 0)?);
        }
        let lhs = get_checked(&data, get_checked(&data, ip + 1)?)?;
        let rhs = get_checked(&data, get_checked(&data, ip + 2)?)?;
        let dest = get_checked(&data, ip + 3)?;
        match opcode {
            1 => *get_checked_mut(&mut data, dest)? = lhs + rhs,
            2 => *get_checked_mut(&mut data, dest)? = lhs * rhs,
            _ => bail!("Invalid opcode: {}", opcode),
        };
        ip += 4;
    }
}

fn ans2(data: Vec<usize>) -> Result<usize> {
    for noun in 0..100 {
        for verb in 0..100 {
            if execute(data.clone(), noun, verb)? == 19690720 {
                return Ok(100 * noun + verb);
            }
        }
    }
    bail!("Cannot find answer")
}

fn get_checked<T, I>(vec: &Vec<T>, index: I) -> Result<T>
where
    T: Clone,
    I: TryInto<usize> + std::fmt::Display,
    <I as TryInto<usize>>::Error: Into<Error>,
{
    let index: usize = index.try_into().map_err(|e| e.into())?;
    Ok(vec
        .get(index)
        .with_context(|| format!("Out of bounds: {}", index))?
        .clone())
}

fn get_checked_mut<T, I>(vec: &mut Vec<T>, index: I) -> Result<&mut T>
where
    I: TryInto<usize> + std::fmt::Display,
    <I as TryInto<usize>>::Error: Into<Error>,
{
    let index: usize = index.try_into().map_err(|e| e.into())?;
    Ok(vec
        .get_mut(index)
        .with_context(|| format!("Out of bounds: {}", index))?)
}
