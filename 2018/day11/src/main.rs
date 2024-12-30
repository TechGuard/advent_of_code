use std::io::{self, Read};

const WIDTH: usize = 300;
const HEIGHT: usize = 300;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Expected input");

    let serial = input.parse::<i32>().unwrap();

    // Calculate power and build integral image
    // https://en.wikipedia.org/wiki/Summed-area_table
    let mut data = vec![vec![0; WIDTH + 1]; HEIGHT + 1];
    for y in 1..HEIGHT + 1 {
        for x in 1..WIDTH + 1 {
            let rack_id = x as i32 + 10;
            let mut power = rack_id * (y as i32);
            power += serial;
            power *= rack_id;
            power = (power / 100) % 10;
            power -= 5;
            data[y][x] = power + data[y - 1][x] + data[y][x - 1] - data[y - 1][x - 1];
        }
    }

    let mut max_total = 0;
    let mut best_cell = None;
    let mut answer_1 = None;

    // Check every cell_size
    for cell_size in 1..WIDTH + 1 {
        if let Some(cell) = find_best_cell(&data, cell_size) {
            // Store 1st answer
            if cell_size == 3 {
                answer_1 = Some(cell.clone());
            }
            // Check best total
            if cell.total >= max_total {
                max_total = cell.total;
                best_cell = Some(cell);
            }
        }
    }

    match answer_1 {
        None => println!("1st answer = None"),
        Some(cell) => println!("1st answer = {},{}", cell.x, cell.y),
    }
    match best_cell {
        None => println!("2nd answer = None"),
        Some(cell) => println!("2nd answer = {},{},{}", cell.x, cell.y, cell.size),
    }
}

#[derive(Debug, Clone)]
struct Cell {
    x: usize,
    y: usize,
    size: usize,
    total: i32,
}

fn find_best_cell(data: &Vec<Vec<i32>>, cell_size: usize) -> Option<Cell> {
    let mut max_total = std::i32::MIN;
    let mut best_cell = None;

    for y in 1..HEIGHT - cell_size {
        for x in 1..WIDTH - cell_size {
            let total = data[y + cell_size][x + cell_size] + data[y][x]
                - data[y][x + cell_size]
                - data[y + cell_size][x];

            // Check best total
            if total > max_total {
                max_total = total;
                best_cell = Some(Cell {
                    x: x + 1,
                    y: y + 1,
                    size: cell_size,
                    total: total,
                });
            }
        }
    }
    return best_cell;
}
