use anyhow::*;

pub static DAY: u32 = 12;
pub static EXAMPLE_INPUT: &str = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

/**
 * You can totally cheese this day. You do not have to do any complicated packing and just checking
 * if the total area of the shapes can fit inside the given region is enough.
 */

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    shapes: Vec<usize>,
}

pub fn main(input: &str) -> Result<(usize, String)> {
    let (shapes, regions) = parse_input(input)?;
    Ok((
        regions
            .into_iter()
            .map(|r| if solve(r, &shapes) { 1 } else { 0 })
            .sum(),
        "Not implemented".into(),
    ))
}

fn solve(region: Region, shapes: &Vec<usize>) -> bool {
    region
        .shapes
        .iter()
        .enumerate()
        .map(|(index, &count)| shapes[index] * count)
        .sum::<usize>()
        < region.width * region.height
}

fn parse_input(input: &str) -> Result<(Vec<usize>, Vec<Region>)> {
    let mut shapes = Vec::new();
    let mut sections = input.split("\n\n");
    let count = sections.clone().count();
    for section in sections.by_ref().take(count - 1) {
        // Just count the number of units
        shapes.push(section.chars().filter(|c| c == &'#').count());
    }
    let regions = sections
        .next()
        .context("missing regions")?
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let width = chars
                .by_ref()
                .take_while(|c| c != &'x')
                .collect::<String>()
                .parse()?;
            let height = chars
                .by_ref()
                .take_while(|c| c != &':')
                .collect::<String>()
                .parse()?;
            let shapes = chars
                .skip(1)
                .collect::<String>()
                .split_ascii_whitespace()
                .map(|str| Ok(str.parse()?))
                .collect::<Result<_>>()?;
            Ok(Region {
                width,
                height,
                shapes,
            })
        })
        .collect::<Result<_>>()?;
    Ok((shapes, regions))
}
