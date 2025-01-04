use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fmt::Display;
use std::iter::FromIterator;
use std::ops::Add;
use std::ops::Sub;

pub static DAY: u32 = 15;
pub static EXAMPLE_INPUT: &str = "\
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######
";

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Pos(i32, i32);

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).then_with(|| self.1.cmp(&other.1))
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for Pos {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Pos {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(other.0 - self.0, other.1 - self.1)
    }
}

impl Pos {
    fn distance(self, other: Self) -> i32 {
        let diff = self - other;
        diff.0.abs() + diff.1.abs()
    }
}

const DIRS: [Pos; 4] = [Pos(0, 1), Pos(1, 0), Pos(0, -1), Pos(-1, 0)];

#[derive(Debug, Clone, Copy, PartialEq)]
enum Team {
    Goblin,
    Elf,
}

#[derive(Debug, Clone, Copy)]
struct Entity {
    team: Team,
    hp: i32,
    processed: bool,
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}({})", Into::<char>::into(*self), self.hp))
    }
}

impl TryFrom<char> for Entity {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value == 'G' || value == 'E' {
            Ok(Entity {
                team: match value {
                    'G' => Team::Goblin,
                    'E' => Team::Elf,
                    _ => unreachable!(),
                },
                hp: 200,
                processed: false,
            })
        } else {
            Err(())
        }
    }
}

impl Into<char> for Entity {
    fn into(self) -> char {
        match self.team {
            Team::Goblin => 'G',
            Team::Elf => 'E',
        }
    }
}

fn find_best_next_pos(map: &Vec<Vec<char>>, start: Pos, targets: Vec<Pos>) -> Option<Pos> {
    #[derive(Clone, Eq, PartialEq)]
    struct State {
        cost: usize,
        pos: Pos,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&self.cost)
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut visited = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut max_cost = None;
    let mut ends = Vec::new();

    visited.insert(start, (0usize, HashSet::new()));
    heap.push(State {
        cost: 0,
        pos: start,
    });

    while let Some(State { cost, pos }) = heap.pop() {
        // Stop searching beyond the maximum cost
        if max_cost.is_some() && cost > max_cost.unwrap() {
            continue;
        }
        // Find new position in each direction
        for dir in DIRS {
            let next = State {
                cost: cost + 1,
                pos: pos + dir,
            };

            // Found a target pos
            if targets.contains(&next.pos) {
                if max_cost.is_none() {
                    // set a maximum cost because we only care about the best end positions
                    max_cost = Some(cost);
                }
                ends.push(pos);
                continue;
            }

            // Can only move to an empty tile
            if map[next.pos.0 as usize][next.pos.1 as usize] != '.' {
                continue;
            }

            if let Some((visited_cost, prev)) = visited.get_mut(&next.pos) {
                // If another path already exists with the SAME cost, insert a branching path
                if next.cost == *visited_cost {
                    prev.insert(pos);
                }
                continue;
            } else {
                let mut prev = HashSet::new();
                if pos != start {
                    prev.insert(pos);
                }
                visited.insert(next.pos, (next.cost, prev));
            }
            heap.push(next);
        }
    }

    if ends.is_empty() {
        return None;
    }

    ends.sort();
    let lowest_end = *ends.first().unwrap();

    // Find all starting positions and pick the lowest
    let mut lowest_begin = None;
    let mut queue = vec![lowest_end];
    while let Some(prev) = queue.pop() {
        if let Some((_, nexts)) = visited.get(&prev) {
            if nexts.is_empty() {
                if lowest_begin.is_none() || lowest_begin.unwrap() > prev {
                    lowest_begin = Some(prev);
                }
            }
            for next in nexts {
                queue.push(*next);
            }
        }
    }
    lowest_begin
}

fn battle(
    mut map: Vec<Vec<char>>,
    mut entities: BTreeMap<Pos, Entity>,
    attack_power: i32,
) -> (i32, bool) {
    let begin_elf_count = entities.values().filter(|e| e.team == Team::Elf).count();

    let mut round = 0;
    let mut end_round = false;
    while !end_round {
        round += 1;

        // Loop until all entities are processed
        loop {
            // Find first unprocessed entity
            let (mut pos, entity) = {
                let entity_itr = entities.iter_mut().find(|(_, e)| !e.processed);
                if entity_itr.is_none() {
                    break;
                }
                let (pos, entity) = entity_itr.unwrap();
                entity.processed = true; // Mark as processed
                (*pos, *entity)
            };

            // Find all its targets
            let targets: Vec<_> = entities
                .iter()
                .filter(|(_, e)| e.team != entity.team)
                .map(|(pos, _)| *pos)
                .collect();

            if targets.is_empty() {
                round -= 1;
                end_round = true;
                break;
            }

            // Find shortest paths to those targets
            let next_pos = find_best_next_pos(&map, pos, targets);
            if let Some(next_pos) = next_pos {
                // Move
                let entity = entities.remove(&pos).unwrap();
                map[pos.0 as usize][pos.1 as usize] = '.';

                pos = next_pos;
                entities.insert(pos, entity);
                map[pos.0 as usize][pos.1 as usize] = entity.into();
            }

            let target = {
                // Find all the targets that can be attacked
                let mut targets: Vec<_> = entities
                    .iter_mut()
                    .filter(|(rhs_pos, rhs_entity)| {
                        rhs_entity.team != entity.team && rhs_pos.distance(pos) == 1
                    })
                    .collect();

                // Find target with lowest hp
                targets.sort_by(|(_, lhs), (_, rhs)| lhs.hp.cmp(&rhs.hp));
                targets.first().map(|(pos, _)| **pos)
            };

            // Attack!
            if let Some(target_pos) = target {
                let target_entity = entities.get_mut(&target_pos).unwrap();
                target_entity.hp -= match entity.team {
                    Team::Goblin => 3,
                    Team::Elf => attack_power,
                };

                // Remove target because it's dead
                if target_entity.hp <= 0 {
                    entities.remove(&target_pos);
                    map[target_pos.0 as usize][target_pos.1 as usize] = '.';
                }
            }
        }

        // Reset processed flags
        for entity in entities.values_mut() {
            entity.processed = false;
        }
    }

    if false {
        println!("Round {}", round);
        for (y, row) in map.iter().enumerate() {
            println!(
                "{}   {}",
                String::from_iter(row),
                entities
                    .iter()
                    .filter(|(pos, _)| pos.0 == y as i32)
                    .map(|(_, entity)| format!("{}", entity))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }

    let elf_count = entities.values().filter(|e| e.team == Team::Elf).count();
    (
        round * entities.values().map(|e| e.hp).sum::<i32>(),
        begin_elf_count == elf_count,
    )
}

pub fn main(input: &str) -> (i32, i32) {
    let map: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    // Extract entities and place inside BTreeMap because it's ordered on the keys
    let mut entities = BTreeMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if let Ok(entity) = Entity::try_from(*c) {
                entities.insert(Pos(y as i32, x as i32), entity);
            }
        }
    }

    let ans1 = battle(map.clone(), entities.clone(), 3).0;
    let ans2;

    // Keep increasing attack power until there are no losses
    let mut attack_power = 4;
    loop {
        let (ans, no_losses) = battle(map.clone(), entities.clone(), attack_power);
        if no_losses {
            ans2 = ans;
            break;
        } else {
            attack_power += 1;
        }
    }

    (ans1, ans2)
}
