use crate::utils::intcode;
use anyhow::*;
use itertools::Itertools;
use pathfinding::prelude as pathfinding;
use std::collections::BTreeMap;

pub static DAY: u32 = 15;
pub static EXAMPLE_INPUT: &str = "";

pub fn main(input: &str) -> Result<(usize, usize)> {
    let mut droid = RepairDroid::new(intcode::parse_data(input)?);
    droid.build_ship_map()?;
    Ok((droid.find_oxygen_system()?, droid.time_to_fill_oxygen()?))
}

impl RepairDroid {
    fn new(data: Vec<i64>) -> Self {
        RepairDroid {
            program: intcode::Program::new(data),
            map: BTreeMap::new(),
            pos: (0, 0),
            oxygen_system: (0, 0),
        }
    }

    fn find_oxygen_system(&self) -> Result<usize> {
        // Find shortest path from (0, 0) to oxygen system
        pathfinding::dijkstra(
            &(0, 0),
            |&(y, x)| {
                [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)]
                    .into_iter()
                    .filter_map(|pos| match self.map.get(&(y, x)) {
                        Some(Tile::Wall) | None => None,
                        _ => Some((pos, 1usize)),
                    })
                    .collect_vec()
            },
            |p| p == &self.oxygen_system,
        )
        .context("No path found")
        .map(|(_, length)| length)
    }

    fn time_to_fill_oxygen(&self) -> Result<usize> {
        // Find all reachable positions and their shortest distance to the oxygen system
        let reachables = pathfinding::dijkstra_all(&self.oxygen_system, |&(y, x)| {
            [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)]
                .into_iter()
                .filter_map(|pos| match self.map.get(&(y, x)) {
                    Some(Tile::Wall) | None => None,
                    _ => Some((pos, 1usize)),
                })
                .collect_vec()
        });
        // Return the maximum distance
        reachables
            .values()
            .map(|(_, dist)| dist)
            .max()
            .cloned()
            .context("No path found")
    }

    fn build_ship_map(&mut self) -> Result<()> {
        let mut prev_steps = Vec::new();
        let mut dir = Direction::North;

        self.map.insert(self.pos, Tile::Empty);

        loop {
            // Try left, right, forward
            let mut dir_options = Vec::new();
            for dir in match dir {
                Direction::North | Direction::South => [Direction::West, Direction::East, dir],
                Direction::West | Direction::East => [Direction::North, Direction::South, dir],
            } {
                // Look into direction
                if self.move_droid(dir)? != Tile::Wall {
                    // Move back if it's a valid position and record option
                    self.move_droid(dir.inverse())?;
                    dir_options.push(dir);
                }
            }

            // Take the first direction
            if !dir_options.is_empty() {
                dir = dir_options.pop().unwrap();
                prev_steps.push((dir, dir_options));

                self.move_droid(dir)?;
                continue;
            }

            // Out of options, start backtracking
            while let Some((prev_dir, mut dir_options)) = prev_steps.pop() {
                self.move_droid(prev_dir.inverse())?;

                // Take the next option if possible
                if !dir_options.is_empty() {
                    dir = dir_options.pop().unwrap();
                    prev_steps.push((dir, dir_options));

                    self.move_droid(dir)?;
                    break;
                }
            }

            // Exit loop if droid cannot go backwards
            if prev_steps.is_empty() {
                break;
            }
        }

        Ok(())
    }

    fn move_droid(&mut self, direction: Direction) -> Result<Tile> {
        self.program.give_input(direction as i64);
        if let intcode::Action::Output(output) = self.program.execute()? {
            let tile: Tile = output.try_into()?;
            let tile_pos = direction.update_pos(self.pos);
            if tile != Tile::Wall {
                self.pos = tile_pos;
            }
            if tile == Tile::OxygenSystem {
                self.oxygen_system = tile_pos;
            }
            self.map.insert(tile_pos, tile.clone());
            Ok(tile)
        } else {
            bail!("Invalid program action")
        }
    }
}

struct RepairDroid {
    program: intcode::Program,
    map: BTreeMap<(i64, i64), Tile>,
    pos: (i64, i64),
    oxygen_system: (i64, i64),
}

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Direction {
    fn update_pos(&self, pos: (i64, i64)) -> (i64, i64) {
        match *self {
            Direction::North => (pos.0 + -1, pos.1),
            Direction::South => (pos.0 + 1, pos.1),
            Direction::West => (pos.0, pos.1 + -1),
            Direction::East => (pos.0, pos.1 + 1),
        }
    }

    fn inverse(&self) -> Self {
        match *self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
}

#[derive(PartialEq, Clone)]
enum Tile {
    Wall,
    Empty,
    OxygenSystem,
}

impl TryFrom<i64> for Tile {
    type Error = anyhow::Error;
    fn try_from(value: i64) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Wall,
            1 => Self::Empty,
            2 => Self::OxygenSystem,
            _ => bail!("Invalid status: {}", value),
        })
    }
}
