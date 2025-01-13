use anyhow::*;
use std::collections::{BTreeMap, BTreeSet};

pub static DAY: u32 = 06;
pub static EXAMPLE_INPUT: &str = "\
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN
";

pub fn main(input: &str) -> Result<(usize, usize)> {
    let mut map: BTreeMap<&str, Object<'_>> = BTreeMap::new();
    for line in input.lines() {
        let mut pair = line.split(")");
        let lhs = pair.next().context("Invalid input")?;
        let rhs = pair.next().context("Invalid input")?;

        let parent = map.entry(lhs).or_insert(Object {
            parent: None,
            children: Vec::new(),
        });
        parent.children.push(rhs);

        let child = map.entry(rhs).or_insert(Object {
            parent: None,
            children: Vec::new(),
        });
        child.parent = Some(lhs);
    }
    Ok((ans1(&map)?, ans2(&map)?))
}

fn ans1(map: &BTreeMap<&str, Object>) -> Result<usize> {
    let mut stack = Vec::new();
    stack.push((map.get("COM").context("Missing COM")?, 0));

    let mut total = 0;
    while let Some((object, depth)) = stack.pop() {
        for child in object.children.iter() {
            stack.push((map.get(child).unwrap(), depth + 1));
        }
        total += depth;
    }
    Ok(total)
}

fn ans2(map: &BTreeMap<&str, Object>) -> Result<usize> {
    let origin = map.get("YOU").context("Missing YOU")?;
    let target = map.get("SAN").context("Missing SAN")?;

    let mut visited = BTreeSet::new();
    let mut stack = Vec::new();
    stack.push((origin, 0));

    while let Some((object, depth)) = stack.pop() {
        if object == target {
            return Ok(depth - 2);
        }

        if visited.contains(object) {
            continue;
        }
        visited.insert(object);

        for child in object.children.iter() {
            stack.push((map.get(child).unwrap(), depth + 1));
        }
        if let Some(parent) = object.parent {
            stack.push((map.get(parent).unwrap(), depth + 1));
        }
    }
    bail!("Unable to find target")
}

#[derive(PartialEq, Ord, PartialOrd, Eq)]
struct Object<'a> {
    parent: Option<&'a str>,
    children: Vec<&'a str>,
}
