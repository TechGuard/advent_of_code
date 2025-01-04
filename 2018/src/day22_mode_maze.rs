use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    usize, vec,
};

pub static DAY: u32 = 22;
pub static EXAMPLE_INPUT: &str = "\
depth: 510
target: 10,10
";

enum Type {
    Rocky,
    Wet,
    Narrow,
}

impl From<usize> for Type {
    fn from(value: usize) -> Self {
        match value % 3 {
            0 => Self::Rocky,
            1 => Self::Wet,
            2 => Self::Narrow,
            _ => unreachable!(),
        }
    }
}

impl From<Type> for char {
    fn from(val: Type) -> Self {
        match val {
            Type::Rocky => '.',
            Type::Wet => '=',
            Type::Narrow => '|',
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Pos(usize, usize);

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Tool {
    Torch,
    ClimbingGear,
    Neither,
}

struct Cave {
    erosion: Vec<Vec<usize>>,
    target_x: usize,
    target_y: usize,
    depth: usize,
}

impl std::fmt::Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let lines: Vec<String> = self
            .erosion
            .iter()
            .map(|row| {
                row.iter()
                    .map(|x| Into::<char>::into(Type::from(*x)))
                    .collect()
            })
            .collect();
        write!(f, "{}", lines.join("\n"))
    }
}

impl Cave {
    fn new(depth: usize, target_x: usize, target_y: usize) -> Cave {
        const MARGIN: usize = 100;
        let mut cave = Cave {
            erosion: vec![vec![0; target_x + MARGIN + 1]; target_y + MARGIN + 1],
            target_x,
            target_y,
            depth,
        };
        cave.calculate_erosion();
        cave
    }

    fn calculate_erosion(&mut self) {
        for y in 0..self.erosion.len() {
            for x in 0..self.erosion[y].len() {
                let geologic_index = match (y, x) {
                    (0, x) => x * 16807,
                    (y, 0) => y * 48271,
                    _ => self.erosion[y][x - 1] * self.erosion[y - 1][x],
                };
                self.erosion[y][x] = (geologic_index + self.depth) % 20183;
            }
        }
        self.erosion[self.target_y][self.target_x] = 0;
    }

    fn risk_level(&self) -> usize {
        self.erosion
            .iter()
            .take(self.target_y + 1)
            .flat_map(|row| {
                row.iter()
                    .take(self.target_x + 1)
                    .map(|x| Type::from(*x) as usize)
            })
            .sum()
    }

    fn allowed_tools(&self, pos: &Pos) -> [Tool; 2] {
        let region: Type = self.erosion[pos.0][pos.1].into();
        match region {
            Type::Rocky => [Tool::ClimbingGear, Tool::Torch],
            Type::Wet => [Tool::ClimbingGear, Tool::Neither],
            Type::Narrow => [Tool::Torch, Tool::Neither],
        }
    }

    fn min_travel_time(&self) -> Option<usize> {
        #[derive(Clone, Eq, PartialEq)]
        struct State {
            cost: usize,
            pos: Pos,
            tool: Tool,
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

        let mut visited = HashSet::new();
        let mut heap = BinaryHeap::new();

        let goal = Pos(self.target_y, self.target_x);

        heap.push(State {
            cost: 0,
            pos: Pos(0, 0),
            tool: Tool::Torch,
        });

        while let Some(State { cost, pos, tool }) = heap.pop() {
            if visited.contains(&(pos, tool)) {
                continue;
            }
            visited.insert((pos, tool));

            if pos == goal && tool == Tool::Torch {
                return Some(cost);
            }

            // Switch tool
            heap.push(State {
                cost: cost + 7,
                pos,
                tool: *self
                    .allowed_tools(&pos)
                    .iter()
                    .filter(|t| **t != tool)
                    .next()
                    .unwrap(),
            });

            // Find new position in each direction where we have the correct tool
            for next_pos in (0..4)
                .filter_map(|i| match i {
                    0 if pos.1 + 1 < self.erosion[0].len() => Some(Pos(pos.0, pos.1 + 1)),
                    1 if pos.0 + 1 < self.erosion.len() => Some(Pos(pos.0 + 1, pos.1)),
                    2 if pos.1 > 0 => Some(Pos(pos.0, pos.1 - 1)),
                    3 if pos.0 > 0 => Some(Pos(pos.0 - 1, pos.1)),
                    _ => None,
                })
                .filter(|next_pos| self.allowed_tools(next_pos).contains(&tool))
            {
                heap.push(State {
                    cost: cost + 1,
                    pos: next_pos,
                    tool,
                });
            }
        }
        None
    }
}

fn parse_input(s: &str) -> Option<(usize, (usize, usize))> {
    let mut lines = s.lines().flat_map(|l| l.split_whitespace().skip(1));
    let depth: usize = lines.next()?.parse().ok()?;
    let mut target = lines.next()?.split(",").flat_map(|s| s.parse());
    Some((depth, (target.next()?, target.next()?)))
}

pub fn main(input: &str) -> (usize, usize) {
    let (depth, (target_x, target_y)) = parse_input(&input).unwrap();
    let cave = Cave::new(depth, target_x, target_y);
    (cave.risk_level(), cave.min_travel_time().unwrap())
}
