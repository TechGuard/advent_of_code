use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;

pub static DAY: u32 = 10;
pub static EXAMPLE_INPUT: &str = "";

#[derive(Debug, Clone)]
struct Point {
    posx: i32,
    posy: i32,
    velx: i32,
    vely: i32,
}

impl FromStr for Point {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^position=<\s?(?P<posx>-?\d+), \s?(?P<posy>-?\d+)> velocity=<\s?(?P<velx>-?\d+), \s?(?P<vely>-?\d+)>$"
            )
            .unwrap();
        }
        let capture = RE.captures(s).unwrap();
        Ok(Point {
            posx: capture["posx"].parse()?,
            posy: capture["posy"].parse()?,
            velx: capture["velx"].parse()?,
            vely: capture["vely"].parse()?,
        })
    }
}

pub fn main(input: &str) -> (String, i32) {
    let mut points = input
        .lines()
        .map(Point::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut text = String::new();
    let mut seconds = 0;
    loop {
        seconds += 1;

        // Move points
        for point in points.iter_mut() {
            point.posx += point.velx;
            point.posy += point.vely;
        }

        // Calc width / height
        let xmap = points.iter().map(|p| p.posx);
        let minx = xmap.clone().min().unwrap();
        let maxx = xmap.max().unwrap();

        let ymap = points.iter().map(|p| p.posy);
        let miny = ymap.clone().min().unwrap();
        let maxy = ymap.max().unwrap();
        let height = maxy - miny + 1;

        // Print text
        if height == 10 {
            let data = points
                .iter()
                .map(|p| (p.posx, p.posy))
                .collect::<HashSet<_>>();

            text.push('\n');
            for y in miny..maxy + 1 {
                for x in minx..maxx + 1 {
                    if data.contains(&(x, y)) {
                        text.push('#');
                    } else {
                        text.push('.');
                    }
                }
                text.push('\n');
            }
            text.pop();
            break;
        }
    }
    (text, seconds)
}
