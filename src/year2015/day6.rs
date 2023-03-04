use std::str::FromStr;

use grid::Grid;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::{Match, Regex};

use crate::utils::{MyGrid, Point};

lazy_static! {
    static ref COORDS_REGEX: Regex = Regex::new(r"(\d+),(\d+) through (\d+),(\d+)").unwrap();
}

#[derive(Debug)]
struct ParseError;

#[derive(Debug)]
struct Coords {
    top: Point,
    bottom: Point,
}

impl FromStr for Coords {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn to_point(x: Match, y: Match) -> Option<Point> {
            let a = x.as_str().parse::<isize>().ok()?;
            let b = y.as_str().parse::<isize>().ok()?;
            Some(Point::from((a, b)))
        }

        COORDS_REGEX
            .captures(s)
            .and_then(
                |cap| match (cap.get(1), cap.get(2), cap.get(3), cap.get(4)) {
                    (Some(a), Some(b), Some(c), Some(d)) => Some(Coords {
                        top: to_point(a, b)?,
                        bottom: to_point(c, d)?,
                    }),
                    _ => None,
                },
            )
            .ok_or(ParseError)
    }
}

#[derive(Debug)]
enum Instruction {
    TurnOn(Coords),
    TurnOff(Coords),
    Toggle(Coords),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = Coords::from_str(s)?;
        match s {
            x if x.starts_with("turn on") => Ok(Self::TurnOn(coords)),
            x if x.starts_with("turn off") => Ok(Self::TurnOff(coords)),
            x if x.starts_with("toggle") => Ok(Self::Toggle(coords)),
            _ => Err(ParseError),
        }
    }
}

trait ExecInstruction {
    fn exec(&mut self, instruction: Instruction);
}

impl ExecInstruction for MyGrid<bool> {
    fn exec(&mut self, instruction: Instruction) {
        fn coords_iter(coords: Coords) -> impl Iterator<Item=(isize, isize)> {
            (coords.top.y..=coords.bottom.y).cartesian_product(coords.top.x..=coords.bottom.x)
        }
        match instruction {
            Instruction::TurnOn(coords) => coords_iter(coords).for_each(|p| self[p] = true),
            Instruction::TurnOff(coords) => coords_iter(coords).for_each(|p| self[p] = false),
            Instruction::Toggle(coords) => coords_iter(coords).for_each(|p| self[p] = !self[p]),
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut grid: MyGrid<bool> = MyGrid(Grid::new(1000, 1000));
    input
        .lines()
        .map(Instruction::from_str)
        .for_each(|instruction| match instruction {
            Ok(inst) => grid.exec(inst),
            _ => panic!("Faulty instruction"),
        });
    grid.iter().filter(|x| **x).count()
}

pub fn part2(_input: &str) -> usize {
    0
}

#[test]
fn test() {
    crate::test_2015!(6, 1_000_000 - 1000 - 4)
}
