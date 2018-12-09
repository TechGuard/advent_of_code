use std::io::{self, Read};

extern crate regex;
use regex::Regex;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Expected input");

    let re =
        Regex::new(r"^(?P<players>\d+) players; last marble is worth (?P<max_score>\d+) points$")
            .unwrap();
    let captures = re.captures(&input).unwrap();
    let players = captures["players"].parse::<usize>().unwrap();
    let max_score = captures["max_score"].parse::<usize>().unwrap();

    println!("1st Answer = {}", get_answer1(players, max_score));
}

fn get_answer1(players: usize, max_score: usize) -> usize {
    let mut data = Vec::<usize>::with_capacity(max_score);
    let mut scores = vec![0; players];
    let mut current = 0;

    fn index(data: &Vec<usize>, i: i32) -> usize {
        return if i < 0 {
            (data.len() as i32 + i) as usize
        } else {
            i as usize % data.len()
        };
    };

    data.push(0);

    for i in 1..max_score + 1 {
        if i % 23 == 0 {
            let player = i % players;
            let next = index(&data, current - 7);
            scores[player] += data.remove(next) + i;
            current = next as i32;
        } else {
            let next = index(&data, current + 2);
            data.insert(next, i);
            current = next as i32;
        }
    }

    scores.into_iter().max().unwrap()
}
