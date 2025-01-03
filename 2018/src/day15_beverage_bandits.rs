use std::fmt::Display;
use std::str::FromStr;

pub static DAY: u32 = 15;
pub static EXAMPLE_INPUT: &str = "";

#[derive(Debug, Clone, Copy)]
struct Entity {
    team: i32,
    health: i32,
}

#[derive(Debug, Clone, Copy)]
enum Unit {
    Empty,
    Wall,
    Entity { data: Entity },
}

impl From<char> for Unit {
    fn from(c: char) -> Self {
        match c {
            '.' => Unit::Empty,
            '#' => Unit::Wall,
            'E' => Unit::Entity {
                data: Entity {
                    team: 0,
                    health: 200,
                },
            },
            'G' => Unit::Entity {
                data: Entity {
                    team: 1,
                    health: 200,
                },
            },
            _ => unreachable!(),
        }
    }
}

impl From<&Unit> for char {
    fn from(unit: &Unit) -> char {
        match unit {
            Unit::Empty => '.',
            Unit::Wall => '#',
            Unit::Entity { data } => match data.team {
                0 => 'E',
                1 => 'G',
                _ => unreachable!(),
            },
        }
    }
}

struct Board {
    units: Vec<Vec<Unit>>,
}

impl FromStr for Board {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Board {
            units: s
                .lines()
                .map(|line| line.chars().map(|c| Unit::from(c)).collect())
                .collect(),
        })
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.units
                .iter()
                .flat_map(|row| row.iter().map(|unit| char::from(unit)).chain(vec!['\n']))
                .collect::<String>()
        )
    }
}

impl Board {
    // Progress board by one round
    fn update(&mut self) {
        println!("NEXT ROUND");

        let mut entities = self.get_entity_positions();
        for i in 0..entities.len() {
            entities = self.get_entity_positions();

            // Get current entity ref
            let (x, y) = &entities[i];
            let entity = self.get_entity(x, y).unwrap();
            // println!("{},{}: {:?}", x, y, entity);

            // Calculate distance to all enemies
            let mut distances = Vec::new();
            for (x2, y2) in &entities {
                let entity2 = self.get_entity(x2, y2).unwrap();
                if entity.team != entity2.team {
                    distances.push(((x2 - x).abs() + (y2 - y).abs(), (x2, y2), entity2.health));
                }
            }

            // Find attackable targets
            let mut targets: Vec<_> = distances
                .iter()
                .filter(|&(x, _, _)| x == &1)
                .map(|&(_, pos, health)| (pos, health))
                .collect();

            // Standing next to an enemy. Attack
            if !targets.is_empty() {
                // Find weakest enemy
                targets.sort_by(|(_, health1), (_, health2)| health1.cmp(&health2));

                // Get mutable ref
                let &((x2, y2), _) = targets.first().unwrap();
                let target = self.get_entity_mut(x2, y2).unwrap();

                // Deal damage
                target.health -= 3;
                if target.health < 0 {
                    println!("DED");
                    // self.units[*y2 as usize][*x2 as usize] = Unit::Empty;
                }
                continue;
            }

            // Find closest enemy
            distances.sort_by(|(d1, _, _), (d2, _, _)| d1.cmp(d2));

            // Find next position
            let &(_, (x2, y2), _) = distances.first().unwrap();
            let mut newx = *x;
            let mut newy = *y;

            if x2 > x {
                newx += 1;
            } else if x2 < x {
                newx -= 1;
            } else if y2 > y {
                newy += 1;
            } else if y2 < y {
                newy -= 1;
            }

            // Move entity
            self.units[newy as usize][newx as usize] = self.units[*y as usize][*x as usize];
            self.units[*y as usize][*x as usize] = Unit::Empty;
        }
    }

    // Returns ordered list of the positions of all entities
    fn get_entity_positions(&self) -> Vec<(i32, i32)> {
        self.units
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(x, unit)| match unit {
                        Unit::Entity { data: _ } => Some((x as i32, y as i32)),
                        _ => None,
                    })
            })
            .collect()
    }

    // Returns ordered list of all entities
    fn get_entities(&self) -> Vec<&Entity> {
        self.units
            .iter()
            .flat_map(|row| {
                row.iter().filter_map(|unit| match unit {
                    Unit::Entity { data } => Some(data),
                    _ => None,
                })
            })
            .collect()
    }

    // Returns entity at position
    fn get_entity_mut(&mut self, x: &i32, y: &i32) -> Option<&mut Entity> {
        match &mut self.units[*y as usize][*x as usize] {
            Unit::Entity { data } => Some(data),
            _ => None,
        }
    }

    // Returns entity at position
    fn get_entity(&self, x: &i32, y: &i32) -> Option<&Entity> {
        match &self.units[*y as usize][*x as usize] {
            Unit::Entity { data } => Some(data),
            _ => None,
        }
    }
}

pub fn main(input: &str) -> (&'static str, &'static str) {
    let mut board: Board = input.parse().unwrap();

    for _ in 0..35 {
        board.update();

        println!("{}", board);
        println!("{:?}", board.get_entities());
    }

    // (get_answer1(recipes), get_answer2(recipes))
    ("Not implemented", "Not implemented")
}
