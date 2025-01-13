use crate::utils::intcode;
use anyhow::*;
use itertools::Itertools;

pub static DAY: u32 = 09;
pub static EXAMPLE_INPUT: &str = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";

pub fn main(input: &str) -> Result<(String, String)> {
    let data = intcode::parse_data(input)?;
    Ok((
        intcode::execute_until_end(data.clone(), [1])?
            .iter()
            .format(",")
            .to_string(),
        intcode::execute_until_end(data, [2])?
            .iter()
            .format(",")
            .to_string(),
    ))
}
