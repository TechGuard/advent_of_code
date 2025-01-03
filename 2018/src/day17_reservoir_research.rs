use regex::Regex;
use std::collections::HashMap;

pub static DAY: u32 = 17;
pub static EXAMPLE_INPUT: &str = "";

const SPRING_X: i32 = 500;

#[derive(Debug, PartialEq)]
enum Material {
    Clay,
    Sand,
    Water,
    StillWater,
}

impl From<&Material> for char {
    fn from(material: &Material) -> Self {
        match material {
            Material::Clay => '#',
            Material::Sand => '.',
            Material::Water => '|',
            Material::StillWater => '~',
        }
    }
}

type MaterialMap = HashMap<(i32, i32), Material>;

fn parse_input(s: &str) -> MaterialMap {
    let mut map = HashMap::new();

    let re =
        Regex::new(r"^(?P<axis>x|y)=(?P<value>\d+), (x|y)=(?P<min>\d+)\.\.(?P<max>\d+)$").unwrap();

    for s in s.lines() {
        let capture = re.captures(s).unwrap();
        let value: i32 = capture["value"].parse().unwrap();
        let min: i32 = capture["min"].parse().unwrap();
        let max: i32 = capture["max"].parse().unwrap();

        if &capture["axis"] == "x" {
            for y in min..max + 1 {
                map.insert((value, y), Material::Clay);
            }
        } else {
            for x in min..max + 1 {
                map.insert((x, value), Material::Clay);
            }
        }
    }
    map
}

pub fn main(input: &str) -> (usize, usize) {
    let mut map = parse_input(&input);
    // let min_x = *map.iter().map(|((x, _), _)| x).min().unwrap() - 1;
    let min_y = *map.iter().map(|((_, y), _)| y).min().unwrap();
    // let max_x = *map.iter().map(|((x, _), _)| x).max().unwrap() + 1;
    let max_y = *map.iter().map(|((_, y), _)| y).max().unwrap();

    let is_infinite = |y| y < min_y || y > max_y;

    fn get_material(map: &MaterialMap, x: i32, y: i32) -> &Material {
        map.get(&(x, y)).unwrap_or(&Material::Sand)
    }

    loop {
        let mut x = SPRING_X;
        let mut y = min_y;
        let mut check_left = None;
        let mut material = Material::Water;

        loop {
            if is_infinite(y + 1) || get_material(&map, x, y + 1) == &Material::Water {
                break;
            }
            if get_material(&map, x, y + 1) == &Material::Sand {
                y += 1;
                check_left = None;
                continue;
            }

            // Just collided with something below, check left or right
            if check_left == None {
                check_left = Some(get_material(&map, x - 1, y) == &Material::Sand);
            }

            match check_left.unwrap() {
                true => {
                    if get_material(&map, x - 1, y) == &Material::Water {
                        break;
                    }
                    if get_material(&map, x - 1, y) == &Material::Sand {
                        x -= 1;
                        continue;
                    }
                }
                false => {
                    if get_material(&map, x + 1, y) == &Material::Water {
                        break;
                    }
                    if get_material(&map, x + 1, y) == &Material::Sand {
                        x += 1;
                        continue;
                    }
                }
            }

            // Can water settle? (either its next to settled water or between two walls)
            let mut can_settle = |direction| {
                let mut check_x = x + direction;
                let mut can_settle = false;
                loop {
                    // Cannot settle if there is no material to rest on
                    if get_material(&map, check_x, y + 1) != &Material::StillWater
                        && get_material(&map, check_x, y + 1) != &Material::Clay
                    {
                        break;
                    }
                    // Can settle if there is Water or Clay to the side
                    if get_material(&map, check_x, y) == &Material::StillWater
                        || get_material(&map, check_x, y) == &Material::Clay
                    {
                        can_settle = true;
                        break;
                    }
                    check_x += direction;
                }
                if !can_settle {
                    return false;
                }
                // If the water can settle, that means we can fill up the entire row
                for check_x in check_x - direction..x {
                    map.insert((check_x, y), Material::StillWater);
                }
                return true;
            };

            // Check both sides
            if can_settle(-1) && can_settle(1) {
                material = Material::StillWater;
            }
            break;
        }

        // Update map
        map.insert((x, y), material);

        // Cannot fill any more water
        if x == SPRING_X && y == min_y {
            break;
        }
    }

    // Print
    // for y in min_y..10 {
    //     for x in min_x..max_x + 1 {
    //         let c: char = map.get(&(x, y)).unwrap_or(&Material::Sand).into();
    //         print!("{}", c);
    //     }
    //     println!();
    // }

    (
        map.iter()
            .filter(|&(_, m)| m == &Material::StillWater || m == &Material::Water)
            .count(),
        map.iter()
            .filter(|&(_, m)| m == &Material::StillWater)
            .count(),
    )
}
