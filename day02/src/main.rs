use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Expected input");

    println!("1st Answer = {}", get_answer1(&input));
    println!("2st Answer = {}", get_answer2(&input));
}

fn get_answer1(input: &String) -> i32 {
    let mut total_2 = 0;
    let mut total_3 = 0;

    for line in input.lines() {
        let mut map = HashMap::new();
        for c in line.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        // Count 2
        for (_, i) in &map {
            if *i == 2 {
                total_2 += 1;
                break;
            }
        }
        // Count 3
        for (_, i) in &map {
            if *i == 3 {
                total_3 += 1;
                break;
            }
        }
    }

    total_2 * total_3
}

fn get_answer2(input: &String) -> String {
    let lines: Vec<_> = input.lines().collect();
    let mut answer = String::new();

    for (offset, line1) in lines.iter().enumerate() {
        let chars1: Vec<_> = line1.chars().collect();

        'line2: for line2 in lines.iter().skip(offset + 1) {
            let mut chars2 = line2.chars();
            let mut diff_pos: Option<usize> = None;

            for (i, c) in chars1.iter().enumerate() {
                if chars2.next().unwrap() != *c {
                    if diff_pos.is_some() {
                        // more than one change
                        continue 'line2;
                    }
                    diff_pos = Some(i);
                }
            }

            if diff_pos.is_some() {
                answer = line1.to_string();
                answer.remove(diff_pos.unwrap());
            }
        }
    }
    answer
}
