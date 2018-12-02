use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Expected input");

    let numbers: Vec<i32> = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    println!("1st Answer = {}", numbers.iter().sum::<i32>());
    println!("2nd Answer = {}", get_answer2(&numbers));
}

fn get_answer2(numbers: &Vec<i32>) -> i32 {
    let mut freq = 0;
    let mut frequencies = HashSet::new();

    loop {
        for number in numbers.iter() {
            freq += number;

            if frequencies.contains(&freq) {
                return freq;
            }

            frequencies.insert(freq);
        }
    }
}
