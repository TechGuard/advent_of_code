use anyhow::*;
use vecmath::*;

pub static DAY: u32 = 09;
pub static EXAMPLE_INPUT: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

type Vec2 = Vector2<i64>;

pub fn main(input: &str) -> Result<(i64, i64)> {
    let squares = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            Ok([
                parts.next().context("missing first number")?.parse()?,
                parts.next().context("missing second number")?.parse()?,
            ])
        })
        .collect::<Result<Vec<Vec2>>>()?;

    // Compute all options and sort by area size
    let mut solutions = Vec::new();
    for i in 0..(squares.len() - 1) {
        for j in (i + 1)..squares.len() {
            let diff = vec2_sub(squares[i], squares[j]);
            let area = (diff[0].abs() + 1) * (diff[1].abs() + 1);
            solutions.push((area, i, j));
        }
    }
    solutions.sort_by_key(|x| -x.0);

    let result1 = solutions.first().context("no answer")?.0;
    let mut result2 = 0;

    for (area, i, j) in solutions {
        let min = [
            squares[i][0].min(squares[j][0]),
            squares[i][1].min(squares[j][1]),
        ];
        let max = [
            squares[i][0].max(squares[j][0]),
            squares[i][1].max(squares[j][1]),
        ];

        if aabb_intersection(&squares, min, max) {
            continue;
        }

        result2 = area;
        break;
    }

    Ok((result1, result2))
}

fn aabb_intersection(squares: &Vec<Vec2>, p1: Vec2, q1: Vec2) -> bool {
    // Each line segment becomes a rectangle and then do a simple AABB test against the target area
    for i in 0..squares.len() {
        let j = (i + 1) % squares.len();
        let p2 = [
            squares[i][0].min(squares[j][0]),
            squares[i][1].min(squares[j][1]),
        ];
        let q2 = [
            squares[i][0].max(squares[j][0]),
            squares[i][1].max(squares[j][1]),
        ];
        // AABB test
        if (p1[0] < q2[0]) && (q1[0] > p2[0]) && (p1[1] < q2[1]) && (q1[1] > p2[1]) {
            return true;
        }
    }
    false
}
