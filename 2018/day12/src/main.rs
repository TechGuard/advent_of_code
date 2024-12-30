use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Expected input");

    // This is ridiculous
    const NUM_PADDING: i64 = 600;
    let padding: String = vec!['.'; NUM_PADDING as usize].iter().collect::<String>();

    let mut state = padding.clone();
    state.push_str(&input.lines().next().unwrap()[15..]);
    state.push_str(padding.as_ref());

    let patterns = input
        .lines()
        .skip(2)
        .map(|s| (&s[..5], s.chars().nth(9).unwrap()))
        .collect::<Vec<_>>();

    fn match_pattern(state: &str, pattern: &str, i: usize) -> bool {
        state
            .chars()
            .skip(i)
            .take(pattern.len())
            .zip(pattern.chars())
            .all(|(a, b)| a == b)
    }

    fn count(state: &str) -> i64 {
        state
            .char_indices()
            .map(|(i, c)| if c == '#' { i as i64 - NUM_PADDING } else { 0 })
            .sum()
    }

    let mut last_count = count(&state);
    let mut last_diff = 0;
    let mut repeated = 0;
    let mut generation = 0;

    loop {
        // Match all patterns and build new state
        let mut new_state = state.chars().collect::<Vec<char>>();
        for &pattern in &patterns {
            for i in (0..state.len() - 5).filter(|&i| match_pattern(&state, pattern.0, i)) {
                new_state[i + 2] = pattern.1;
            }
        }
        state = new_state.iter().collect::<String>();

        let count = count(&state);
        let diff = count - last_count;

        // If diff is repeated 5 times we call it a day
        if diff == last_diff {
            repeated += 1;
            if repeated > 4 {
                break;
            }
        } else {
            repeated = 0;
        }

        last_count = count;
        last_diff = diff;

        generation += 1;
        if generation == 20 {
            println!("1st answer = {}", count);
        }
    }

    let total = last_count + (50000000000 - generation) * last_diff;
    println!("2nd answer = {}", total);
}
