use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;

#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate regex;

use chrono::{NaiveDate, NaiveDateTime, Timelike};
use regex::Regex;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct LogEntry {
    timestamp: NaiveDateTime,
    text: String,
}

impl FromStr for LogEntry {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\[(?P<year>\d+)-(?P<month>\d+)-(?P<day>\d+) (?P<hour>\d+):(?P<minute>\d+)\] (?P<text>.+)$").unwrap();
        }
        let capture = RE.captures(s).unwrap();
        let year: i32 = capture["year"].parse()?;
        let month: u32 = capture["month"].parse()?;
        let day: u32 = capture["day"].parse()?;
        let hour: u32 = capture["hour"].parse()?;
        let minute: u32 = capture["minute"].parse()?;
        Ok(LogEntry {
            timestamp: NaiveDate::from_ymd(year, month, day).and_hms(hour, minute, 0),
            text: capture["text"].into(),
        })
    }
}

fn get_guard_id(s: &str) -> Result<u32, std::num::ParseIntError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^Guard #(?P<id>\d+) begins shift$").unwrap();
    }
    let capture = RE.captures(s).unwrap();
    Ok(capture["id"].parse()?)
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Expected input");

    // Parse entries
    let mut entries = input
        .lines()
        .map(LogEntry::from_str)
        .collect::<Result<Vec<LogEntry>, _>>()
        .unwrap();

    // Sort by timestamp
    entries.sort();

    let mut guards = HashMap::<u32, Vec<u32>>::new();
    let mut start_sleeping_at: Option<u32> = None;
    let mut curr_guard: Option<u32> = None;

    // Parse entry logs
    for entry in &entries {
        match entry.text.as_str() {
            "falls asleep" => start_sleeping_at = Some(entry.timestamp.minute()),
            "wakes up" => match curr_guard {
                None => panic!("wakes up without guard"),
                Some(guard_id) => {
                    // Add sleeping hours to guards map
                    if let Some(begin) = start_sleeping_at {
                        start_sleeping_at = None;
                        guards
                            .entry(guard_id)
                            .or_default()
                            .extend(begin..entry.timestamp.minute());
                    }
                }
            },
            s => curr_guard = Some(get_guard_id(s).unwrap()),
        }
    }

    println!("1st Answer = {}", get_answer1(&guards));
    println!("2st Answer = {:?}", get_answer2(&guards));
}

struct AvgModeResult<T> {
    value: T,
    occurrence: u32,
}

// Finds the mode value in the given list of numbers and returns AvgModeResult
fn get_avg_mode<T: std::hash::Hash + Eq + Copy>(numbers: &[T]) -> AvgModeResult<T> {
    let mut occurrences = HashMap::<&T, u32>::new();

    for value in numbers {
        *occurrences.entry(value).or_default() += 1;
    }

    let (value, occurrence) = occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .expect("Cannot compute the mode of zero numbers");

    AvgModeResult {
        value: *value,
        occurrence: occurrence,
    }
}

fn get_answer1(guards: &HashMap<u32, Vec<u32>>) -> u32 {
    // Find the lazy guard
    let (guard, minutes) = guards
        .iter()
        .max_by_key(|(_, minutes)| minutes.len())
        .unwrap();

    guard * get_avg_mode(minutes).value
}

fn get_answer2(guards: &HashMap<u32, Vec<u32>>) -> u32 {
    // Map guards to (guard, AvgModeResult) and max by most occurrences
    let (guard, avg) = guards
        .iter()
        .map(|(guard, minutes)| (guard, get_avg_mode(minutes)))
        .max_by_key(|(_, avg)| avg.occurrence)
        .unwrap();

    guard * avg.value
}
