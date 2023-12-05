use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static ref CARD_RE: Regex = Regex::new(r"Card\s+(\d+):([^|]+)\|(.+)").unwrap();
}

#[derive(Debug)]
struct Cards {
    id: u32,
    wins: usize,
}

impl Cards {
    fn parse(line: &str) -> Self {
        fn parse_numbers(input: &str) -> HashSet<u32> {
            input
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        }
        let (_, [id, win, mine]) = CARD_RE.captures(line).unwrap().extract();
        let winning = parse_numbers(win);
        let mine = parse_numbers(mine);
        Cards {
            id: id.parse().unwrap(),
            wins: winning.intersection(&mine).count(),
        }
    }

    fn get_points(&self) -> usize {
        let len = self.wins;
        if len == 0 {
            return 0;
        }
        1 << (len - 1)
    }
}

fn parse(input: &str) -> impl Iterator<Item = Cards> + '_ {
    input.lines().map(Cards::parse)
}

pub fn part1(input: &str) -> usize {
    parse(input).map(|c| c.get_points()).sum()
}

pub fn part2(input: &str) -> usize {
    let games = parse(input).collect_vec();
    let mut times = vec![1; games.len()];
    for game in games {
        let id = (game.id - 1) as usize;
        for i in 1..=game.wins {
            times[i + id] += times[id];
        }
    }
    times.iter().sum()
}

#[test]
fn test() {
    test_2023!(4, 13, 30);
}
