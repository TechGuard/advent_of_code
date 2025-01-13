use crate::utils::intcode;
use anyhow::*;
use itertools::Itertools;

pub static DAY: u32 = 07;
pub static EXAMPLE_INPUT: &str =
    "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";

pub fn main(input: &str) -> Result<(i64, i64)> {
    let data = intcode::parse_data(input)?;
    Ok((highest_signal(&data, 0..5)?, highest_signal(&data, 5..10)?))
}

fn highest_signal<Iter: Iterator<Item = i64>>(
    data: &Vec<i64>,
    phase_settings: Iter,
) -> Result<i64> {
    let mut highest_signal = 0;
    for phase_settings in phase_settings.permutations(5) {
        highest_signal = run_thruster(data, phase_settings)?.max(highest_signal);
    }
    Ok(highest_signal)
}

fn run_thruster(data: &Vec<i64>, phase_settings: Vec<i64>) -> Result<i64> {
    use intcode::Action;

    // Create programs
    let mut programs = Vec::with_capacity(phase_settings.len());
    for phase in phase_settings {
        let mut program = intcode::Program::new(data.clone());
        program.give_input(phase);
        match program.execute()? {
            Action::WaitingForInput => programs.push(program),
            _ => bail!("Program ended too early"),
        };
    }

    // Execute programs until a program halts
    let mut prev_output = 0;
    let mut any_program_halted = false;

    while !any_program_halted {
        for program in programs.iter_mut() {
            // Calculate new output with previous output
            program.give_input(prev_output);
            match program.execute()? {
                Action::Output(output) => prev_output = output,
                _ => bail!("Expected output from program"),
            };
            // See if program wants more input or halt
            match program.execute()? {
                Action::WaitingForInput => {}
                Action::Halt => any_program_halted = true,
                _ => bail!("Program should not output twice in a row"),
            };
        }
    }

    Ok(prev_output)
}
