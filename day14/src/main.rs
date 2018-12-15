use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Expected input");

    let recipes = input.parse::<usize>().unwrap();
    println!("1st answer = {}", get_answer1(recipes));
    println!("2nd answer = {}", get_answer2(recipes));
}

fn get_answer1(input: usize) -> String {
    let mut scoreboard = vec![3, 7];
    let mut elf_idx1 = 0;
    let mut elf_idx2 = 1;

    while scoreboard.len() < input + 10 {
        let new_recipe = scoreboard[elf_idx1] + scoreboard[elf_idx2];
        scoreboard.extend(new_recipe.to_string().bytes().map(|c| (c - b'0') as usize));

        elf_idx1 = (elf_idx1 + scoreboard[elf_idx1] + 1) % scoreboard.len();
        elf_idx2 = (elf_idx2 + scoreboard[elf_idx2] + 1) % scoreboard.len();
    }

    scoreboard
        .iter()
        .skip(input)
        .take(10)
        .map(|x| x.to_string())
        .collect::<String>()
}

fn get_answer2(input: usize) -> usize {
    let mut scoreboard = vec![3, 7];
    let mut elf_idx1 = 0;
    let mut elf_idx2 = 1;

    let input_sequence = input
        .to_string()
        .bytes()
        .map(|c| (c - b'0') as usize)
        .collect::<Vec<_>>();

    loop {
        let new_recipe = scoreboard[elf_idx1] + scoreboard[elf_idx2];
        for digit in new_recipe.to_string().bytes().map(|c| (c - b'0') as usize) {
            scoreboard.push(digit);

            if scoreboard.len() > input_sequence.len() {
                let slice = &scoreboard[scoreboard.len() - input_sequence.len()..];
                if input_sequence == slice {
                    return scoreboard.len() - input_sequence.len();
                }
            }
        }

        elf_idx1 = (elf_idx1 + scoreboard[elf_idx1] + 1) % scoreboard.len();
        elf_idx2 = (elf_idx2 + scoreboard[elf_idx2] + 1) % scoreboard.len();
    }
}
