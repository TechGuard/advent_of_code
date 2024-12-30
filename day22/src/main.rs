use std::io::{self, Read};

#[derive(Debug, Clone)]
enum Type {
    Start,
    Target,
    Rocky,
    Wet,
    Narrow,
}

struct Cave {
    erosion: Vec<Vec<usize>>,
    target_x: usize,
    target_y: usize,
    depth: usize,
}

impl std::fmt::Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let chars = self
            .erosion
            .iter()
            .map(|row| {
                row.iter()
                    .map(|x| match x % 3 {
                        0 => b'.',
                        1 => b'=',
                        2 => b'|',
                        _ => b'?',
                    })
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<_>>();
        write!(f, "{}", String::from_utf8(chars.join(&b'\n')).unwrap())
    }
}

impl Cave {
    fn new(depth: usize, target_x: usize, target_y: usize) -> Cave {
        Cave {
            erosion: vec![vec![0; target_x + 1]; target_y + 1],
            target_x: target_x,
            target_y: target_y,
            depth: depth,
        }
    }

    fn calculate_erosion(&mut self) {
        for y in 0..=self.target_y {
            for x in 0..=self.target_x {
                let geologic_index = if y == 0 {
                    x * 16807
                } else if x == 0 {
                    y * 48271
                } else {
                    self.erosion[y][x - 1] * self.erosion[y - 1][x]
                };
                self.erosion[y][x] = (geologic_index + self.depth) % 20183;
            }
        }

        self.erosion[0][0] = 0;
        self.erosion[self.target_y][self.target_x] = 0;
    }

    fn risk_level(&self) -> usize {
        self.erosion
            .iter()
            .map(|row| row.iter().map(|x| x % 3).sum::<usize>())
            .sum()
    }
}

fn parse_input(s: &str) -> (usize, (usize, usize)) {
    let mut lines = s.lines();
    let depth = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let mut target = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .into_iter()
        .map(|s| s.parse().unwrap());
    (depth, (target.next().unwrap(), target.next().unwrap()))
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Expected input");

    let (depth, (target_x, target_y)) = parse_input(&input);
    println!("{}: {},{}", depth, target_x, target_y);

    let mut cave = Cave::new(depth, target_x, target_y);
    cave.calculate_erosion();

    println!("{}", cave);
    println!("1st answer = {}", cave.risk_level());
}
