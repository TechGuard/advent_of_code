use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Debug)]
struct Cart {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    rotation: usize,
    crashed: bool,
}

impl Cart {
    // Sort by y and x
    fn sort(a: &Cart, b: &Cart) -> Ordering {
        if a.y == b.y {
            if a.x < b.x {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        } else if a.y < b.y {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    }

    fn update(&mut self, next: u8) {
        if next == b'-' || next == b'|' {
            self.x += self.dx;
            self.y += self.dy;
        } else if next == b'\\' {
            self.x += self.dx;
            self.y += self.dy;
            // mirror
            let tmp = self.dx;
            self.dx = self.dy;
            self.dy = tmp;
        } else if next == b'/' {
            self.x += self.dx;
            self.y += self.dy;
            // mirror
            let tmp = self.dx;
            self.dx = -self.dy;
            self.dy = -tmp;
        } else if next == b'+' {
            self.x += self.dx;
            self.y += self.dy;
            // check rotation
            match self.rotation {
                0 => {
                    // counter clockwise
                    let tmp = self.dx;
                    self.dx = self.dy;
                    self.dy = -tmp;
                }
                1 => {} // forward
                2 => {
                    // clockwise
                    let tmp = self.dx;
                    self.dx = -self.dy;
                    self.dy = tmp;
                }
                _ => {}
            }
            // increase intersection count
            self.rotation = (self.rotation + 1) % 3;
        }
    }
}

fn parse_input(input: &str) -> (Vec<Vec<u8>>, Vec<Cart>) {
    // Convert to bytes
    let mut tracks = input
        .lines()
        .map(|l| l.bytes().collect())
        .collect::<Vec<Vec<u8>>>();

    let width = tracks[0].len();
    let height = tracks.len();

    // Find carts and replace with correct symbol
    let mut carts = Vec::<Cart>::new();
    for y in 0..height {
        let track = &mut tracks[y];
        for x in 0..width {
            let mut dx = 0;
            let mut dy = 0;
            if track[x] == b'>' {
                track[x] = b'-';
                dx = 1;
            }
            if track[x] == b'<' {
                track[x] = b'-';
                dx = -1;
            }
            if track[x] == b'v' {
                track[x] = b'|';
                dy = 1;
            }
            if track[x] == b'^' {
                track[x] = b'|';
                dy = -1;
            }
            if dx != 0 || dy != 0 {
                carts.push(Cart {
                    x: x as i32,
                    y: y as i32,
                    dx: dx,
                    dy: dy,
                    rotation: 0,
                    crashed: false,
                });
            }
        }
    }
    return (tracks, carts);
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Expected input");

    let (tracks, mut carts) = parse_input(&input);
    let mut first = true;

    loop {
        carts.sort_by(Cart::sort);
        for i in 0..carts.len() {
            let cart = &mut carts[i];
            if cart.crashed {
                continue;
            }

            // Update cart
            let next = tracks[(cart.y + cart.dy) as usize][(cart.x + cart.dx) as usize];
            cart.update(next);

            // Find collisions
            let pos = (cart.x, cart.y);
            let collisions = carts
                .iter()
                .filter(|other| (other.x, other.y) == pos)
                .count();
            if collisions > 1 {
                if first {
                    first = false;
                    println!("1st answer = {},{}", pos.0, pos.1);
                }
                // println!("collision at {},{}", pos.0, pos.1);

                // Mark as crashed
                carts
                    .iter_mut()
                    .filter(|other| (other.x, other.y) == pos)
                    .for_each(|other| {
                        other.crashed = true;
                    });
            }
        }

        // Remove crashed carts
        carts.retain(|c| !c.crashed);

        // Exit if only one cart remains
        if carts.len() == 1 {
            let cart = carts.first().unwrap();
            println!("2nd answer = {},{}", cart.x, cart.y);
            break;
        }
    }
}
