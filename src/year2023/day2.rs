use itertools::Itertools;
use regex::Regex;

use lazy_static::lazy_static;

lazy_static! {
    static ref GAME_RE: Regex = Regex::new(r"Game (\d+): (.*)").unwrap();
}

#[derive(Debug)]
struct Set {
    red: u8,
    green: u8,
    blue: u8,
}

impl Set {
    fn parse(entry: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        entry.split(", ").for_each(|e| {
            if let Some((n, color)) = e.split(' ').collect_tuple() {
                match color {
                    "red" => red = n.parse().unwrap(),
                    "green" => green = n.parse().unwrap(),
                    "blue" => blue = n.parse().unwrap(),
                    _ => panic!(),
                }
            }
        });
        Set { blue, red, green }
    }

    fn in_limit(&self) -> bool {
        self.blue <= 14 && self.red <= 12 && self.green <= 13
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}

impl Game {
    fn parse(line: &str) -> Self {
        let (_, [id, entries]) = GAME_RE.captures(line).unwrap().extract();
        Game {
            id: id.parse().unwrap(),
            sets: entries.split("; ").map(Set::parse).collect(),
        }
    }

    fn in_limit(&self) -> bool {
        self.sets.iter().all(Set::in_limit)
    }

    fn minimum_cubes(&self) -> usize {
        let mut it = self.sets.iter();
        let first = it.next().unwrap();
        let mut red = first.red;
        let mut green = first.green;
        let mut blue = first.blue;
        self.sets.iter().for_each(|it| {
            if it.red > red {
                red = it.red;
            }
            if it.green > green {
                green = it.green;
            }
            if it.blue > blue {
                blue = it.blue;
            }
        });
        red as usize * green as usize * blue as usize
    }
}

fn parse(input: &str) -> impl Iterator<Item = Game> + '_ {
    input.lines().map(Game::parse)
}

pub fn part1(input: &str) -> usize {
    parse(input).filter(Game::in_limit).map(|it| it.id).sum()
}

pub fn part2(input: &str) -> usize {
    parse(input).map(|g| g.minimum_cubes()).sum()
}

#[test]
fn test() {
    test_2023!(2, 8, 2286);
}
