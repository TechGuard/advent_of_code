use anyhow::*;
use itertools::Itertools;

pub static DAY: u32 = 09;
pub static EXAMPLE_INPUT: &str = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";

pub fn main(input: &str) -> Result<(String, String)> {
    let data: Vec<i64> = input
        .lines()
        .next()
        .context("Invalid input")?
        .split(",")
        .map(|s| s.parse())
        .try_collect()?;
    Ok((
        execute_program(&data, 1)?.to_string(),
        execute_program(&data, 2)?.to_string(),
    ))
}

fn execute_program(data: &Vec<i64>, input: i64) -> Result<i64> {
    let mut output = 0;
    let mut program = Program::new(data.clone());
    program.give_input(input);
    Ok(loop {
        match program.execute()? {
            Action::WaitingForInput => bail!("Program should only ask for input once"),
            Action::Output(value) => output = value,
            Action::Halt => break output,
        }
    })
}

struct Program {
    data: Vec<i64>,
    input: Option<i64>,
    ip: usize,
    rel_base: i64,
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
            rel_base: 0,
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
                    *self.get_dest_mut(&mut instruction)? = lhs + rhs;
                }
                Opcode::Multiply => {
                    let lhs = self.get_param(&mut instruction)?;
                    let rhs: i64 = self.get_param(&mut instruction)?;
                    *self.get_dest_mut(&mut instruction)? = lhs * rhs;
                }
                Opcode::Input => {
                    if let Some(input) = self.input {
                        self.input = None;
                        *self.get_dest_mut(&mut instruction)? = input;
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
                    *self.get_dest_mut(&mut instruction)? = (lhs < rhs).into();
                }
                Opcode::Equals => {
                    let lhs = self.get_param(&mut instruction)?;
                    let rhs = self.get_param(&mut instruction)?;
                    *self.get_dest_mut(&mut instruction)? = (lhs == rhs).into();
                }
                Opcode::AdjustRelBase => {
                    self.rel_base += self.get_param(&mut instruction)?;
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

        // Consume next instruction char and find parameter mode
        let param_mode = instruction.by_ref().next().unwrap_or('0').try_into()?;
        match param_mode {
            ParameterMode::Position => self.get_checked(param),
            ParameterMode::Immediate => Ok(param),
            ParameterMode::Relative => self.get_checked(self.rel_base + param),
        }
    }

    fn get_dest_mut<Iter>(&mut self, instruction: &mut Iter) -> Result<&mut i64>
    where
        Iter: Iterator<Item = char>,
    {
        // Read destination value at instruction pointer
        let param = self.get_checked(self.ip)?;
        self.ip += 1;

        // Consume next instruction char and find parameter mode
        let param_mode = instruction.by_ref().next().unwrap_or('0').try_into()?;
        match param_mode {
            ParameterMode::Position => self.get_checked_mut(param),
            ParameterMode::Immediate => bail!("Cannot have immediate mode for destination address"),
            ParameterMode::Relative => self.get_checked_mut(self.rel_base + param),
        }
    }

    fn get_checked<I>(&mut self, index: I) -> Result<i64>
    where
        I: TryInto<usize> + std::fmt::Display,
        <I as TryInto<usize>>::Error: Into<Error>,
    {
        let index: usize = index.try_into().map_err(|e| e.into())?;
        // Grow size of data if it's trying to write out of bounds
        if index >= self.data.len() {
            self.data.resize(index + 1, 0);
        }
        // Index should always be valid
        unsafe { Ok(self.data.get_unchecked(index).clone()) }
    }

    fn get_checked_mut<I>(&mut self, index: I) -> Result<&mut i64>
    where
        I: TryInto<usize> + std::fmt::Display,
        <I as TryInto<usize>>::Error: Into<Error>,
    {
        let index: usize = index.try_into().map_err(|e| e.into())?;
        // Grow size of data if it's trying to write out of bounds
        if index >= self.data.len() {
            self.data.resize(index + 1, 0);
        }
        // Index should always be valid
        unsafe { Ok(self.data.get_unchecked_mut(index)) }
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
    AdjustRelBase,
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
            9 => Self::AdjustRelBase,
            99 => Self::Halt,
            _ => bail!("Invalid opcode: {}", value),
        })
    }
}

enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl TryFrom<char> for ParameterMode {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '0' => Self::Position,
            '1' => Self::Immediate,
            '2' => Self::Relative,
            _ => bail!("Invalid parameter mode: {}", value),
        })
    }
}
