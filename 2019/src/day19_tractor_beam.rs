use crate::utils::intcode;
use anyhow::*;

pub static DAY: u32 = 19;
pub static EXAMPLE_INPUT: &str = "";

pub fn main(input: &str) -> Result<(i64, i64)> {
    let data = intcode::parse_data(input)?;
    let get_output = |x, y| -> Result<i64> {
        Ok(intcode::execute_until_end(data.clone(), [x, y])?
            .first()
            .cloned()
            .unwrap_or_default())
    };

    // Part 1 and find slope
    let mut ans1 = 0;
    let mut y_min = i64::MAX;
    let mut y_max = 0;
    for y in 0..50 {
        for x in 0..50 {
            if get_output(x, y)? != 1 {
                continue;
            }
            ans1 += 1;
            if x + 1 == 50 {
                y_min = y_min.min(y);
                y_max = y_max.max(y);
            }
        }
    }

    let slope = (y_min + y_max) as f32 / 100.0;

    // Part 2 binary search

    let mut x_min = 1;
    let mut x_max = 10000;
    let ans2 = loop {
        let mut x = (x_min + x_max) / 2;

        // Find lower left corner of square
        let mut y_max: i64 = (slope * x as f32) as i64;
        loop {
            if get_output(x, y_max + 1)? != 1 {
                break;
            }
            y_max += 1;
        }

        if get_output(x - 1, y_max)? == 1 {
            x -= 1;
        }

        // Find upper right corner of square
        let size = 100;
        let mut offset = 0;
        while offset + 1 < size {
            offset += 1;
            if get_output(x + offset, y_max - offset)? != 1 {
                offset -= 1;
                break;
            }
        }

        // The square fits
        if offset + 1 == size {
            if x == x_min {
                break x * 10000 + y_max - offset;
            }
            x_max = x - 1;
        } else {
            x_min = x + 1;
        }
    };

    Ok((ans1, ans2))
}
