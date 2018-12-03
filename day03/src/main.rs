use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Read};

extern crate regex;
use regex::Regex;

#[derive(Debug)]
struct Claim {
    id: i32,
    top: i32,
    left: i32,
    width: i32,
    height: i32,
}

fn parse_claims(input: &String) -> Result<Vec<Claim>, &'static str> {
    let parse_error = Err("parse error");
    let re =
        Regex::new(r"^#(?P<id>\d+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)x(?P<height>\d+)$")
            .unwrap();
    input
        .lines()
        .map(|line| {
            let capture = re.captures(line).ok_or("invalid format")?;
            Ok(Claim {
                id: capture["id"].parse().or(parse_error)?,
                top: capture["top"].parse().or(parse_error)?,
                left: capture["left"].parse().or(parse_error)?,
                width: capture["width"].parse().or(parse_error)?,
                height: capture["height"].parse().or(parse_error)?,
            })
        })
        .collect()
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Expected input");

    let claims = parse_claims(&input).unwrap();

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
