use std::slice::Iter;

pub static DAY: u32 = 08;
pub static EXAMPLE_INPUT: &str = "";

pub fn main(input: &str) -> (u32, u32) {
    // Parse input
    let data = input
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    // Read all nodes
    let root = read_node(&mut data.iter());

    (root.get_answer1(), root.get_answer2())
}

#[derive(Debug, Default)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn get_answer1(&self) -> u32 {
        let total = self.children.iter().map(|c| c.get_answer1()).sum::<u32>();
        total + self.metadata.iter().sum::<u32>()
    }

    fn get_answer2(&self) -> u32 {
        if self.children.is_empty() {
            return self.metadata.iter().sum();
        }

        let mut total = 0;
        for &idx in &self.metadata {
            if idx > 0 && idx <= self.children.len() as u32 {
                total += self.children[(idx - 1) as usize].get_answer2();
            }
        }
        return total;
    }
}

fn read_node(mut data: &mut Iter<u32>) -> Node {
    let child_count = data.next().unwrap();
    let meta_count = data.next().unwrap();

    let mut children = vec![];
    for _ in 0..*child_count {
        children.push(read_node(&mut data));
    }

    Node {
        children: children,
        metadata: data.take(*meta_count as usize).map(|x| *x).collect(),
    }
}
