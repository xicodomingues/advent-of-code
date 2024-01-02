use std::collections::{hash_map::Entry, HashMap};

use grid::Grid;
use itertools::Itertools;

use crate::utils::{Direction, MyGrid};

struct Dish {
    data: MyGrid<u8>,
}

fn move_rocks(iter: impl Iterator<Item = u8>) -> Vec<u8> {
    let mut round = 0;
    let mut res = vec![];
    let mut i = 0;
    let mut last_square = 0;
    for x in iter {
        match x {
            b'O' => round += 1,
            b'#' => {
                (0..round).for_each(|_| res.push(b'O'));
                (round..i - last_square).for_each(|_| res.push(b'.'));
                res.push(b'#');
                last_square = i + 1;
                round = 0;
            }
            _ => {}
        }
        i += 1;
    }
    (0..round).for_each(|_| res.push(b'O'));
    (round..i - last_square).for_each(|_| res.push(b'.'));
    res
}

#[test]
fn test_move() {
    assert_eq!(move_rocks("....OO..".bytes()), "OO......".as_bytes());
    assert_eq!(move_rocks(".#.O#.O.".bytes()), ".#O.#O..".as_bytes());
    assert_eq!(move_rocks("...O#.O.".bytes()), "O...#O..".as_bytes());
}

impl Dish {
    fn parse(input: &str) -> Self {
        Self {
            data: MyGrid::parse(input, |x| x),
        }
    }

    fn tilt(&mut self, dir: Direction) {
        let mut new_grid = MyGrid(Grid::<u8>::new(0, 0));
        match dir {
            Direction::Up => {
                for col in self.data.iter_cols() {
                    new_grid.push_row(move_rocks(col.cloned()))
                }
                new_grid.transpose();
            }
            Direction::Down => {
                self.data.rotate_half();
                for col in self.data.iter_cols() {
                    new_grid.push_row(move_rocks(col.cloned()))
                }
                new_grid.transpose();
                new_grid.rotate_half();
            }
            Direction::Left => {
                self.data.rotate_right();
                for col in self.data.iter_cols() {
                    new_grid.push_row(move_rocks(col.cloned()))
                }
                new_grid.transpose();
                new_grid.rotate_left();
            }
            Direction::Right => {
                self.data.rotate_left();
                for col in self.data.iter_cols() {
                    new_grid.push_row(move_rocks(col.cloned()))
                }
                new_grid.transpose();
                new_grid.rotate_right();
            }
        }
        self.data = new_grid;
    }

    fn load(&self) -> usize {
        self.data
            .iter_rows()
            .enumerate()
            .map(|(i, row)| row.filter(|x| x == &&b'O').count() * (self.data.rows() - i))
            .sum()
    }

    fn repr(&self) -> Vec<u128> {
        self.data
            .iter_rows()
            .map(|row| {
                let bin_str = &row.map(|x| if x == &b'O' { 0 } else { 1 }).join("");
                u128::from_str_radix(bin_str, 2).unwrap()
            })
            .collect_vec()
    }
}

pub fn part1(input: &str) -> usize {
    let mut dish = Dish::parse(input);
    dish.tilt(Direction::Up);
    dish.load()
}

pub fn part2(input: &str) -> usize {
    let mut dish = Dish::parse(input);
    let mut dir = Direction::Up;
    let mut encounterd: HashMap<Vec<u128>, usize> = HashMap::new();
    let mut saved = vec![0];
    let mut i = 1;
    let mut cycles = 0;
    let last = loop {
        dish.tilt(dir);
        dir = dir.rotate(crate::utils::Rotation::AntiClockwise);
        if i % 4 == 0 {
            cycles += 1;
            saved.push(dish.load());
            let repr = dish.repr();
            if let Entry::Vacant(e) = encounterd.entry(repr.clone()) {
                e.insert(cycles);
            } else {
                break *encounterd.get(&repr).unwrap();
            }
        }
        i += 1;
    };
    let cycle_len = cycles - last;
    saved[((1000000000 - last) % cycle_len) + last]
}

#[test]
fn test() {
    test_2023!(14, 136, 64);
}
