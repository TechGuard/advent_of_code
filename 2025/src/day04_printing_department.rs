use anyhow::*;

pub static DAY: u32 = 04;
pub static EXAMPLE_INPUT: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

struct Map {
    tiles: Vec<Vec<char>>,
    height: i64,
    width: i64,
}

pub fn main(input: &str) -> Result<(usize, usize)> {
    let tiles: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut map = Map {
        height: tiles.len() as i64,
        width: tiles.first().context("invalid map")?.len() as i64,
        tiles,
    };

    let result1 = solve(&mut map);
    let mut result2 = result1;
    loop {
        let result = solve(&mut map);
        if result == 0 {
            break;
        }
        result2 += result;
    }

    Ok((result1, result2))
}

fn solve(map: &mut Map) -> usize {
    let mut results = Vec::new();
    for y in 0..map.height {
        for x in 0..map.width {
            if map.tiles[y as usize][x as usize] == '@' && valid(&map, y, x) {
                results.push((y, x));
            }
        }
    }
    for (y, x) in &results {
        map.tiles[*y as usize][*x as usize] = 'x';
    }
    results.len()
}

fn valid(map: &Map, y: i64, x: i64) -> bool {
    let mut count = 0;
    for j in 0.max(y - 1)..=(map.height - 1).min(y + 1) {
        for i in 0.max(x - 1)..=(map.width - 1).min(x + 1) {
            if !(j == y && i == x) && map.tiles[j as usize][i as usize] == '@' {
                count += 1;
                if count >= 4 {
                    return false;
                }
            }
        }
    }
    true
}
