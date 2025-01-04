pub static DAY: u32 = 23;
pub static EXAMPLE_INPUT: &str = "\
pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5
";

#[derive(Debug)]
struct Nanobot {
    pos: (i32, i32, i32),
    radius: i32,
}

pub fn main(input: &str) -> (usize, String) {
    let nanobots: Vec<Nanobot> = input
        .lines()
        .filter_map(|line| {
            let mut split = line.split(", ");
            let mut pos = split
                .next()?
                .strip_prefix("pos=<")?
                .strip_suffix(">")?
                .split(",")
                .flat_map(|x| x.parse());
            Some(Nanobot {
                pos: (pos.next()?, pos.next()?, pos.next()?),
                radius: split.next()?.strip_prefix("r=")?.parse().ok()?,
            })
        })
        .collect();

    let largest = nanobots.iter().max_by_key(|b| b.radius).unwrap();
    let ans1 = nanobots
        .iter()
        .filter(|b| {
            let diff = (
                b.pos.0 - largest.pos.0,
                b.pos.1 - largest.pos.1,
                b.pos.2 - largest.pos.2,
            );
            let dist = diff.0.abs() + diff.1.abs() + diff.2.abs();
            dist <= largest.radius
        })
        .count();

    (ans1, String::from("Not implemented"))
}
