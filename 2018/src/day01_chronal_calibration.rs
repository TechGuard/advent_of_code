use std::collections::HashSet;

pub static DAY: u32 = 01;
pub static EXAMPLE_INPUT: &str = "\
+1
-2
+3
+1
";

pub fn main(input: &str) -> (i32, i32) {
    let numbers: Vec<i32> = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    (numbers.iter().sum::<i32>(), get_answer2(&numbers))
}

fn get_answer2(numbers: &Vec<i32>) -> i32 {
    let mut freq = 0;
    let mut frequencies = HashSet::new();

    loop {
        for number in numbers.iter() {
            freq += number;
            if !frequencies.insert(freq) {
                return freq;
            }
        }
    }
}
