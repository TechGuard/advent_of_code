use anyhow::*;
use itertools::Itertools;

pub struct Program {
    data: Vec<i64>,
    input: Option<i64>,
    ip: usize,
    rel_base: i64,
}

pub enum Action {
    WaitingForInput,
    Output(i64),
    Halt,
}

pub fn parse_data(input: &str) -> Result<Vec<i64>> {
    input
        .lines()
        .next()
        .context("Expected at least one line")?
        .split(",")
        .map(|s| {
            s.parse()
                .with_context(|| anyhow!("Expected valid i64: {}", s))
        })
        .try_collect()
}

pub fn execute_until_end<Iter>(data: Vec<i64>, input: Iter) -> Result<Vec<i64>>
where
    Iter: IntoIterator<Item = i64>,
{
    let mut input = input.into_iter();
    let mut output = Vec::new();
    let mut program = Program::new(data);
    Ok(loop {
        match program.execute()? {
            Action::WaitingForInput => {
                program.give_input(input.next().context("Program requires more input")?);
            }
            Action::Output(value) => output.push(value),
            Action::Halt => break output,
        }
    })
}

impl Program {
    pub fn new(data: Vec<i64>) -> Program {
        Program {
            data,
            input: None,
            ip: 0,
            rel_base: 0,
        }
    }

    pub fn get_data(&self) -> &Vec<i64> {
        &self.data
    }

    pub fn get_data_mut(&mut self) -> &mut Vec<i64> {
        &mut self.data
    }

    pub fn give_input(&mut self, input: i64) {
        self.input = Some(input);
    }

    pub fn execute(&mut self) -> Result<Action> {
        Ok(loop {
            // Parse instruction
            let instruction = self.get_checked(self.ip)?.to_string();
            self.ip += 1;

            let opcode_begin = instruction.len().max(2) - 2;
            let opcode = instruction[opcode_begin..].parse::<i64>()?.try_into()?; // parse last 2 chars into opcode
            let mut instruction = instruction[..opcode_begin].chars().rev(); // read right to left

            // Process opcodes
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
                        break Action::WaitingForInput;
                    }
                }
                Opcode::Output => {
                    break Action::Output(self.get_param(&mut instruction)?);
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
                Opcode::Halt => break Action::Halt,
            };
        })
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
            ParameterMode::Immediate => bail!("Cannot use immediate mode for destination address"),
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
