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

fn get_answer1(input: &str) -> i32 {
    let mut total_2 = 0;
    let mut total_3 = 0;

    for line in input.lines() {
        let mut map = HashMap::new();
        for c in line.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        // Count 2
        if map.values().any(|&x| x == 2) {
            total_2 += 1;
        }
        // Count 3
        if map.values().any(|&x| x == 3) {
            total_3 += 1;
        }
    }

    total_2 * total_3
}

fn get_answer2(input: &str) -> String {
    let lines: Vec<_> = input.lines().collect();

    for (offset, &line1) in lines.iter().enumerate() {
        for &line2 in lines.iter().skip(offset + 1) {
            // Find the one and only difference
            if let Some(diff_pos) = find_one_diff(line1, line2) {
                // Copy line1 and remove char at diff_pos
                let mut answer = line1.to_string();
                answer.remove(diff_pos);
                return answer;
            }
        }
    }
    String::new()
}

// Returns the index of the char index that is different. None if same or more than one differences.
fn find_one_diff(str_a: &str, str_b: &str) -> Option<usize> {
    let mut index = 0;
    let mut diff_pos = None;

    // Compare every char
    for (char_a, char_b) in str_a.chars().zip(str_b.chars()) {
        if char_a != char_b {
            // Return None if more than one difference
            match diff_pos {
                None => diff_pos = Some(index),
                _ => return None,
            }
        }
        index += 1;
    }
    diff_pos
}
