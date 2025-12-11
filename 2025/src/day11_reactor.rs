use anyhow::*;
use itertools::Itertools;
use std::collections::HashMap;

pub static DAY: u32 = 11;
pub static EXAMPLE_INPUT: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

pub fn main(input: &str) -> Result<(i64, i64)> {
    let mut outputs = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split_ascii_whitespace();
        let node = parts.next().context("missing first")?;
        let connections = parts.collect_vec();
        outputs.insert(node[..node.len() - 1].to_owned(), connections);
    }
    Ok((
        part1(&outputs, &mut HashMap::new(), "you"),
        part2(&outputs, &mut HashMap::new(), (false, false), "svr"),
    ))
}

fn part1(
    outputs: &HashMap<String, Vec<&str>>,
    cache: &mut HashMap<String, i64>,
    node: &str,
) -> i64 {
    if let Some(&cached) = cache.get(node) {
        return cached;
    }
    if node == "out" {
        return 1;
    }
    let mut paths = 0;
    if let Some(nodes) = outputs.get(node) {
        for next in nodes {
            paths += part1(outputs, cache, next);
        }
    }
    cache.insert(node.to_owned(), paths);
    paths
}

fn part2(
    outputs: &HashMap<String, Vec<&str>>,
    cache: &mut HashMap<(String, (bool, bool)), i64>,
    mut memory: (bool, bool),
    node: &str,
) -> i64 {
    if let Some(&cached) = cache.get(&(node.to_owned(), memory)) {
        return cached;
    }
    if node == "out" {
        return if memory == (true, true) { 1 } else { 0 };
    }
    if node == "dac" {
        memory.0 = true;
    }
    if node == "fft" {
        memory.1 = true;
    }
    let mut paths = 0;
    if let Some(nodes) = outputs.get(node) {
        for next in nodes {
            paths += part2(outputs, cache, memory, next);
        }
    }
    cache.insert((node.to_owned(), memory), paths);
    paths
}
