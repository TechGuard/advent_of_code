use std::collections::HashMap;
use std::io::{self, Read};

type Pos = (i32, i32);

#[derive(Default, Hash, PartialEq, Eq)]
struct Room {
    doors: Vec<Pos>,
}

#[derive(Default)]
struct Grid {
    rooms: HashMap<Pos, Room>,
}

impl Grid {
    fn direction(&mut self, pos: &mut Pos, dx: i32, dy: i32) {
        self.add_door(*pos, (pos.0 + dx, pos.1 + dy));
        pos.0 += dx;
        pos.1 += dy;
    }

    fn add_door(&mut self, from: Pos, to: Pos) {
        let room = self.rooms.entry(from).or_default();
        room.doors.push(to);
        let room = self.rooms.entry(to).or_default();
        room.doors.push(from);
    }

    fn get_doors(&self, pos: &Pos) -> Vec<Pos> {
        if let Some(room) = self.rooms.get(pos) {
            room.doors.clone()
        } else {
            Vec::new()
        }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let min_x = *self.rooms.keys().map(|(x, _)| x).min().unwrap();
        let min_y = *self.rooms.keys().map(|(_, y)| y).min().unwrap();
        let max_x = *self.rooms.keys().map(|(x, _)| x).max().unwrap();
        let max_y = *self.rooms.keys().map(|(_, y)| y).max().unwrap();
        let data_x = |x: i32| (x - min_x) as usize * 2 + 1;
        let data_y = |y: i32| (y - min_y) as usize * 2 + 1;
        let mut data = vec![vec![b'#'; data_x(max_x + 1)]; data_y(max_y + 1)];

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if let Some(room) = self.rooms.get(&(x, y)) {
                    let data_x = data_x(x);
                    let data_y = data_y(y);
                    data[data_y][data_x] = b'.';

                    if room.doors.contains(&(x + 1, y)) {
                        data[data_y][data_x + 1] = b'|';
                    }
                    if room.doors.contains(&(x - 1, y)) {
                        data[data_y][data_x - 1] = b'|';
                    }
                    if room.doors.contains(&(x, y + 1)) {
                        data[data_y + 1][data_x] = b'-';
                    }
                    if room.doors.contains(&(x, y - 1)) {
                        data[data_y - 1][data_x] = b'-';
                    }
                }
            }
        }

        data[data_y(0)][data_x(0)] = b'X';
        write!(f, "{}", String::from_utf8(data.join(&b'\n')).unwrap())
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Expected input");

    let mut chars = input.chars();
    chars.next();

    let mut grid = Grid::default();
    let mut stack = Vec::<Pos>::new();
    let mut pos = (0, 0);

    for c in chars {
        match c {
            'N' => grid.direction(&mut pos, 0, -1),
            'E' => grid.direction(&mut pos, 1, 0),
            'S' => grid.direction(&mut pos, 0, 1),
            'W' => grid.direction(&mut pos, -1, 0),
            '(' => stack.push(pos),
            ')' => {
                stack.pop();
            }
            '|' => pos = *stack.last().unwrap(),
            _ => break,
        };
    }

    println!("{}", grid);

    // Check distances for all rooms
    let mut visited = HashMap::<Pos, usize>::new();
    let mut stack = Vec::<(Pos, usize)>::new();
    stack.push(((0, 0), 0));

    while !stack.is_empty() {
        let (pos, distance) = stack.pop().unwrap();
        if !visited.contains_key(&pos) {
            // Mark position as visited
            *visited.entry(pos).or_default() = distance;

            // Check neighbouring doors
            let doors = grid.get_doors(&pos);
            stack.extend(doors.iter().map(|&x| (x, distance + 1)));
        }
    }

    let (_, distance) = visited.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
    println!("1st answer = {}", distance);
    println!(
        "2nd answer = {}",
        visited.iter().filter(|x| x.1 >= &1000).count()
    );
}
