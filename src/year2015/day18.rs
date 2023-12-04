use std::str::FromStr;

use grid::Grid;
use itertools::{iproduct, Itertools};

use crate::utils::{MyGrid, ParseError, Point};

use rayon::prelude::*;

#[derive(Default, PartialEq, Eq, Clone, Copy)]
enum Light {
    On,
    #[default]
    Off,
}

impl std::fmt::Debug for Light {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::On => write!(f, "#"),
            Self::Off => write!(f, "."),
        }
    }
}

#[derive(Debug)]
struct Lights {
    grid: MyGrid<Light>,
    width: usize,
    height: usize,
}

impl FromStr for Lights {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut grid = MyGrid(Grid::new(0, 0));
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => Light::On,
                        '.' => Light::Off,
                        _ => panic!("Unknown light char: {}", c),
                    })
                    .collect::<Vec<_>>()
            })
            .for_each(|row| grid.push_row(row));
        Ok(Lights {
            width: grid.cols(),
            height: grid.rows(),
            grid,
        })
    }
}

impl Lights {
    fn tick(&mut self) {
        let bla = iproduct!(0..self.height, 0..self.width)
            .collect_vec()
            .into_par_iter()
            .map(|(r, c)| Point::from((c, r)))
            .map(|point| {
                let on_lights = point
                    .all_neighbors()
                    .into_iter()
                    .filter(|p| self.grid.contains(p))
                    .map(|p| self.grid[p])
                    .filter(|l| *l == Light::On)
                    .count();
                match self.grid[point] {
                    Light::Off => {
                        if on_lights == 3 {
                            Light::On
                        } else {
                            Light::Off
                        }
                    }
                    Light::On => {
                        if on_lights == 2 || on_lights == 3 {
                            Light::On
                        } else {
                            Light::Off
                        }
                    }
                }
            })
            .collect::<Vec<_>>();
        self.grid = MyGrid(Grid::from_vec(bla, self.width))
    }

    fn part2_tick(&mut self) {
        self.tick();
        self.grid[(0_isize, 0)] = Light::On;
        self.grid[(self.height - 1, 0)] = Light::On;
        self.grid[(0, self.width - 1)] = Light::On;
        self.grid[(self.height - 1, self.width - 1)] = Light::On;
    }
}

fn parse(input: &str) -> Lights {
    Lights::from_str(input).unwrap()
}

fn solve(input: &str, tick_fn: fn(&mut Lights) -> ()) -> usize {
    let mut lights = parse(input);
    for _ in 0..100 {
        tick_fn(&mut lights);
    }
    lights
        .grid
        .iter()
        .filter(|light| **light == Light::On)
        .count()
}

pub fn part1(input: &str) -> usize {
    solve(input, Lights::tick)
}

pub fn part2(input: &str) -> usize {
    solve(input, Lights::part2_tick)
}

#[test]
fn test() {
    crate::test_2015!(18, 4);
}
