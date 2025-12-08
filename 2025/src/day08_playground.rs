use anyhow::*;
use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap};
use vecmath::*;

pub static DAY: u32 = 08;
pub static EXAMPLE_INPUT: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

type Vec3 = Vector3<f64>;

pub fn main(input: &str) -> Result<(usize, f64)> {
    let boxes = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            Ok([
                parts.next().context("invalid input")?.parse()?,
                parts.next().context("invalid input")?.parse()?,
                parts.next().context("invalid input")?.parse()?,
            ])
        })
        .collect::<Result<Vec<Vec3>>>()?;

    let mut pairs = Vec::with_capacity(boxes.len() * boxes.len());
    for i in 0..(boxes.len() - 1) {
        for j in (i + 1)..boxes.len() {
            let diff = vec3_sub(boxes[j], boxes[i]);
            let dist = vec3_len(diff);
            pairs.push((dist, (i, j)));
        }
    }
    pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));

    let mut edges = HashMap::<usize, Vec<usize>>::new();
    let mut part1_count = if input == EXAMPLE_INPUT { 10 } else { 1000 };

    let mut result1 = 0;
    let mut result2 = 0.0;

    for (_, (i, j)) in pairs {
        edges.entry(i).or_default().push(j);
        edges.entry(j).or_default().push(i);

        let groups = count_groups(edges.clone());

        part1_count -= 1;
        if part1_count == 0 {
            result1 = groups;
        } else if groups == boxes.len() {
            result2 = boxes[i][0] * boxes[j][0];
            break;
        }
    }

    Ok((result1, result2))
}

fn count_groups(mut edges: HashMap<usize, Vec<usize>>) -> usize {
    let mut groups = Vec::new();

    while !edges.is_empty() {
        let mut stack = vec![*edges.keys().next().unwrap()];
        let mut count = 0;
        while let Some(i) = stack.pop() {
            if let Some(mut nodes) = edges.remove(&i) {
                stack.append(&mut nodes);
                count += 1;
            }
        }
        groups.push(count);
    }

    groups.iter().sorted().rev().take(3).product()
}
