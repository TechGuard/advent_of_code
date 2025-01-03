use core::str::FromStr;

pub static DAY: u32 = 06;
pub static EXAMPLE_INPUT: &str = "";

#[derive(Debug, Clone)]
struct Coord {
    x: i32,
    y: i32,
    total: i32,
    infinite: bool,
}

impl Coord {
    fn distance(&self, x: i32, y: i32) -> i32 {
        i32::abs(self.x - x) + i32::abs(self.y - y)
    }
}

impl FromStr for Coord {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pos = s.find(",").unwrap();
        Ok(Coord {
            x: s[..pos].parse::<i32>()?,
            y: s[pos + 2..].parse::<i32>()?,
            total: 0,
            infinite: false,
        })
    }
}

#[derive(Debug)]
struct Area {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

impl Area {
    fn is_outside(&self, x: i32, y: i32) -> bool {
        return x <= self.min_x || y <= self.min_x || x >= self.max_x || y >= self.max_y;
    }
}

pub fn main(input: &str) -> (i32, i32) {
    let coords = input
        .lines()
        .map(Coord::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut area = Area {
        min_x: std::i32::MAX,
        min_y: std::i32::MAX,
        max_x: std::i32::MIN,
        max_y: std::i32::MIN,
    };
    for coord in &coords {
        area.min_x = std::cmp::min(area.min_x, coord.x);
        area.min_y = std::cmp::min(area.min_y, coord.y);
        area.max_x = std::cmp::max(area.max_x, coord.x);
        area.max_y = std::cmp::max(area.max_y, coord.y);
    }

    (
        get_answer1(&area, coords.clone()),
        get_answer2(&area, coords),
    )
}

fn get_answer1(area: &Area, mut coords: Vec<Coord>) -> i32 {
    let border = 20;
    for y in area.min_y - border..area.max_y + 1 + border {
        for x in area.min_x - border..area.max_x + 1 + border {
            let mut min_dist = std::i32::MAX;
            let mut unique = false;
            let mut winner = None;

            for coord in &mut coords {
                let dist = coord.distance(x, y);
                if dist == min_dist {
                    unique = false;
                }
                if dist < min_dist {
                    min_dist = dist;
                    unique = true;
                    winner = Some(coord);
                }
            }

            if unique {
                let winner = winner.unwrap();
                if area.is_outside(x, y) {
                    winner.infinite = true;
                }
                winner.total += 1;
            }
        }
    }

    let mut check = coords
        .iter()
        .filter(|coord| !coord.infinite)
        .collect::<Vec<_>>();
    check.sort_by_key(|coord| coord.total);
    check.last().unwrap().total
}

fn get_answer2(area: &Area, coords: Vec<Coord>) -> i32 {
    let threshold = 10000;
    let mut total = 0;

    for y in area.min_y..area.max_y + 1 {
        for x in area.min_x..area.max_x + 1 {
            let sum = coords.iter().map(|coord| coord.distance(x, y)).sum::<i32>();
            if sum < threshold {
                total += 1;
            }
        }
    }
    total
}
