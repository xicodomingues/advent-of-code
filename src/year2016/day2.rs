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
    input.lines().map(parse_line).collect()
}

struct Keyboard {
    grid: MyGrid<char>,
    pos: Point,
}

impl Keyboard {
    fn new_part1() -> Self {
        Keyboard {
            grid: MyGrid(grid![['1', '2', '3']['4', '5', '6']['7', '8', '9']]),
            pos: Point { x: 1, y: 1 },
        }
    }
    fn new_part2() -> Self {
        Keyboard {
            grid: MyGrid(grid![
                ['0', '0', '1', '0', '0']
                ['0', '2', '3', '4', '0']
                ['5', '6', '7', '8', '9']
                ['0', 'A', 'B', 'C', '0']
                ['0', '0', 'D', '0', '0']]),
            pos: Point { x: 0, y: 2 },
        }
    }
    
    fn move_in(&mut self, dir: &Direction) {
        let new_pos = self.pos.move1(dir);
        if self.grid.contains(&new_pos) && self.grid[new_pos] != '0' {
            self.pos = new_pos;
        }
    }
    
    fn get_current_number(&self) -> char {
        match self.grid[self.pos] {
            it @ '1'..='9' => it,
            it @ 'A'..='D' => it,
            it => panic!("{} is not a valid entry in the keypad", it),
        }
    }
    
    fn do_moves(&mut self, moves: Vec<Direction>) -> char {
        moves.iter().for_each(|dir| self.move_in(dir));
        self.get_current_number()
    }
}

pub fn part1(input: &str) -> String {
    let mut keyboard = Keyboard::new_part1();
    parse(input).into_iter().map(|moves| {
        keyboard.do_moves(moves)
    }).join("")
}

pub fn part2(input: &str) -> String {
    let mut keyboard = Keyboard::new_part2();
    parse(input).into_iter().map(|moves| {
        keyboard.do_moves(moves)
    }).join("")
}

#[test]
fn test() {
    test_2016!(2, "1985", "5DB3")
}
