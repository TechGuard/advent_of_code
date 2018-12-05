use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Expected input");

    println!("1st Answer = {}", react(&input).len());
    println!("2st Answer = {}", get_answer2(&input));
}

fn react(s: &str) -> Vec<char> {
    let mut i = 0;
    let mut chars = s.chars().collect::<Vec<_>>();

    while i < chars.len() - 1 {
        let a = chars[i];
        let b = chars[i + 1];
        if a != b && a.eq_ignore_ascii_case(&b) {
            chars.drain(i..i + 2);
            if i > 0 {
                i -= 1;
            }
        } else {
            i += 1;
        }
    }
    chars
}

fn get_answer2(s: &str) -> usize {
    (b'a'..b'z' + 1)
        .map(|c| c as char)
        .map(|ignore_c| {
            let chars = s
                .chars()
                .filter(|c| !ignore_c.eq_ignore_ascii_case(c))
                .collect::<String>();
            react(&chars)
        })
        .min_by_key(|polymer| polymer.len())
        .unwrap()
        .len()
}
