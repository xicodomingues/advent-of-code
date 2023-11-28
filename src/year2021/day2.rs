use std::str::FromStr;

use crate::utils::Direction::*;
use crate::utils::{Direction, ParseError};

#[derive(Debug)]
struct Command {
    dir: Direction,
    amount: u8,
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals: Vec<_> = s.split(' ').collect();
        let d = match vals[0] {
            "forward" => Right,
            "down" => Down,
            "up" => Up,
            _ => Left,
        };
        Ok(Command {
            dir: d,
            amount: vals[1].parse()?,
        })
    }
}

fn parse(input: &str) -> impl Iterator<Item = Command> + '_ {
    input.lines().map(Command::from_str).map(Result::unwrap)
}

pub fn part1(input: &str) -> isize {
    // (horizontal, detph)
    let tmp = parse(input).fold((0, 0), |acc, com| {
        (
            acc.0
                + match com.dir {
                    Right => com.amount as isize,
                    _ => 0,
                },
            acc.1
                + match com.dir {
                    Down => com.amount as isize,
                    Up => -(com.amount as isize),
                    _ => 0,
                },
        )
    });
    tmp.0 * tmp.1
}

pub fn part2(input: &str) -> isize {
    // (horizontal, depth, aim)
    let tmp = parse(input).fold((0, 0, 0), |acc, com| {
        (
            acc.0
                + match com.dir {
                    Right => com.amount as isize,
                    _ => 0,
                },
            acc.1
                + acc.2 * match com.dir {
                    Right => com.amount as isize,
                    _ => 0,
                },
            acc.2
                + match com.dir {
                    Down => com.amount as isize,
                    Up => -(com.amount as isize),
                    _ => 0,
                },
        )
    });
    tmp.0 * tmp.1
}

#[test]
fn test() {
    test_2021!(2, 150, 900);
}
