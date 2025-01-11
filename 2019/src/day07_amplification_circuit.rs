use anyhow::*;
use itertools::Itertools;

pub static DAY: u32 = 07;
pub static EXAMPLE_INPUT: &str =
    "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";

pub fn main(input: &str) -> Result<(String, String)> {
    let data: Vec<i64> = input
        .lines()
        .next()
        .context("Invalid input")?
        .split(",")
        .map(|s| s.parse())
        .try_collect()?;
    Ok((
        highest_signal(&data, 0..5)?.to_string(),
        highest_signal(&data, 5..10)?.to_string(),
    ))
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
    // Create programs
    let mut programs = Vec::with_capacity(phase_settings.len());
    for phase in phase_settings {
        let mut program = Program::new(data.clone());
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

struct Program {
    data: Vec<i64>,
    input: Option<i64>,
    ip: usize,
}

enum Action {
    WaitingForInput,
    Output(i64),
    Halt,
}

impl Program {
    fn new(data: Vec<i64>) -> Program {
        Program {
            data,
            input: None,
            ip: 0,
        }
    }

    fn give_input(&mut self, input: i64) {
        self.input = Some(input);
    }

    fn execute(&mut self) -> Result<Action> {
        loop {
            // Parse instruction
            let instruction = self.get_checked(self.ip)?.to_string();
            self.ip += 1;

            let opcode_begin = instruction.len().max(2) - 2;
            let opcode = instruction[opcode_begin..].parse::<i64>()?.try_into()?; // parse last 2 chars into opcode
            let mut instruction = instruction[..opcode_begin].chars().rev(); // read right to left

            // Proces opcodes
            match opcode {
                Opcode::Add => {
                    let lhs = self.get_param(&mut instruction)?;
                    let rhs = self.get_param(&mut instruction)?;
                    *self.get_dest_mut()? = lhs + rhs;
                }
                Opcode::Multiply => {
                    let lhs = self.get_param(&mut instruction)?;
                    let rhs: i64 = self.get_param(&mut instruction)?;
                    *self.get_dest_mut()? = lhs * rhs;
                }
                Opcode::Input => {
                    if let Some(input) = self.input {
                        self.input = None;
                        *self.get_dest_mut()? = input;
                    } else {
                        // Rewind instruction pointer and wait for new input
                        self.ip -= 1;
                        return Ok(Action::WaitingForInput);
                    }
                }
                Opcode::Output => {
                    return Ok(Action::Output(self.get_param(&mut instruction)?));
                }
                Opcode::JumpNotZero => {
                    let test = self.get_param(&mut instruction)?;
                    let jmp = self.get_param(&mut instruction)?;
                    if test != 0 {
                        self.ip = jmp.try_into()?;
                    }
                }
                Opcode::JumpZero => {
                    let test = self.get_param(&mut instruction)?;
                    let jmp = self.get_param(&mut instruction)?;
                    if test == 0 {
                        self.ip = jmp.try_into()?;
                    }
                }
                Opcode::LessThan => {
                    let lhs = self.get_param(&mut instruction)?;
                    let rhs = self.get_param(&mut instruction)?;
                    *self.get_dest_mut()? = (lhs < rhs).into();
                }
                Opcode::Equals => {
                    let lhs = self.get_param(&mut instruction)?;
                    let rhs = self.get_param(&mut instruction)?;
                    *self.get_dest_mut()? = (lhs == rhs).into();
                }
                Opcode::Halt => return Ok(Action::Halt),
            };
        }
    }

    fn get_param<Iter>(&mut self, instruction: &mut Iter) -> Result<i64>
    where
        Iter: Iterator<Item = char>,
    {
        // Read parameter value at instruction pointer
        let param = self.get_checked(self.ip)?;
        self.ip += 1;

        // Consume next instruction char and deduce instruction mode for parameter
        let immediate_mode = instruction.by_ref().next().unwrap_or('0') == '1';
        if immediate_mode {
            Ok(param)
        } else {
            self.get_checked(param)
        }
    }

    fn get_dest_mut(&mut self) -> Result<&mut i64> {
        // Read destination value at instruction pointer
        let dest = self.get_checked(self.ip)?;
        self.ip += 1;
        // Return mutable address at destination
        self.get_checked_mut(dest)
    }

    fn get_checked<I>(&mut self, index: I) -> Result<i64>
    where
        I: TryInto<usize> + std::fmt::Display,
        <I as TryInto<usize>>::Error: Into<Error>,
    {
        let index: usize = index.try_into().map_err(|e| e.into())?;
        Ok(self
            .data
            .get(index)
            .with_context(|| format!("Out of bounds: {}", index))?
            .clone())
    }

    fn get_checked_mut<I>(&mut self, index: I) -> Result<&mut i64>
    where
        I: TryInto<usize> + std::fmt::Display,
        <I as TryInto<usize>>::Error: Into<Error>,
    {
        let index: usize = index.try_into().map_err(|e| e.into())?;
        Ok(self
            .data
            .get_mut(index)
            .with_context(|| format!("Out of bounds: {}", index))?)
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
