use crate::utils::intcode;
use anyhow::*;

pub static DAY: u32 = 05;
pub static EXAMPLE_INPUT: &str = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";

pub fn main(input: &str) -> Result<(i64, i64)> {
    let data = intcode::parse_data(input)?;
    Ok((
        intcode::execute_until_end(data.clone(), [1])
            .map(|output| output.last().cloned().unwrap_or_default())?,
        intcode::execute_until_end(data, [5])
            .map(|output| output.last().cloned().unwrap_or_default())?,
    ))
}
