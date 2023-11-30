use core::fmt;

use grid::grid;
use itertools::Itertools;

use crate::utils::{Direction, MyGrid, Point};

use crate::utils::Direction::*;

fn parse_line(line: &str) -> Vec<Direction> {
    line.trim()
        .as_bytes()
        .iter()
        .map(|c| match c {
            b'L' => Left,
            b'R' => Right,
            b'U' => Up,
            b'D' => Down,
            _ => panic!("Unknown direction"),
        })
        .collect()
}

fn parse(input: &str) -> Vec<Vec<Direction>> {
    input.lines().map(|line| parse_line(line)).collect()
}

struct Keyboard {
    grid: MyGrid<i8>,
    pos: Point,
}

impl Keyboard {
    fn new_part1() -> Self {
        Keyboard {
            grid: MyGrid(grid![[1, 2, 3][4, 5, 6][7, 8, 9]]),
            pos: Point { x: 1, y: 1 },
        }
    }
    fn new_part2() -> Self {
        Keyboard {
            grid: MyGrid(grid![[-1, -1, 1, -1, -1][-1, 2, 3, 4, -1][5, 6, 7, 8, 9][-1, 0xA, 0xB, 0xC, -1][-1, -1, 0xD, -1, -1]]),
            pos: Point { x: 1, y: 1 },
        }
    }
    
    fn move_in(&mut self, dir: &Direction) {
        let new_pos = self.pos.move1(dir);
        if self.grid.contains(&new_pos) && self.grid[new_pos] != -1 {
            self.pos = new_pos;
        }
    }
    
    fn get_current_number(&self) -> char {
        match self.grid[self.pos] {
            1..=9 => 9.to_string(),
            10..15 => 
        }
    }
    
    fn do_moves(&mut self, moves: Vec<Direction>) -> char {
        moves.iter().for_each(|dir| self.move_in(dir));
        self.get_current_number()
    }
}

pub fn part1(input: &str) -> isize {
    let mut keyboard = Keyboard::new();
    parse(input).into_iter().map(|moves| {
        keyboard.do_moves(moves)
    }).join("").parse().unwrap()
}

pub fn part2(input: &str) -> isize {
    0
}

#[test]
fn test() {
    test_2016!(2, 1985)
}
