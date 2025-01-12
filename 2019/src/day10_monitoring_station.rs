use anyhow::*;
use itertools::Itertools;
use std::f32::consts::FRAC_PI_2;

pub static DAY: u32 = 10;
pub static EXAMPLE_INPUT: &str = "\
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
";

pub fn main(input: &str) -> Result<(String, String)> {
    let mut asteroids = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (y as i64, x as i64))
        })
        .collect_vec();

    let mut counts = vec![0; asteroids.len()];

    // For each pair
    for i in 0..asteroids.len() {
        for j in (i + 1)..asteroids.len() {
            let min = &asteroids[i];
            let max = &asteroids[j];
            let slope = ((max.0 - min.0) as f32).atan2((max.1 - min.1) as f32);

            // Check if there is another asteroid with the same slope
            let mut is_visible = true;
            for k in (i + 1)..j {
                let max = &asteroids[k];
                let other_slope = ((max.0 - min.0) as f32).atan2((max.1 - min.1) as f32);
                if slope == other_slope {
                    is_visible = false;
                    break;
                }
            }

            if is_visible {
                counts[i] += 1;
                counts[j] += 1;
            }
        }
    }

    // Get asteroid with most visibility
    let best_asteroid = counts.iter().position_max().context("No asteroid found")?;
    let ans1 = counts[best_asteroid];

    // Calculate slope/distance to all targets and sort by distance
    let center: (i64, i64) = asteroids.remove(best_asteroid);

    let mut targets = asteroids
        .iter()
        .cloned()
        .map(|other| {
            // Note: flip y coordinate to make it play nicely with atan2 and we can iterate rotating clockwise
            let diff = (center.0 - other.0, other.1 - center.1);
            let slope = (diff.0 as f32).atan2(diff.1 as f32);
            (slope, diff.0.abs() + diff.1.abs(), other)
        })
        // Sort by slope from high to low and distance low to high
        .sorted_by(|lhs, rhs| lhs.0.total_cmp(&rhs.0).reverse().then(lhs.1.cmp(&rhs.1)))
        .collect_vec();

    // imma firin mah lazer
    let mut last_slope = FRAC_PI_2 + f32::EPSILON; // start *just* before our first target
    let mut ans2 = 0;

    for _ in 0..200 {
        // Find target that is closest to the last slope and fall back to start of the list
        // when no target is found because we looped 360 degrees
        let mut hit = 0;
        for (index, target) in targets.iter().enumerate() {
            if target.0 < last_slope {
                hit = index;
                break;
            }
        }

        // Vaporize target
        let (slope, _, pos) = targets.remove(hit);
        ans2 = pos.0 + pos.1 * 100;
        last_slope = slope;
    }

    Ok((ans1.to_string(), ans2.to_string()))
}
