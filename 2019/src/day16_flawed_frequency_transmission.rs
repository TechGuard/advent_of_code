use anyhow::*;
use itertools::Itertools;

pub static DAY: u32 = 16;
pub static EXAMPLE_INPUT: &str = "03036732577212944063491565474664";

pub fn main(input: &str) -> Result<(String, String)> {
    let signal: Vec<u32> = input
        .lines()
        .next()
        .context("Invalid input")?
        .chars()
        .map(|c| c.to_digit(10).context("Invalid digit"))
        .try_collect()?;

    let signal = signal.into_iter().map(|x| x as i64).collect_vec();

    // let mut long_signal = Vec::with_capacity(signal.len() * 10_000); // 6.5 million
    // let long_offset: usize = signal.iter().take(7).join("").parse()?;
    // for _ in 0..10_000 {
    //     long_signal.extend_from_slice(&signal);
    // }

    Ok((calculate_signal(signal, 0)?, "Not implemented".into()))
}

fn calculate_signal(mut signal: Vec<i64>, offset: usize) -> Result<String> {
    let mut next_signal = Vec::with_capacity(signal.len());

    for _ in 0..100 {
        // Calculate next signal
        for i in 1..=signal.len() {
            // Apply pattern
            let mut value = 0;
            let mut j = 1;
            while j <= signal.len() {
                let base_pattern = [0, 1, 0, -1];
                let pattern_offset = (j / i) % base_pattern.len();
                // skip all zeros
                if base_pattern[pattern_offset] == 0 {
                    if j == 1 {
                        j += i - 1;
                    } else {
                        j += i;
                    }
                    continue;
                }
                value += signal[j - 1] * base_pattern[pattern_offset];
                j += 1;
            }

            // Take last digit
            value = value
                .to_string()
                .chars()
                .last()
                .context("Invalid number")?
                .to_digit(10)
                .context("Invalid number")? as i64;
            next_signal.push(value);
        }

        std::mem::swap(&mut signal, &mut next_signal);
        next_signal.clear();
    }

    Ok(signal.into_iter().skip(offset).take(8).join(""))
}
