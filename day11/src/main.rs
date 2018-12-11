use std::io::{self, Read};

const WIDTH: usize = 300;
const HEIGHT: usize = 300;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Expected input");

    let serial = input.parse::<i32>().unwrap();

    // Calculate power
    let mut data = vec![0; WIDTH * HEIGHT];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let rack_id = x as i32 + 11;
            let mut power = rack_id * (y as i32 + 1);
            power += serial;
            power *= rack_id;
            power = (power / 100) % 10;
            power -= 5;
            data[y * WIDTH + x] = power;
        }
    }

    let mut answer_1 = None;

    let mut max_total = 0;
    let mut best_cell = None;
    let mut cache = vec![0; WIDTH * HEIGHT];

    for cell_size in 1..WIDTH + 1 {
        if let Some(cell) = find_best_cell(&data, &mut cache, cell_size) {
            // Store 1st answer
            if cell_size == 3 {
                answer_1 = Some(cell.clone());
            }

            // Stop searching if total is lower than zero
            if cell.total < 0 {
                break;
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

fn find_best_cell(data: &Vec<i32>, cache: &mut Vec<i32>, cell_size: usize) -> Option<Cell> {
    let mut max_total = std::i32::MIN;
    let mut best_cell = None;

    for y in 0..HEIGHT - (cell_size - 1) {
        for x in 0..WIDTH - (cell_size - 1) {
            // Assume last cache was cell_size - 1
            let mut total = cache[y * WIDTH + x];

            // Add one row
            for xoff in 0..cell_size {
                total += data[(y + cell_size - 1) * WIDTH + (x + xoff)];
            }
            // Add one column (without corner)
            for yoff in 0..cell_size - 1 {
                total += data[(y + yoff) * WIDTH + (x + cell_size - 1)];
            }

            // Save to cache
            cache[y * WIDTH + x] = total;

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
