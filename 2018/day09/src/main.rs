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

    println!("1st Answer = {}", get_answer(players, max_score));
    println!("2nd Answer = {}", get_answer(players, max_score * 100));
}

#[derive(Debug, Clone)]
struct Marble {
    prev: usize,
    next: usize,
    value: usize,
}

// LinkedList implementation from:
// https://github.com/Aidiakapi/advent_of_code_2018/blob/master/src/day09.rs
fn get_answer(players: usize, max_score: usize) -> usize {
    let mut data = Vec::<Marble>::with_capacity(max_score);
    let mut scores = vec![0; players];
    let mut current = 0usize;

    data.push(Marble {
        prev: 0,
        next: 0,
        value: 0,
    });

    for i in 1..max_score + 1 {
        if i % 23 == 0 {
            for _ in 0..7 {
                current = data[current].prev;
            }
            let player = i % players;
            scores[player] += data[current].value + i;

            let marble = data[current].clone();
            data[marble.prev].next = marble.next;
            data[marble.next].prev = marble.prev;
            current = marble.next;
        } else {
            current = data[current].next;
            let new = data.len();
            let prev = current;
            let next = data[current].next;
            data.push(Marble {
                prev: prev,
                next: next,
                value: i,
            });
            data[prev].next = new;
            data[next].prev = new;
            current = new;
        }
    }

    scores.into_iter().max().unwrap()
}
