use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Read};

struct Claim {
    id: i32,
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Expected input");

    // Parse claims
    let claims: Vec<Claim> = input
        .lines()
        .map(|line: &str| {
            let id: i32 = line[1..line.find(' ').unwrap()].parse().unwrap();
            let left: i32 = line[line.find('@').unwrap() + 2..line.find(',').unwrap()]
                .parse()
                .unwrap();
            let top: i32 = line[line.find(',').unwrap() + 1..line.find(':').unwrap()]
                .parse()
                .unwrap();
            let width: i32 = line[line.find(':').unwrap() + 2..line.find('x').unwrap()]
                .parse()
                .unwrap();
            let height: i32 = line[line.find('x').unwrap() + 1..].parse().unwrap();
            Claim {
                id,
                left,
                top,
                width,
                height,
            }
        })
        .collect();

    let mut potential = HashSet::<i32>::new();
    let mut fabric = HashMap::<i32, Vec<i32>>::new();

    // Fill fabric
    claims.iter().for_each(|claim: &Claim| {
        let mut overlaps = false;

        for y in claim.top..claim.top + claim.height {
            for x in claim.left..claim.left + claim.width {
                let entry = fabric.entry(y * 1000 + x).or_default();

                // Remove potential claims
                if !entry.is_empty() {
                    overlaps = true;
                    entry.iter().for_each(|id| {
                        potential.remove(id);
                    });
                }
                entry.push(claim.id);
            }
        }

        // Add to potential if it didn't overlap
        if !overlaps {
            potential.insert(claim.id);
        }
    });

    let answer_1 = fabric.values().filter(|f| f.len() > 1).count();
    let answer_2 = *potential.iter().next().unwrap();

    println!("1st Answer = {}", answer_1);
    println!("2st Answer = {}", answer_2);
}
