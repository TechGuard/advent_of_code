pub static DAY: u32 = 25;
pub static EXAMPLE_INPUT: &str = "\
 0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0
";

pub fn main(input: &str) -> (usize, String) {
    let mut points: Vec<(i32, i32, i32, i32)> = input
        .lines()
        .filter_map(|line| {
            let mut numers = line.trim().split(",").filter_map(|x| x.parse().ok());
            Some((
                numers.next()?,
                numers.next()?,
                numers.next()?,
                numers.next()?,
            ))
        })
        .collect();

    let mut ans1 = 0;
    while let Some(start) = points.pop() {
        let mut offset = 0;
        let mut constellation = vec![start];
        while !points.is_empty() && offset < constellation.len() {
            let current = constellation[offset];
            points.retain(|other| {
                let diff = (
                    other.0 - current.0,
                    other.1 - current.1,
                    other.2 - current.2,
                    other.3 - current.3,
                );
                let dist = diff.0.abs() + diff.1.abs() + diff.2.abs() + diff.3.abs();
                if dist <= 3 {
                    constellation.push(*other);
                    false
                } else {
                    true
                }
            });
            offset += 1;
        }
        ans1 += 1;
    }

    (ans1, String::from("Not implemented"))
}
