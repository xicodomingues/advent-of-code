use std::str::FromStr;

use crate::utils::ParseError;

fn ways_to_sum() -> Vec<(i64, i64, i64, i64)> {
    let mut res = vec![];
    for a in 1..=97 {
        for b in 1..=100 - a {
            for c in 1..=100 - a - b - 1 {
                res.push((a, b, c, 100 - a - b - c))
            }
        }
    }
    res
}

#[derive(Debug)]
struct Ingredients {
    cap: i64,
    dur: i64,
    fla: i64,
    tex: i64,
    cal: i64,
}

impl FromStr for Ingredients {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let helper = line.replace(",", "");
        let seq = helper.split_whitespace().collect::<Vec<_>>();
        Ok(Ingredients {
            cap: seq[2].parse()?,
            dur: seq[4].parse()?,
            fla: seq[6].parse()?,
            tex: seq[8].parse()?,
            cal: seq[10].parse()?,
        })
    }
}

fn parse(input: &str) -> Vec<Ingredients> {
    let res: Result<Vec<_>, _> = input.lines().map(Ingredients::from_str).collect();
    res.unwrap()
}

macro_rules! sum_prop {
    ($field:ident, $ing:expr, $tuple:expr) => {{
        let (a, b, c, d) = $tuple;
        let res = a * $ing[0].$field + b * $ing[1].$field + c * $ing[2].$field + d * $ing[3].$field;
        if res > 0 { res } else { 0 }
    }};
}

fn apply_sum(sum: (i64, i64, i64, i64), ing: &Vec<Ingredients>) -> i64 {
    let cap = sum_prop!(cap, ing, sum);
    let dur = sum_prop!(dur, ing, sum);
    let fla = sum_prop!(fla, ing, sum);
    let tex = sum_prop!(tex, ing, sum);
    cap * dur * fla * tex
}

fn is_500(sum: (i64, i64, i64, i64), ing: &Vec<Ingredients>) -> bool {
    sum_prop!(cal, ing, sum) == 500
}

pub fn part1(input: &str) -> i64 {
    let ing = parse(input);
    ways_to_sum().into_iter().map(|s| {
        apply_sum(s, &ing)
    }).max().unwrap()
}

pub fn part2(input: &str) -> i64 {
    let ing = parse(input);
    ways_to_sum().into_iter().map(|s| {
        if is_500(s, &ing) {
            apply_sum(s, &ing)
        } else {
            0
        }
    }).max().unwrap()
}
