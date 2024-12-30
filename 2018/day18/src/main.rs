use std::cmp::{max, min};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::{self, Read};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Acre {
    Open,
    Trees,
    Lumberyard,
}

impl From<char> for Acre {
    fn from(c: char) -> Self {
        match c {
            '.' => Acre::Open,
            '|' => Acre::Trees,
            '#' => Acre::Lumberyard,
            _ => unreachable!(),
        }
    }
}

impl From<&Acre> for char {
    fn from(acre: &Acre) -> Self {
        match acre {
            Acre::Open => '.',
            Acre::Trees => '|',
            Acre::Lumberyard => '#',
        }
    }
}

type AcreField = Vec<Vec<Acre>>;

fn progress(acres: &AcreField, new_acres: &mut AcreField) {
    let width = acres.iter().next().unwrap().len() as i32;
    let height = acres.len() as i32;

    // For every acre...
    for y in 0..height {
        for x in 0..width {
            // Count neighbours
            let mut count_trees = 0;
            let mut count_lumberyard = 0;
            for sy in max(0, y - 1)..min(height, y + 2) {
                for sx in max(0, x - 1)..min(width, x + 2) {
                    if !(sy == y && sx == x) {
                        match acres[sy as usize][sx as usize] {
                            Acre::Trees => count_trees += 1,
                            Acre::Lumberyard => count_lumberyard += 1,
                            _ => {}
                        };
                    }
                }
            }

            // Update acre
            new_acres[y as usize][x as usize] = match &acres[y as usize][x as usize] {
                Acre::Open => {
                    if count_trees >= 3 {
                        Acre::Trees
                    } else {
                        Acre::Open
                    }
                }
                Acre::Trees => {
                    if count_lumberyard >= 3 {
                        Acre::Lumberyard
                    } else {
                        Acre::Trees
                    }
                }
                Acre::Lumberyard => {
                    if count_lumberyard >= 1 && count_trees >= 1 {
                        Acre::Lumberyard
                    } else {
                        Acre::Open
                    }
                }
            };
        }
    }
}

fn count(acres: &AcreField) -> usize {
    let mut count_trees = 0;
    let mut count_lumberyard = 0;
    acres.iter().for_each(|row| {
        row.iter().for_each(|x| match x {
            Acre::Trees => count_trees += 1,
            Acre::Lumberyard => count_lumberyard += 1,
            _ => {}
        })
    });
    count_trees * count_lumberyard
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Expected input");

    let mut acres: AcreField = Vec::new();
    for s in input.lines() {
        acres.push(s.chars().map(|c| Acre::from(c)).collect());
    }

    println!("1st answer = {}", get_answer1(acres.clone()));
    println!("2nd answer = {}", get_answer2(acres));
}

fn get_answer1(mut acres: AcreField) -> usize {
    let mut new_acres = acres.clone();
    for _ in 0..10 {
        progress(&acres, &mut new_acres);
        std::mem::swap(&mut acres, &mut new_acres);
    }
    count(&acres)
}

fn get_answer2(mut acres: AcreField) -> usize {
    let mut new_acres = acres.clone();
    let mut hashes = Vec::<u64>::new();
    let mut repeat_count = 0;
    let mut last_index = 0;

    // Find looping pattern
    loop {
        progress(&acres, &mut new_acres);
        std::mem::swap(&mut acres, &mut new_acres);

        let mut s = DefaultHasher::new();
        acres.hash(&mut s);
        let hash = s.finish();

        if let Some(idx) = hashes.iter().position(|&x| x == hash) {
            repeat_count = last_index - idx;
            break;
        }

        hashes.push(hash);
        last_index += 1;
    }

    // Skip repeated patterns
    for _ in 0..(1000000000 - last_index) % repeat_count - 1 {
        progress(&acres, &mut new_acres);
        std::mem::swap(&mut acres, &mut new_acres);
    }

    // for row in acres {
    //     println!("{}", row.iter().map(|x| char::from(x)).collect::<String>());
    // }
    count(&acres)
}
