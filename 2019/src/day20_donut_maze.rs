use anyhow::*;
use itertools::Itertools;
use pathfinding::prelude as pathfinding;
use std::collections::BTreeMap;

pub static DAY: u32 = 20;
pub static EXAMPLE_INPUT: &str = "\
/            Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     
";

pub fn main(input: &str) -> Result<(usize, usize)> {
    let map = parse_input(input)?;
    Ok((
        find_shortest_path(&map, false)?,
        find_shortest_path(&map, true)?,
    ))
}

fn find_shortest_path(map: &Map, recursive_spaces: bool) -> Result<usize> {
    let (_, dist) = pathfinding::dijkstra(
        &(map.start, 0),
        |(prev_node, depth)| {
            if let Some(edges) = map.edges.get(prev_node) {
                return edges
                    .iter()
                    .filter_map(|&(node, depth_change)| {
                        if node == *prev_node {
                            return None;
                        }
                        if recursive_spaces {
                            if depth_change < 0 && *depth == 0 {
                                return None;
                            }
                            return Some(((node, *depth + depth_change), 1));
                        } else {
                            return Some(((node, *depth), 1));
                        }
                    })
                    .collect_vec();
            } else {
                vec![]
            }
        },
        |node| node == &(map.end, 0),
    )
    .context("No valid path exists")?;
    return Ok(dist);
}

fn parse_input(input: &str) -> Result<Map> {
    let input = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let mut map = Map::default();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut portals = BTreeMap::<String, Vec<(Pos, bool)>>::new();

    // Find nodes
    for (y, line) in input.iter().enumerate() {
        let mut inside = false;
        let mut empty = true;
        for (x, &c) in line.iter().enumerate() {
            // Detect when we are inside the doughnut or not
            if c == '#' || c == '.' {
                empty = false;
            } else {
                if !empty {
                    inside = !inside;
                }
                empty = true;
            }

            if c == '.' {
                map.nodes.insert((y, x), map.nodes.len());
                continue;
            } else if !c.is_alphabetic() || y + 1 == input.len() || x + 1 == line.len() {
                continue;
            }

            let label;
            let pos;
            if input[y + 1][x].is_alphabetic() {
                // Vertical label
                label = String::from_iter([input[y][x], input[y + 1][x]]);
                if y > 0 && input[y - 1][x] == '.' {
                    pos = (y - 1, x);
                } else {
                    pos = (y + 2, x);
                }
            } else if input[y][x + 1].is_alphabetic() {
                // Horizontal label
                label = String::from_iter([input[y][x], input[y][x + 1]]);
                if x > 0 && input[y][x - 1] == '.' {
                    pos = (y, x - 1);
                } else {
                    pos = (y, x + 2);
                }
            } else {
                continue;
            }

            // Find start and end positions
            if label == "AA" {
                start = pos;
            } else if label == "ZZ" {
                end = pos;
            }

            // Add portal
            if let Some(nodes) = portals.get_mut(&label) {
                nodes.push((pos, inside));
            } else {
                portals.insert(label, vec![(pos, inside)]);
            }
        }
    }

    // Build edges
    for (&(y, x), node) in map.nodes.iter() {
        let mut edges = Vec::new();
        if let Some(&edge) = map.nodes.get(&(y + 1, x)) {
            edges.push((edge, 0));
        }
        if let Some(&edge) = map.nodes.get(&(y, x + 1)) {
            edges.push((edge, 0));
        }
        if let Some(&edge) = map.nodes.get(&(y - 1, x)) {
            edges.push((edge, 0));
        }
        if let Some(&edge) = map.nodes.get(&(y, x - 1)) {
            edges.push((edge, 0));
        }
        map.edges.insert(*node, edges);
    }

    // Connect portals
    for nodes in portals.values() {
        let nodes = nodes
            .iter()
            .map(|(pos, inside)| (*map.nodes.get(pos).unwrap(), if *inside { -1 } else { 1 }))
            .collect_vec();
        for node in nodes.iter() {
            let edges = map.edges.get_mut(&node.0).unwrap();
            for &other in nodes.iter().filter(|&x| x != node) {
                edges.push(other);
            }
        }
    }

    // Set start and end nodes
    map.start = *map.nodes.get(&start).unwrap();
    map.end = *map.nodes.get(&end).unwrap();
    Ok(map)
}

type Pos = (usize, usize);

#[derive(Default, Debug)]
struct Map {
    nodes: BTreeMap<Pos, usize>,
    edges: BTreeMap<usize, Vec<(usize, i64)>>,
    start: usize,
    end: usize,
}
