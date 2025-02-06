use anyhow::*;
use itertools::Itertools;
use pathfinding::prelude as pathfinding;
use std::{
    collections::{BTreeMap, BTreeSet},
    hash::{DefaultHasher, Hash, Hasher},
};

pub static DAY: u32 = 18;
pub static EXAMPLE_INPUT: &str = "\
#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#Ab#
#######
";

pub fn main(input: &str) -> Result<(usize, usize)> {
    // Parse input
    let mut tiles = BTreeSet::new();
    let mut keys = BTreeMap::new();
    let mut doors = BTreeMap::new();
    let mut start_pos = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                continue;
            }

            let tile = (y, x);
            tiles.insert(tile);

            if c == '@' {
                start_pos = tile;
            } else if c != '.' {
                if c.is_uppercase() {
                    doors.insert(c.to_lowercase().next().unwrap(), tile);
                } else {
                    keys.insert(tile, c);
                }
            }
        }
    }

    println!("This can take up to 8 minutes .. ☕");

    let mut cache = BTreeMap::new();
    let ans1 = find_shortest_path(&mut cache, &tiles, &vec![start_pos], &keys, &doors);

    // Split into four separate sections
    tiles.remove(&start_pos);
    tiles.remove(&(start_pos.0 - 1, start_pos.1));
    tiles.remove(&(start_pos.0 + 1, start_pos.1));
    tiles.remove(&(start_pos.0, start_pos.1 - 1));
    tiles.remove(&(start_pos.0, start_pos.1 + 1));

    // Try again with four different start positions
    cache.clear();
    let ans2 = find_shortest_path(
        &mut cache,
        &tiles,
        &vec![
            (start_pos.0 - 1, start_pos.1 - 1),
            (start_pos.0 - 1, start_pos.1 + 1),
            (start_pos.0 + 1, start_pos.1 + 1),
            (start_pos.0 + 1, start_pos.1 - 1),
        ],
        &keys,
        &doors,
    );

    Ok((ans1, ans2))
}

fn find_shortest_path(
    cache: &mut BTreeMap<u64, usize>,
    tiles: &BTreeSet<Pos>,
    positions: &Vec<Pos>,
    keys: &BTreeMap<Pos, char>,
    doors: &BTreeMap<char, Pos>,
) -> usize {
    if keys.is_empty() {
        return 0;
    }

    // Check if it's already cached
    let mut hasher = DefaultHasher::new();
    positions.hash(&mut hasher);
    keys.hash(&mut hasher);

    let hash = hasher.finish();
    if let Some(cached) = cache.get(&hash) {
        return *cached;
    }

    let mut shortest_path = usize::MAX;

    // Test each next possible path
    for i in 0..positions.len() {
        for (key_pos, _) in keys.iter() {
            if let Some((path, _)) = astar(&positions[i], key_pos, &tiles, &doors, false) {
                let mut new_pos = positions[i];
                let mut new_steps = 0;
                let mut new_keys = keys.clone();
                let mut new_doors = doors.clone();

                // Move to key and pick up all keys on path
                for tile in path.iter().skip(1) {
                    new_pos = *tile;
                    new_steps += 1;

                    // Pick up key
                    if let Some(key) = new_keys.remove(tile) {
                        // Unlock door
                        new_doors.remove(&key);
                    }
                }

                let mut new_positions = positions.clone();
                new_positions[i] = new_pos;

                let result = new_steps
                    + find_shortest_path(cache, tiles, &new_positions, &new_keys, &new_doors);
                if result < shortest_path {
                    shortest_path = result;
                }
            }
        }
    }

    cache.insert(hash, shortest_path);
    shortest_path
}

fn astar(
    pos: &Pos,
    goal: &Pos,
    tiles: &BTreeSet<Pos>,
    doors: &BTreeMap<char, Pos>,
    ignore_blocked: bool,
) -> Option<(Vec<Pos>, usize)> {
    pathfinding::astar(
        pos,
        |&(y, x)| {
            if ignore_blocked || !doors.values().contains(&(y, x)) {
                [(y, x + 1), (y + 1, x), (y, x - 1), (y - 1, x)]
                    .into_iter()
                    .filter_map(|pos| {
                        if tiles.contains(&pos) {
                            Some((pos, 1))
                        } else {
                            None
                        }
                    })
                    .collect_vec()
            } else {
                vec![]
            }
        },
        |&(y, x)| goal.0.abs_diff(y) + goal.1.abs_diff(x),
        |pos| pos == goal,
    )
}

type Pos = (usize, usize);
