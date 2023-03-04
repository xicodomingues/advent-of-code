use std::ops::RangeInclusive;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

type IncRange = RangeInclusive<i64>;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
}

fn parse_line(line: &str) -> (IncRange, IncRange) {
    fn to_i64(groups: &Captures, i: usize) -> i64 {
        groups.get(i).unwrap().as_str().parse().unwrap()
    }
    let groups = RE.captures(line).unwrap();
    let first = IncRange::new(to_i64(&groups, 1), to_i64(&groups, 2));
    let second = IncRange::new(to_i64(&groups, 3), to_i64(&groups, 4));
    (first, second)
}

fn parse(input: &str) -> impl Iterator<Item=(IncRange, IncRange)> + '_ {
    input.lines().map(parse_line)
}

fn is_contained(pair: &(IncRange, IncRange)) -> bool {
    let (a, b) = pair;
    (a.contains(b.start()) && a.contains(b.end())) || (b.contains(a.start()) && b.contains(a.end()))
}

fn overlaps(pair: &(IncRange, IncRange)) -> bool {
    let (a, b) = pair;
    a.contains(b.start()) || b.contains(a.start())
}

fn solve(input: &str, f: fn(&(IncRange, IncRange)) -> bool) -> i64 {
    parse(input).filter(f).count() as i64
}

pub fn part1(input: &str) -> i64 {
    solve(input, is_contained)
}

pub fn part2(input: &str) -> i64 {
    solve(input, overlaps)
}

#[test]
fn test() {
    crate::test_2022!(4, 2, 4);
}
