use std::io::{self, Read};

extern crate regex;
use regex::Regex;

#[derive(Debug)]
enum Opcode {
    AddR,
    AddI,
    SetR,
    SetI,
    MulR,
    MulI,
    EqRR,
    GtRR,
}

impl From<&str> for Opcode {
    fn from(s: &str) -> Self {
        match s {
            "addr" => Opcode::AddR,
            "addi" => Opcode::AddI,
            "setr" => Opcode::SetR,
            "seti" => Opcode::SetI,
            "mulr" => Opcode::MulR,
            "muli" => Opcode::MulI,
            "eqrr" => Opcode::EqRR,
            "gtrr" => Opcode::GtRR,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    data: [usize; 3],
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
            Instruction {
                opcode: Opcode::from(&capture[1]),
                data: [
                    capture[2].parse().unwrap(),
                    capture[3].parse().unwrap(),
                    capture[4].parse().unwrap(),
                ],
            }
        })
        .collect();
    (ip_register, instructions)
}

// Refactor later >.<
fn find_solution(ip_register: usize, instructions: &Vec<Instruction>, part2: bool) -> usize {
    let mut registers = vec![0; 6];
    let mut ip = 0;

    registers[0] = if part2 { 1 } else { 0 };

    for _ in 0..20 {
        let instruction = &instructions[ip];
        registers[ip_register] = ip;

        match instruction.opcode {
            Opcode::AddR => {
                registers[instruction.data[2]] =
                    registers[instruction.data[0]] + registers[instruction.data[1]];
            }
            Opcode::AddI => {
                registers[instruction.data[2]] =
                    registers[instruction.data[0]] + instruction.data[1];
            }
            Opcode::SetR => {
                registers[instruction.data[2]] = registers[instruction.data[0]];
            }
            Opcode::SetI => {
                registers[instruction.data[2]] = instruction.data[0];
            }
            Opcode::MulR => {
                registers[instruction.data[2]] =
                    registers[instruction.data[0]] * registers[instruction.data[1]];
            }
            Opcode::MulI => {
                registers[instruction.data[2]] =
                    registers[instruction.data[0]] * instruction.data[1];
            }
            Opcode::EqRR => {
                registers[instruction.data[2]] =
                    if registers[instruction.data[0]] == registers[instruction.data[1]] {
                        1
                    } else {
                        0
                    };
            }
            Opcode::GtRR => {
                registers[instruction.data[2]] =
                    if registers[instruction.data[0]] > registers[instruction.data[1]] {
                        1
                    } else {
                        0
                    };
            }
        };

        ip = registers[ip_register];
        ip += 1;
    }

    let num = registers.iter().max().unwrap();
    let mut result = 0;
    for i in 1..num + 1 {
        if num % i == 0 {
            result += i;
        }
    }
    result
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Expected input");

    let (ip_register, instructions) = parse_input(&input);

    println!(
        "1st answer = {}",
        find_solution(ip_register, &instructions, false)
    );
    println!(
        "2nd answer = {}",
        find_solution(ip_register, &instructions, true)
    );
}
