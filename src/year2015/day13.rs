use std::collections::HashMap;

use itertools::Itertools;

type Nodes<'a> = HashMap<&'a str, u8>;
type Edges = HashMap<(u8, u8), i32>;

fn parse(input: &str) -> (Nodes, Edges) {
    let mut nodes: Nodes = HashMap::new();
    let mut edges: Edges = HashMap::new();
    let mut curr = 0;
    input.lines()
        .map(|line| line.split_whitespace().next().unwrap())
        .for_each(|name| {
            if !nodes.contains_key(name) {
                nodes.insert(name, curr);
                curr += 1;
            }
        });
    input.lines().for_each(|line| {
        let seq = line.strip_suffix('.').unwrap().split_whitespace().collect::<Vec<_>>();
        let edge1 = *nodes.get(seq[0]).unwrap();
        let edge2 = *nodes.get(seq[10]).unwrap();
        let cost = match seq[2] {
            "gain" => seq[3].parse::<i32>().unwrap(),
            "lose" => -seq[3].parse::<i32>().unwrap(),
            _ => panic!("wtf")
        };
        edges.insert((edge1, edge2), cost);
    });
    (nodes, edges)
}

fn calculate_cost(edges: &Edges, perm: Vec<u8>) -> Option<i32> {
    let mut sum = 0;
    for (a, b) in perm.into_iter().circular_tuple_windows::<(u8, u8)>() {
        sum += edges.get(&(a, b))?;
        sum += edges.get(&(b, a))?;
    }
    Some(sum)
}

fn get_costs(input: &str) -> impl Iterator<Item=i32> {
    let (nodes, edges) = parse(input);
    (0..nodes.len() as u8).permutations(nodes.len())
        .filter_map(move |perm| calculate_cost(&edges, perm))
}

pub fn part1(input: &str) -> i32 {
    get_costs(input).max().unwrap()
}

pub fn part2(input: &str) -> i32 {
    let (mut nodes, mut edges) = parse(input);
    let my_id = nodes.len() as u8;
    for i in 0..nodes.len() {
        edges.insert((i as u8, my_id), 0);
        edges.insert((my_id, i as u8), 0);
    }
    nodes.insert("me", my_id);
    (0..nodes.len() as u8).permutations(nodes.len())
        .filter_map(move |perm| calculate_cost(&edges, perm))
        .max().unwrap()
}

#[test]
fn test() {
    crate::test_2015!(13, 330);
}