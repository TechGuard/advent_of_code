use std::collections::HashSet;
use std::io::{self, Read};
use std::str::FromStr;

#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;

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

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Expected input");

    let mut points = input
        .lines()
        .map(Point::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

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

            println!("1st Answer =");
            for y in miny..maxy + 1 {
                for x in minx..maxx + 1 {
                    if data.contains(&(x, y)) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            break;
        }
    }
    println!("2nd Answer = {}", seconds);
}
