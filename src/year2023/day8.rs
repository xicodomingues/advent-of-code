use std::collections::HashMap;

use itertools::Itertools;
use num::integer::lcm;

#[derive(Debug)]
enum Dir {
    L,
    R,
}

#[derive(Debug)]
struct Network<'a> {
    steps: Vec<Dir>,
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Network<'a> {
    fn parse(input: &'a str) -> Self {
        let mut lines_iter = input.lines();
        let steps = lines_iter
            .next()
            .unwrap()
            .bytes()
            .map(|c| match c {
                b'L' => Dir::L,
                b'R' => Dir::R,
                _ => panic!("wtf!!!"),
            })
            .collect();
        let nodes = lines_iter
            .skip(1)
            .map(|line| {
                let (s, l, r) = line
                    .split(&[' ', '=', '(', ',', ')'])
                    .filter(|x| !x.is_empty())
                    .collect_tuple()
                    .unwrap();
                (s, (l, r))
            })
            .collect();
        Self { steps, nodes }
    }

    fn next(&self, curr: &str, i: usize) -> &str {
        let dir = &self.steps[i % self.steps.len()];
        let (l, r) = self.nodes.get(curr).unwrap();
        match dir {
            Dir::L => l,
            Dir::R => r,
        }
    }

    fn cycle_len(&self, start: &str, end: fn(&str) -> bool) -> usize {
        let mut curr = start;
        let mut i = 0;
        while !end(curr) {
            curr = self.next(curr, i);
            i += 1;
        }
        i
    }
}

pub fn part1(input: &str) -> usize {
    Network::parse(input).cycle_len("AAA", |x| x == "ZZZ")
}

pub fn part2(input: &str) -> usize {
    let net = Network::parse(input);
    net.nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|x| net.cycle_len(x, |x| x.ends_with('Z')))
        .fold(1, lcm)
}

#[test]
fn test() {
    use indoc::indoc;
    assert_eq!(
        part1(indoc! {"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
    "}),
        2
    );

    assert_eq!(
        part1(indoc! {"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    "}),
        6
    );

    assert_eq!(
        part2(indoc! {"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "}),
        6
    );
}
