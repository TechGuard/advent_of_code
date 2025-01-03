use std::collections::HashMap;
use std::collections::HashSet;

pub static DAY: u32 = 07;
pub static EXAMPLE_INPUT: &str = "";

pub fn main(input: &str) -> (String, u32) {
    // Build requirements map
    let mut requirements = HashMap::<char, Vec<char>>::new();
    input.lines().for_each(|s| {
        let parent = s.chars().nth(5).unwrap();
        let child = s.chars().nth(36).unwrap();
        requirements.entry(child).or_default().push(parent);
        requirements.entry(parent).or_default();
    });

    (get_answer1(&requirements), get_answer2(&requirements))
}

fn get_eligible(requirements: &HashMap<char, Vec<char>>, locations: &HashSet<char>) -> Vec<char> {
    // Find all eligible locations
    let mut eligible = vec![];
    requirements
        .iter()
        // Ignore processed locations
        .filter(|(c, _)| !locations.contains(c))
        // Find locations without requirements
        .filter(|(_, parents)| parents.iter().all(|p| locations.contains(p)))
        // Add to eligible array
        .for_each(|(&c, _)| eligible.push(c));

    // Sort alphabetically
    eligible.sort();
    return eligible;
}

fn get_answer1(requirements: &HashMap<char, Vec<char>>) -> String {
    let mut locations = HashSet::<char>::new();
    let mut result = vec![];

    loop {
        let eligible = get_eligible(requirements, &locations);
        match eligible.iter().next() {
            None => break, // All locations processed
            Some(&c) => {
                result.push(c);
                locations.insert(c);
            }
        }
    }
    result.iter().collect::<String>()
}

#[derive(Default, Debug, Clone)]
struct Worker {
    location: char,
    time_remaining: i32,
    working: bool,
}

fn get_answer2(requirements: &HashMap<char, Vec<char>>) -> u32 {
    let mut workers: Vec<Worker> = vec![Default::default(); 5];
    let mut locations = HashSet::<char>::new();
    let mut working = HashSet::<char>::new();
    let mut time = 0;

    loop {
        let mut not_working = true;

        // Process workers
        for worker in &mut workers {
            if worker.working {
                worker.time_remaining -= 1;
                if worker.time_remaining == 0 {
                    worker.working = false;
                    locations.insert(worker.location);
                    working.remove(&worker.location);
                } else {
                    not_working = false;
                }
            }
        }

        // Done if no eligible locations and no workers are workin
        let eligible = get_eligible(requirements, &locations)
            .into_iter()
            .filter(|c| !working.contains(c))
            .collect::<Vec<_>>();
        if eligible.is_empty() && not_working {
            break;
        }

        // Assign new tasks
        let mut itr = eligible.iter();
        for worker in workers.iter_mut().filter(|w| !w.working) {
            match itr.next() {
                None => break, // All locations processed
                Some(&c) => {
                    working.insert(c);
                    worker.location = c;
                    worker.working = true;
                    worker.time_remaining = c as i32 - 'A' as i32 + 61;
                }
            }
        }

        time += 1;
    }

    time
}
