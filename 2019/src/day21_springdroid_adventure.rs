use crate::utils::intcode;
use anyhow::*;

pub static DAY: u32 = 21;
pub static EXAMPLE_INPUT: &str = "";

pub fn main(input: &str) -> Result<(i64, i64)> {
    let data = intcode::parse_data(input)?;
    let ans1 = {
        let mut input = Vec::new();
        // (!A || !B || !C) && D
        write_instruction(&mut input, "NOT A J"); // J  = !A  Hole 1 tile away
        write_instruction(&mut input, "NOT B T"); // T  = !B
        write_instruction(&mut input, "OR T J"); //  J |= T   Or hole 2 tiles away
        write_instruction(&mut input, "NOT C T"); // T  = !C
        write_instruction(&mut input, "OR T J"); //  J |= T   Or hole 3 tiles away
        write_instruction(&mut input, "AND D J"); // J &= D   And ground where droid lands

        write_instruction(&mut input, "WALK");
        let output = intcode::execute_until_end(data.clone(), input)?;
        *output.last().unwrap()
    };
    let ans2 = {
        let mut input = Vec::new();
        // X = (!A || !B || !C) && D
        write_instruction(&mut input, "NOT A J"); // J  = !A  Hole 1 tile away
        write_instruction(&mut input, "NOT B T"); // T  = !B
        write_instruction(&mut input, "OR T J"); //  J |= T   Or hole 2 tiles away
        write_instruction(&mut input, "NOT C T"); // T  = !C
        write_instruction(&mut input, "OR T J"); //  J |= T   Or hole 3 tiles away
        write_instruction(&mut input, "AND D J"); // J &= D   And ground where droid lands

        // Y = H || E
        write_instruction(&mut input, "NOT H T"); // T  = !H
        write_instruction(&mut input, "NOT T T"); // T  = !T  Can make another jump after landing
        write_instruction(&mut input, "OR E T"); //  T |= E   Or can take another step after landing

        // X && Y
        write_instruction(&mut input, "AND T J"); // J &= T   Test if both sections are true

        write_instruction(&mut input, "RUN");
        let output = intcode::execute_until_end(data, input)?;
        *output.last().unwrap()
    };
    Ok((ans1, ans2))
}

fn write_instruction(input: &mut Vec<i64>, instruction: &str) {
    for c in instruction.chars() {
        input.push(c as _);
    }
    input.push('\n' as _);
}
