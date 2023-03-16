use std::collections::HashMap;

use itertools::Itertools;

type Nodes<'a> = HashMap<&'a str, u32>;
type Edges = HashMap<(u32, u32), u32>;

fn parse(input: &str) -> (Nodes, Edges) {
    let mut nodes: HashMap<&str, u32> = HashMap::new();
    let mut edges: HashMap<(u32, u32), u32> = HashMap::new();
    let mut curr = 0;
    input.lines()
        .flat_map(|line| {
            let seq = line.split_whitespace().collect::<Vec<_>>();
            vec![seq[0], seq[2]]
        })
        .for_each(|city| {
            if !nodes.contains_key(city) {
                nodes.insert(city, curr);
                curr += 1;
            }
        });
    input.lines().for_each(|line| {
        let seq = line.split_whitespace().collect::<Vec<_>>();
        let edge1 = *nodes.get(seq[0]).unwrap();
        let edge2 = *nodes.get(seq[2]).unwrap();
        let cost = seq[4].parse().unwrap();
        edges.insert((edge1, edge2), cost);
        edges.insert((edge2, edge1), cost);
    });
    (nodes, edges)
}

fn calculate_cost(edges: &Edges, perm: Vec<u32>) -> Option<u32> {
    let mut sum = 0;
    for win in perm.into_iter().tuple_windows::<(u32, u32)>() {
        sum += edges.get(&win)?;
    }
    Some(sum)
}

fn get_costs(input: &str) -> impl Iterator<Item=u32> {
    let (nodes, edges) = parse(input);
    (0 as u32..nodes.len() as u32).permutations(nodes.len())
        .filter_map(move |perm| calculate_cost(&edges, perm))
}

pub fn part1(input: &str) -> u32 {
    get_costs(input).min().unwrap()
}

pub fn part2(input: &str) -> u32 {
    get_costs(input).max().unwrap()
}

#[test]
fn test() {
    crate::test_2015!(9, 605, 982)
}
