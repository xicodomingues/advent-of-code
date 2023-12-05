use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static ref CARD_RE: Regex = Regex::new(r"Card: (\d+):\s+([^\|])\s+\|\s+(.+)").unwrap();
}

struct Cards {
    winning: HashSet<u32>,
    mine: HashSet<u32>,
}

pub fn part1(input: &str) -> usize {
    0
}

pub fn part2(input: &str) -> usize {
    0
}

#[test]
fn test() {
    test_2023!(4, 13);
}
