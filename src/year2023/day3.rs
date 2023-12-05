use std::cmp::{max, min};

use itertools::{enumerate, Itertools};
use lazy_static::lazy_static;
use regex::Regex;

use crate::utils::{MyGrid, Point};

lazy_static! {
    static ref NUMBER_RE: Regex = Regex::new(r"\d+").unwrap();
}

#[derive(Debug)]
struct Gear {
    value: usize,
    gear: Option<Point>,
    engajed: bool,
}

#[derive(Debug)]
struct Engine {
    grid: MyGrid<char>,
    numbers: Vec<Gear>,
    symbols: Vec<Point>,
}

impl Engine {
    fn parse(input: &str) -> Self {
        fn is_symbol(c: char) -> bool {
            !c.is_ascii_alphanumeric() && c != '.'
        }

        fn check_engajed(
            row: usize,
            start: usize,
            end: usize,
            grid: &MyGrid<char>,
        ) -> Option<Point> {
            let check_row = |r: usize| {
                for col in max(start as isize - 1, 0)..min(end + 1, grid.size().1) as isize {
                    if is_symbol(grid[(r, col as usize)]) {
                        return Some((col as usize, r).into());
                    }
                }
                None
            };

            if row > 0 {
                if let Some(p) = check_row(row - 1) {
                    return Some(p);
                }
            }

            if row < grid.size().0 - 1 {
                if let Some(p) = check_row(row + 1) {
                    return Some(p);
                }
            }

            if start > 0 && is_symbol(grid[(row, start - 1)]) {
                return Some((start - 1, row).into());
            }

            if end < grid.size().1 - 1 && is_symbol(grid[(row, end)]) {
                return Some((end, row).into());
            }
            None
        }

        fn extract_numbers((row, line): (usize, &str), grid: &MyGrid<char>) -> Vec<Gear> {
            NUMBER_RE
                .find_iter(line)
                .map(|m| {
                    let start = m.start();
                    let end = m.end();
                    let value = m.as_str().parse().unwrap();
                    let gear_place = check_engajed(row, start, end, grid);
                    Gear {
                        value,
                        gear: gear_place,
                        engajed: gear_place.is_some(),
                    }
                })
                .collect()
        }

        fn extract_symbols(grid: &MyGrid<char>) -> Vec<Point> {
            grid.indexed_iter()
                .filter(|(_, c)| is_symbol(**c))
                .map(|((r, c), _)| (c, r).into())
                .collect()
        }

        let grid = MyGrid::from_str(input);
        let numbers = enumerate(input.lines())
            .flat_map(|rows| extract_numbers(rows, &grid))
            .collect();
        let symbols = extract_symbols(&grid);
        Engine {
            grid,
            numbers,
            symbols,
        }
    }
}

pub fn part1(input: &str) -> usize {
    Engine::parse(input)
        .numbers
        .iter()
        .filter(|n| n.engajed)
        .map(|n| n.value)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let binding = Engine::parse(input);
    binding
        .numbers
        .iter()
        .filter(|x| x.engajed)
        .into_grouping_map_by(|n| n.gear)
        .collect::<Vec<_>>()
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v.iter().map(|x| x.value).product::<usize>())
        .sum()
}

#[test]
fn test() {
    test_2023!(3, 4361, 467835);
}
