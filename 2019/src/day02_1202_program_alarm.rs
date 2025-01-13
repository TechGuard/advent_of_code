use crate::utils::intcode;
use anyhow::*;

pub static DAY: u32 = 02;
pub static EXAMPLE_INPUT: &str = "1,9,10,3,2,3,11,0,99,30,40,50";

pub fn main(input: &str) -> Result<(i64, i64)> {
    let data = intcode::parse_data(input)?;
    Ok((execute(&data, 12, 2)?, ans2(&data)?))
}

fn execute(data: &Vec<i64>, noun: i64, verb: i64) -> Result<i64> {
    use intcode::Action;
    if data.len() <= 2 {
        bail!("Expected data length > 2");
    }

    let mut program = intcode::Program::new(data.clone());
    program.get_data_mut()[1] = noun;
    program.get_data_mut()[2] = verb;
    Ok(loop {
        match program.execute()? {
            Action::WaitingForInput => bail!("Program should not ask for input"),
            Action::Output(_) => bail!("Program should not produce an output"),
            Action::Halt => break program.get_data()[0],
        }
    })
}

fn ans2(data: &Vec<i64>) -> Result<i64> {
    for noun in 0..100 {
        for verb in 0..100 {
            if execute(data, noun, verb)? == 19690720 {
                return Ok(100 * noun + verb);
            }
        }
    }
    bail!("Cannot find answer")
}
