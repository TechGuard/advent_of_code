use anyhow::*;
use itertools::Itertools;

pub static DAY: u32 = 05;
pub static EXAMPLE_INPUT: &str = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";

pub fn main(input: &str) -> Result<(String, String)> {
    let data: Vec<i64> = input
        .lines()
        .next()
        .context("Invalid input")?
        .split(",")
        .map(|s| s.parse())
        .try_collect()?;
    Ok((
        execute(data.clone(), 1)?.to_string(),
        execute(data, 5)?.to_string(),
    ))
}

fn execute(mut data: Vec<i64>, input: i64) -> Result<i64> {
    let mut ip = 0;
    let mut last_output = 0;
    loop {
        // Parse instruction
        let instruction = get_checked(&data, ip)?.to_string();
        ip += 1;

        let opcode_begin = instruction.len().max(2) - 2;
        let opcode = instruction[opcode_begin..].parse::<i64>()?.try_into()?; // parse last 2 chars into opcode
        let mut instruction = instruction[..opcode_begin].chars().rev(); // read right to left

        // Proces opcodes
        match opcode {
            Opcode::Add => {
                let lhs = get_param(&data, &mut instruction, &mut ip)?;
                let rhs = get_param(&data, &mut instruction, &mut ip)?;
                *get_dest_mut(&mut data, &mut ip)? = lhs + rhs;
            }
            Opcode::Multiply => {
                let lhs = get_param(&data, &mut instruction, &mut ip)?;
                let rhs = get_param(&data, &mut instruction, &mut ip)?;
                *get_dest_mut(&mut data, &mut ip)? = lhs * rhs;
            }
            Opcode::Input => {
                *get_dest_mut(&mut data, &mut ip)? = input;
            }
            Opcode::Output => {
                last_output = get_param(&data, &mut instruction, &mut ip)?;
            }
            Opcode::JumpNotZero => {
                let test = get_param(&data, &mut instruction, &mut ip)?;
                let jmp = get_param(&data, &mut instruction, &mut ip)?;
                if test != 0 {
                    ip = jmp.try_into()?;
                }
            }
            Opcode::JumpZero => {
                let test = get_param(&data, &mut instruction, &mut ip)?;
                let jmp = get_param(&data, &mut instruction, &mut ip)?;
                if test == 0 {
                    ip = jmp.try_into()?;
                }
            }
            Opcode::LessThan => {
                let lhs = get_param(&data, &mut instruction, &mut ip)?;
                let rhs = get_param(&data, &mut instruction, &mut ip)?;
                *get_dest_mut(&mut data, &mut ip)? = (lhs < rhs).into();
            }
            Opcode::Equals => {
                let lhs = get_param(&data, &mut instruction, &mut ip)?;
                let rhs = get_param(&data, &mut instruction, &mut ip)?;
                *get_dest_mut(&mut data, &mut ip)? = (lhs == rhs).into();
            }
            Opcode::Halt => return Ok(last_output),
        };
    }
}

#[derive(PartialEq)]
enum Opcode {
    Halt,
    Add,
    Multiply,
    Input,
    Output,
    JumpNotZero,
    JumpZero,
    LessThan,
    Equals,
}

impl TryFrom<i64> for Opcode {
    type Error = anyhow::Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::Add,
            2 => Self::Multiply,
            3 => Self::Input,
            4 => Self::Output,
            5 => Self::JumpNotZero,
            6 => Self::JumpZero,
            7 => Self::LessThan,
            8 => Self::Equals,
            99 => Self::Halt,
            _ => bail!("Invalid opcode: {}", value),
        })
    }
}

fn get_param<Iter>(vec: &Vec<i64>, instruction: &mut Iter, ip: &mut usize) -> Result<i64>
where
    Iter: Iterator<Item = char>,
{
    // Read parameter value at instruction pointer
    let param = get_checked(vec, *ip)?;
    *ip += 1;

    // Consume next instruction char and deduce instruction mode for parameter
    let immediate_mode = instruction.by_ref().next().unwrap_or('0') == '1';
    if immediate_mode {
        Ok(param)
    } else {
        get_checked(vec, param)
    }
}

fn get_dest_mut<'a>(vec: &'a mut Vec<i64>, ip: &mut usize) -> Result<&'a mut i64> {
    // Read destination value at instruction pointer
    let dest = get_checked(vec, *ip)?;
    *ip += 1;
    // Return mutable address at destination
    get_checked_mut(vec, dest)
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
