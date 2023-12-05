use grid::Grid;

use crate::utils::{Point, MyGrid};

struct Gear {
    value: usize,
    span: Vec<Point>,
    engajed: bool,
}

struct Engine {
    grid: MyGrid<char>,
    numbers: Vec<Gear>,
    symbols: Vec<Point>,
}

impl Engine {
    fn parse(input: &str) -> Self {
        let grid = MyGrid::from_str(input);
        
    }
}

pub fn part1(input: &str) -> usize {
    0
}

pub fn part2(input: &str) -> usize {
    0
}

#[test]
fn test() {
    test_2023!(3, 4361);
}
