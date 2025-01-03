use std::collections::HashMap;

pub static DAY: u32 = 16;
pub static EXAMPLE_INPUT: &str = "";

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum Opcode {
    addr,
    addi,
    mulr,
    muli,
    banr,
    bani,
    borr,
    bori,
    setr,
    seti,
    gtir,
    gtri,
    gtrr,
    eqir,
    eqri,
    eqrr,
}

impl Opcode {
    pub fn iterator() -> std::slice::Iter<'static, Opcode> {
        static OPCODES: [Opcode; 16] = [
            Opcode::addr,
            Opcode::addi,
            Opcode::mulr,
            Opcode::muli,
            Opcode::banr,
            Opcode::bani,
            Opcode::borr,
            Opcode::bori,
            Opcode::setr,
            Opcode::seti,
            Opcode::gtir,
            Opcode::gtri,
            Opcode::gtrr,
            Opcode::eqir,
            Opcode::eqri,
            Opcode::eqrr,
        ];
        OPCODES.iter()
    }

    #[allow(non_snake_case)]
    fn execute(opcode: Opcode, input: &Vec<u32>, A: u32, B: u32, C: u32) -> Vec<u32> {
        let mut output = input.clone();
        match opcode {
            Opcode::addr => {
                output[C as usize] = input[A as usize] + input[B as usize];
            }
            Opcode::addi => {
                output[C as usize] = input[A as usize] + B;
            }
            Opcode::mulr => {
                output[C as usize] = input[A as usize] * input[B as usize];
            }
            Opcode::muli => {
                output[C as usize] = input[A as usize] * B;
            }
            Opcode::banr => {
                output[C as usize] = input[A as usize] & input[B as usize];
            }
            Opcode::bani => {
                output[C as usize] = input[A as usize] & B;
            }
            Opcode::borr => {
                output[C as usize] = input[A as usize] | input[B as usize];
            }
            Opcode::bori => {
                output[C as usize] = input[A as usize] | B;
            }
            Opcode::setr => {
                output[C as usize] = input[A as usize];
            }
            Opcode::seti => {
                output[C as usize] = A;
            }
            Opcode::gtir => {
                output[C as usize] = if A > input[B as usize] { 1 } else { 0 };
            }
            Opcode::gtri => {
                output[C as usize] = if input[A as usize] > B { 1 } else { 0 };
            }
            Opcode::gtrr => {
                output[C as usize] = if input[A as usize] > input[B as usize] {
                    1
                } else {
                    0
                };
            }
            Opcode::eqir => {
                output[C as usize] = if A == input[B as usize] { 1 } else { 0 };
            }
            Opcode::eqri => {
                output[C as usize] = if input[A as usize] == B { 1 } else { 0 };
            }
            Opcode::eqrr => {
                output[C as usize] = if input[A as usize] == input[B as usize] {
                    1
                } else {
                    0
                };
            }
        }
        output
    }
}

#[allow(non_snake_case)]
#[derive(Debug)]
struct Sample {
    before: Vec<u32>,
    opcode: u32,
    A: u32,
    B: u32,
    C: u32,
    after: Vec<u32>,
}

impl Sample {
    fn find_matching_opcodes(&self) -> Vec<Opcode> {
        let mut opcodes = Vec::new();
        for &opcode in Opcode::iterator() {
            let output = Opcode::execute(opcode, &self.before, self.A, self.B, self.C);
            if output == self.after {
                opcodes.push(opcode);
            }
        }
        opcodes
    }
}

fn parse_input(s: &str) -> (Vec<Sample>, Vec<Vec<u32>>) {
    let mut samples = Vec::new();
    let mut lines = s.lines();

    loop {
        let before = lines.next().unwrap();
        if before.is_empty() {
            break;
        }

        let instruction: Vec<_> = lines
            .next()
            .unwrap()
            .split(' ')
            .map(|c| c.parse().unwrap())
            .collect();

        let after = lines.next().unwrap();

        fn get_digits(s: &str) -> Vec<u32> {
            s[s.find('[').unwrap() + 1..s.len() - 1]
                .split(", ")
                .map(|c| c.parse().unwrap())
                .collect()
        }

        samples.push(Sample {
            before: get_digits(before),
            opcode: instruction[0],
            A: instruction[1],
            B: instruction[2],
            C: instruction[3],
            after: get_digits(after),
        });

        // Skip empty line
        lines.next();
    }

    // Skip empty line
    lines.next();

    let mut instructions = Vec::new();
    loop {
        match lines.next() {
            Some(s) => {
                instructions.push(s.split(' ').map(|c| c.parse().unwrap()).collect());
            }
            None => break,
        }
    }

    (samples, instructions)
}

pub fn main(input: &str) -> (usize, u32) {
    let (samples, instructions) = parse_input(&input);
    (get_answer1(&samples), get_answer2(&samples, &instructions))
}

fn get_answer1(samples: &Vec<Sample>) -> usize {
    let mut total = 0;
    for sample in samples {
        let opcodes = sample.find_matching_opcodes();
        if opcodes.len() >= 3 {
            total += 1;
        }
    }
    total
}

fn get_answer2(samples: &Vec<Sample>, instructions: &Vec<Vec<u32>>) -> u32 {
    let mut opcode_matches = HashMap::<u32, Vec<Opcode>>::new();

    // Find matching opcodes and only keep the ones that matched in every sample
    for sample in samples {
        let matches = sample.find_matching_opcodes();
        let entry = opcode_matches
            .entry(sample.opcode)
            .or_insert(matches.clone());
        entry.retain(|x| matches.contains(x));
    }

    // Begin mapping opcodes
    let mut resolved = Vec::<Opcode>::new();
    let mut opcodes = HashMap::<u32, Opcode>::new();

    while resolved.len() < 16 {
        // Find opcode with only one match and add to resolved list
        for (idx, matches) in opcode_matches.iter().filter(|(_, x)| x.len() == 1) {
            let opcode = *matches.first().unwrap();
            opcodes.insert(*idx, opcode);
            resolved.push(opcode);
        }
        // Remove resolved opcodes from match list
        for (_, matches) in opcode_matches.iter_mut() {
            matches.retain(|x| !resolved.contains(x));
        }
    }

    // Run program with resolved opcodes
    let mut registers = vec![0u32; 4];
    for instruction in instructions {
        registers = Opcode::execute(
            opcodes[&instruction[0]],
            &registers,
            instruction[1],
            instruction[2],
            instruction[3],
        );
    }

    // Return answer
    registers[0]
}
