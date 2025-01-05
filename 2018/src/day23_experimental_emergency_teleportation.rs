pub static DAY: u32 = 23;
pub static EXAMPLE_INPUT: &str = "\
pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5
";

#[derive(Debug, Clone)]
struct Nanobot {
    pos: (i32, i32, i32),
    radius: i32,
}

impl Nanobot {
    fn in_range(&self, other: &Self) -> bool {
        let diff = (
            self.pos.0 - other.pos.0,
            self.pos.1 - other.pos.1,
            self.pos.2 - other.pos.2,
        );
        let dist = diff.0.abs() + diff.1.abs() + diff.2.abs();
        dist <= other.radius
    }

    fn in_range_box(&self, min: &(i32, i32, i32), max: &(i32, i32, i32)) -> bool {
        // get box closest point to sphere center by clamping
        let x = min.0.max(self.pos.0.min(max.0));
        let y = min.1.max(self.pos.1.min(max.1));
        let z = min.2.max(self.pos.2.min(max.2));
        // check if point is in range
        let diff = (self.pos.0 - x, self.pos.1 - y, self.pos.2 - z);
        let dist = diff.0.abs() + diff.1.abs() + diff.2.abs();
        dist <= self.radius
    }
}

pub fn main(input: &str) -> (usize, i32) {
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

    // Part 1
    let largest = nanobots.iter().max_by_key(|b| b.radius).unwrap();
    let ans1 = nanobots.iter().filter(|b| b.in_range(largest)).count();

    // Part 2: Binary search AABB
    let mut min = nanobots
        .iter()
        .map(|Nanobot { pos, radius }| (pos.0 - radius, pos.1 - radius, pos.2 - radius))
        .min()
        .unwrap();
    let mut max = nanobots
        .iter()
        .map(|Nanobot { pos, radius }| (pos.0 + radius + 1, pos.1 + radius + 1, pos.2 + radius + 1))
        .max()
        .unwrap();
    let ans2 = loop {
        // Found solution when box size is zero (aka a point)
        let size = (max.0 - min.0, max.1 - min.1, max.2 - min.2);
        if size.0 == 0 && size.1 == 0 && size.2 == 0 {
            break min.0.abs() + min.1.abs() + min.2.abs();
        }

        let mut split_min = vec![(0, 0, 0); 2];
        let mut split_max = vec![(0, 0, 0); 2];

        // Split on largest axis
        if size.0 >= size.1 && size.0 >= size.2 {
            let split = (min.0 + max.0) / 2;
            split_min[0] = (min.0, min.1, min.2);
            split_min[1] = (split + 1, min.1, min.2);
            split_max[0] = (split, max.1, max.2);
            split_max[1] = (max.0, max.1, max.2);
        } else if size.1 >= size.0 && size.1 >= size.2 {
            let split = (min.1 + max.1) / 2;
            split_min[0] = (min.0, min.1, min.2);
            split_min[1] = (min.0, split + 1, min.2);
            split_max[0] = (max.0, split, max.2);
            split_max[1] = (max.0, max.1, max.2);
        } else {
            let split = (min.2 + max.2) / 2;
            split_min[0] = (min.0, min.1, min.2);
            split_min[1] = (min.0, min.1, split + 1);
            split_max[0] = (max.0, max.1, split);
            split_max[1] = (max.0, max.1, max.2);
        }

        // For each section find all overlapping nanobots
        let (next_index, _) = (0..2)
            .map(|i| {
                nanobots
                    .iter()
                    .filter(|b| b.in_range_box(&split_min[i], &split_max[i]))
                    .count()
            })
            .enumerate()
            .rev()
            .max_by_key(|(_, b)| *b)
            .unwrap();

        // Continue with largest section (if there's a tie prefer index 0)
        min = split_min[next_index];
        max = split_max[next_index];
    };
    (ans1, ans2)
}
