use std::str::FromStr;

use grid::Grid;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::{Match, Regex};

use crate::utils::{MyGrid, ParseError, Point};

lazy_static! {
    static ref COORDS_REGEX: Regex = Regex::new(r"(\d+),(\d+) through (\d+),(\d+)").unwrap();
}

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
            .ok_or(ParseError::new("Impossible to parse Coords"))
    }
}

#[derive(Debug)]
enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            x if x.starts_with("turn on") => Ok(Self::TurnOn),
            x if x.starts_with("turn off") => Ok(Self::TurnOff),
            x if x.starts_with("toggle") => Ok(Self::Toggle),
            _ => Err(ParseError::new("Unrecognized instruction")),
        }
    }
}

trait ExecInstruction<T> {
    fn exec(&mut self, coords: Coords, make_change: impl Fn(&T) -> T);
}

impl<T> ExecInstruction<T> for MyGrid<T> {
    fn exec(&mut self, coords: Coords, make_change: impl Fn(&T) -> T) {
        (coords.top.y..=coords.bottom.y)
            .cartesian_product(coords.top.x..=coords.bottom.x)
            .for_each(|p| self[p] = make_change(&self[p]));
    }
}

fn solve<T>(input: &str, make_change: impl Fn(Instruction) -> Box<dyn Fn(&T) -> T>) -> MyGrid<T>
where
    T: Default,
{
    let mut grid: MyGrid<T> = MyGrid(Grid::new(1000, 1000));
    input
        .lines()
        .map(|line| (Instruction::from_str(line), Coords::from_str(line)))
        .for_each(|info| match info {
            (Ok(inst), Ok(coords)) => grid.exec(coords, &make_change(inst)),
            _ => panic!("Faulty instruction"),
        });
    grid
}

pub fn part1(input: &str) -> usize {
    let grid = solve(input, |inst| {
        Box::new(move |x| match inst {
            Instruction::TurnOff => false,
            Instruction::TurnOn => true,
            Instruction::Toggle => !x,
        })
    });
    grid.iter().filter(|x| **x).count()
}

pub fn part2(input: &str) -> usize {
    let grid = solve(input, |inst| {
        Box::new(move |x| match inst {
            Instruction::TurnOff => {
                if x > &1 {
                    x - 1
                } else {
                    0
                }
            }
            Instruction::TurnOn => x + 1,
            Instruction::Toggle => x + 2,
        })
    });
    grid.iter().sum()
}

#[test]
fn test() {
    assert_eq!(part2("turn on 0,0 through 0,0"), 1);
    assert_eq!(part2("toggle 0,0 through 999,999"), 2_000_000);
    crate::test_2015!(6, 1_000_000 - 1000 - 4, 1_000_000 + 2 * 1000 - 4)
}
