use core::str::FromStr;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

pub static DAY: u32 = 3;
pub static EXAMPLE_INPUT: &str = "";

#[derive(Debug)]
struct Claim {
    id: i32,
    top: i32,
    left: i32,
    width: i32,
    height: i32,
}

impl FromStr for Claim {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^#(?P<id>\d+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)x(?P<height>\d+)$"
            )
            .unwrap();
        }
        let capture = RE.captures(s).unwrap();
        Ok(Claim {
            id: capture["id"].parse()?,
            top: capture["top"].parse()?,
            left: capture["left"].parse()?,
            width: capture["width"].parse()?,
            height: capture["height"].parse()?,
        })
    }
}

pub fn main(input: &str) -> (usize, i32) {
    let claims = input
        .lines()
        .map(Claim::from_str)
        .collect::<Result<Vec<Claim>, _>>()
        .unwrap();

    let mut potential = HashSet::new();
    let mut fabric = HashMap::<_, Vec<_>>::new();

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

    (
        fabric.values().filter(|f| f.len() > 1).count(),
        *potential.iter().next().unwrap(),
    )
}
