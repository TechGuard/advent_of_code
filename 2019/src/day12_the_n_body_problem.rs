use anyhow::*;

pub static DAY: u32 = 12;
pub static EXAMPLE_INPUT: &str = "\
<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>
";

pub fn main(input: &str) -> Result<(i64, i64)> {
    let mut moons = Moons::default();
    for line in input.lines() {
        let mut pos = line[1..line.len() - 1].split(", ").map(|s| s[2..].parse());
        moons
            .x
            .push((pos.next().context("Expected 3 positions")??, 0));
        moons
            .y
            .push((pos.next().context("Expected 3 positions")??, 0));
        moons
            .z
            .push((pos.next().context("Expected 3 positions")??, 0));
    }

    let mut ans1 = 0;
    let start_x = moons.x.clone();
    let start_y = moons.y.clone();
    let start_z = moons.z.clone();
    let mut repeat_x = None;
    let mut repeat_y = None;
    let mut repeat_z = None;

    for step in 1.. {
        moons.update();

        // Find answer to part 1
        if step == 1000 {
            ans1 = moons.total_energy();
        }

        // Record when positions repeat per axis
        if repeat_x.is_none() && start_x == moons.x {
            repeat_x = Some(step);
        }
        if repeat_y.is_none() && start_y == moons.y {
            repeat_y = Some(step);
        }
        if repeat_z.is_none() && start_z == moons.z {
            repeat_z = Some(step);
        }

        // Exit when all axes repeat
        if repeat_x.is_some() && repeat_y.is_some() && repeat_z.is_some() {
            break;
        }
    }

    let repeat_x = repeat_x.unwrap();
    let repeat_y = repeat_y.unwrap();
    let repeat_z = repeat_z.unwrap();

    Ok((ans1, lcm(lcm(repeat_x, repeat_y), repeat_z)))
}

impl Moons {
    fn update(&mut self) {
        // for each pair adjust velocity
        for i in 0..self.x.len() {
            for j in (i + 1)..self.x.len() {
                let diff_x = (self.x[j].0 - self.x[i].0).signum();
                self.x[i].1 += diff_x;
                self.x[j].1 -= diff_x;

                let diff_y = (self.y[j].0 - self.y[i].0).signum();
                self.y[i].1 += diff_y;
                self.y[j].1 -= diff_y;

                let diff_z = (self.z[j].0 - self.z[i].0).signum();
                self.z[i].1 += diff_z;
                self.z[j].1 -= diff_z;
            }
        }

        // apply velocity
        for i in 0..self.x.len() {
            self.x[i].0 += self.x[i].1;
            self.y[i].0 += self.y[i].1;
            self.z[i].0 += self.z[i].1;
        }
    }

    fn total_energy(&self) -> i64 {
        let mut total = 0;
        for i in 0..self.x.len() {
            let potential = self.x[i].0.abs() + self.y[i].0.abs() + self.z[i].0.abs();
            let kinetic = self.x[i].1.abs() + self.y[i].1.abs() + self.z[i].1.abs();
            total += potential * kinetic;
        }
        total
    }
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

#[derive(Default)]
struct Moons {
    x: Vec<(i64, i64)>,
    y: Vec<(i64, i64)>,
    z: Vec<(i64, i64)>,
}
