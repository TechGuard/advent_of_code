use anyhow::*;
use std::collections::{HashMap, HashSet};

pub static DAY: u32 = 07;
pub static EXAMPLE_INPUT: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

pub fn main(input: &str) -> Result<(i64, i64)> {
    let mut teleporter = parse_input(input);
    Ok((part1(&teleporter), part2(&mut teleporter)))
}

fn part1(teleporter: &Teleporter) -> i64 {
    let mut beams = HashSet::new();
    beams.insert(teleporter.start.1);

    let mut result = 0;
    for (_, x) in &teleporter.splitters {
        if beams.remove(x) {
            beams.insert(x - 1);
            beams.insert(x + 1);
            result += 1;
        }
    }
    result
}

fn part2(teleporter: &mut Teleporter) -> i64 {
    teleporter.timelines(teleporter.start)
}

struct Teleporter {
    start: (i64, i64),
    splitters: Vec<(i64, i64)>,
    cache: HashMap<(i64, i64), i64>,
}

impl Teleporter {
    fn timelines(&mut self, pos: (i64, i64)) -> i64 {
        if let Some(&cached) = self.cache.get(&pos) {
            return cached;
        }

        if let Some(&(y, x)) = self
            .splitters
            .iter()
            .find(|(y, x)| *y > pos.0 && *x == pos.1)
        {
            let result = self.timelines((y, x - 1)) + self.timelines((y, x + 1));
            self.cache.insert(pos, result);
            return result;
        } else {
            return 1;
        }
    }
}

fn parse_input(input: &str) -> Teleporter {
    let mut start = (0, 0);
    let mut splitters = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (y as i64, x as i64);
            } else if c == '^' {
                splitters.push((y as i64, x as i64));
            }
        }
    }
    Teleporter {
        start,
        splitters,
        cache: HashMap::new(),
    }
}
