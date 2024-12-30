use std::collections::HashSet;
use std::io::{self, Read};

extern crate regex;
use regex::Regex;

#[derive(Debug)]
struct Register(usize);
#[derive(Debug)]
struct Immediate(usize);

#[derive(Debug)]
enum Opcode {
    SetR(Register),
    SetI(Immediate),
    AddR(Register, Register),
    AddI(Register, Immediate),
    MulR(Register, Register),
    MulI(Register, Immediate),
    EqRR(Register, Register),
    EqRI(Register, Immediate),
    GtRR(Register, Register),
    GtIR(Immediate, Register),
    BanR(Register, Register),
    BanI(Register, Immediate),
    BorR(Register, Register),
    BorI(Register, Immediate),
}

impl Register {
    fn resolve(&self, registers: &Vec<usize>) -> usize {
        registers[self.0 as usize]
    }
}

impl Immediate {
    fn resolve(&self, _registers: &Vec<usize>) -> usize {
        self.0
    }
}

impl Opcode {
    fn execute(&self, registers: &Vec<usize>) -> usize {
        fn comparison(result: bool) -> usize {
            if result {
                1
            } else {
                0
            }
        };
        match self {
            Opcode::SetR(a) => a.resolve(registers),
            Opcode::SetI(a) => a.resolve(registers),
            Opcode::AddR(a, b) => a.resolve(registers) + b.resolve(registers),
            Opcode::AddI(a, b) => a.resolve(registers) + b.resolve(registers),
            Opcode::MulR(a, b) => a.resolve(registers) * b.resolve(registers),
            Opcode::MulI(a, b) => a.resolve(registers) * b.resolve(registers),
            Opcode::BanR(a, b) => a.resolve(registers) & b.resolve(registers),
            Opcode::BanI(a, b) => a.resolve(registers) & b.resolve(registers),
            Opcode::BorR(a, b) => a.resolve(registers) | b.resolve(registers),
            Opcode::BorI(a, b) => a.resolve(registers) | b.resolve(registers),
            Opcode::EqRR(a, b) => comparison(a.resolve(registers) == b.resolve(registers)),
            Opcode::EqRI(a, b) => comparison(a.resolve(registers) == b.resolve(registers)),
            Opcode::GtRR(a, b) => comparison(a.resolve(registers) > b.resolve(registers)),
            Opcode::GtIR(a, b) => comparison(a.resolve(registers) > b.resolve(registers)),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    output: usize,
}

fn parse_input(s: &str) -> (usize, Vec<Instruction>) {
    let mut lines = s.lines();

    let re = Regex::new(r"^#ip (\d+)$").unwrap();
    let ip_register: usize = re.captures(lines.next().unwrap()).unwrap()[1]
        .parse()
        .unwrap();

    let re = Regex::new(r"^([^\s]+) (\d+) (\d+) (\d+)$").unwrap();
    let instructions = lines
        .map(|s| {
            let capture = re.captures(s).unwrap();
            let a = capture[2].parse().unwrap();
            let b = capture[3].parse().unwrap();
            Instruction {
                opcode: match &capture[1] {
                    "setr" => Opcode::SetR(Register(a)),
                    "seti" => Opcode::SetI(Immediate(a)),
                    "addr" => Opcode::AddR(Register(a), Register(b)),
                    "addi" => Opcode::AddI(Register(a), Immediate(b)),
                    "mulr" => Opcode::MulR(Register(a), Register(b)),
                    "muli" => Opcode::MulI(Register(a), Immediate(b)),
                    "eqrr" => Opcode::EqRR(Register(a), Register(b)),
                    "eqri" => Opcode::EqRI(Register(a), Immediate(b)),
                    "gtrr" => Opcode::GtRR(Register(a), Register(b)),
                    "gtir" => Opcode::GtIR(Immediate(a), Register(b)),
                    "banr" => Opcode::BanR(Register(a), Register(b)),
                    "bani" => Opcode::BanI(Register(a), Immediate(b)),
                    "borr" => Opcode::BorR(Register(a), Register(b)),
                    "bori" => Opcode::BorI(Register(a), Immediate(b)),
                    _ => unreachable!("{}", s),
                },
                output: capture[4].parse().unwrap(),
            }
        })
        .collect();
    (ip_register, instructions)
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Expected input");

    let (ip_register, instructions) = parse_input(&input);
    let mut registers = vec![0; 6];
    let mut ip = 0;

    let mut seen = HashSet::new();
    let mut last_number = 0;

    loop {
        if ip >= instructions.len() {
            break;
        }

        registers[ip_register] = ip;

        let instruction = &instructions[ip];
        registers[instruction.output] = instruction.opcode.execute(&registers);

        if ip == 28 {
            let number = registers[2];
            if seen.is_empty() {
                println!("1st answer = {}", number);
            }
            if !seen.insert(number) {
                println!("2nd answer = {}", last_number);
                break;
            }
            last_number = number;
        }

        ip = registers[ip_register];
        ip += 1;
    }
}
