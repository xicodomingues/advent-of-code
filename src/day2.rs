use itertools::Itertools;
use lazy_static::lazy_static;
use regex::{Regex, Match};

lazy_static! {
    static ref BOX_REGEX: Regex = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
}

fn get_val(x: &Match) -> u64 {
	x.as_str().parse::<u64>().expect("Should be parsable")
}

fn ordered_tuple(a: &Match, b: &Match, c: &Match) -> Option<(u64, u64, u64)> {
	let mut values = vec![];
	values.push(get_val(a));
	values.push(get_val(b));
	values.push(get_val(c));
	
	values.sort();
	
	values.into_iter().collect_tuple()
}

fn parse(input: &str) -> impl Iterator<Item = (u64, u64, u64)> + '_ {
    input.lines().map(|line| {
        BOX_REGEX
            .captures(line)
            .and_then(|cap| match (cap.get(1), cap.get(2), cap.get(3)) {
                (Some(a), Some(b), Some(c)) => ordered_tuple(&a, &b, &c),
                _ => None,
            })
			.expect("Every line should be correctly parsed")
    })
}

fn calc_area(dimensions: (u64, u64, u64)) -> u64 {
    let (l, w, h) = dimensions;
    3 * l * w + 2 * w * h + 2 * h * l
}

fn get_ribbon_area(dimensions: (u64, u64, u64)) -> u64 {
    let (l, w, h) = dimensions;
	2 * l + 2 * w + l * w * h
}

pub fn part1(input: &str) -> u64 {
	parse(input).map(calc_area).sum()
	
}

pub fn part2(input: &str) -> u64 {
	parse(input).map(get_ribbon_area).sum()
}

#[test]
fn test() {
    assert_eq!(calc_area((2, 3, 4)), 58);
    assert_eq!(calc_area((1, 1, 10)), 43);
	
	assert_eq!(get_ribbon_area((2, 3, 4)), 34);
	assert_eq!(get_ribbon_area((1, 1, 10)), 14);
	
    crate::test_day!(2, 101, 48)
}
