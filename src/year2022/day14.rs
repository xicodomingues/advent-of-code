use std::fmt::{Debug, Formatter};
use std::str::FromStr;

use grid::Grid;
use itertools::{Itertools, MinMaxResult};
use lazy_static::lazy_static;
use regex::Regex;

use crate::utils::{MyGrid, ParseError, Point};

lazy_static! {
    static ref COORDS_RE: Regex = Regex::new(r"\d+,\d+").unwrap();
}

#[derive(Debug)]
struct Sequence {
    points: Vec<Point>,
}

#[derive(Debug, Clone, PartialEq)]
enum Content {
    Air,
    Rock,
    Sand,
}

#[derive(Debug)]
struct Bounds {
    top: isize,
    bottom: isize,
    left: isize,
    right: isize,
}

struct SandPit {
    grid: MyGrid<Content>,
    bounds: Bounds,
}

struct SeqIter<'a> {
    curr: Option<Point>,
    others: &'a [Point],
}

impl Sequence {
    fn iter_pos(&self) -> SeqIter {
        SeqIter {
            curr: None,
            others: &self.points,
        }
    }
}

impl FromStr for Sequence {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Result<_, _> = COORDS_RE.find_iter(s).map(|m| m.as_str().parse()).collect();
        Ok(Sequence { points: points? })
    }
}

impl<'a> Iterator for SeqIter<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.others.len() == 1 && self.curr.as_ref().is_some_and(|x| *x == self.others[0]) {
            return None;
        }
        match &self.curr {
            None => {
                self.curr = Some(self.others[0].clone());
                self.others = &self.others[1..];
                self.curr.clone()
            }
            Some(p) => {
                if *p == self.others[0] {
                    self.others = &self.others[1..];
                }
                let o = self.others[0].clone();
                let dx = p.x - o.x;
                let dy = p.y - o.y;
                let mut new_curr = None;
                if dx > 0 { new_curr = Some(p.left()); }
                if dx < 0 { new_curr = Some(p.right()); }
                if dy > 0 { new_curr = Some(p.up()); }
                if dy < 0 { new_curr = Some(p.down()); }
                self.curr = new_curr.clone();
                new_curr
            }
        }
    }
}

#[cfg(test)]
mod seq_tests {
    use crate::utils::Point;
    use crate::year2022::day14::Sequence;

    #[test]
    fn test_seq_iter_x() {
        let seq = Sequence { points: vec![Point::new(0, 0), Point::new(2, 0)] };
        let mut iter = seq.iter_pos();
        assert_eq!(Some(Point::new(0, 0)), iter.next());
        assert_eq!(Some(Point::new(1, 0)), iter.next());
        assert_eq!(Some(Point::new(2, 0)), iter.next());
        assert_eq!(None, iter.next());

        let seq = Sequence { points: vec![Point::new(2, 0), Point::new(0, 0)] };
        let mut iter = seq.iter_pos();
        assert_eq!(Some(Point::new(2, 0)), iter.next());
        assert_eq!(Some(Point::new(1, 0)), iter.next());
        assert_eq!(Some(Point::new(0, 0)), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_seq_iter_y() {
        let seq = Sequence { points: vec![Point::new(0, 0), Point::new(0, 2)] };
        let mut iter = seq.iter_pos();
        assert_eq!(Some(Point::new(0, 0)), iter.next());
        assert_eq!(Some(Point::new(0, 1)), iter.next());
        assert_eq!(Some(Point::new(0, 2)), iter.next());
        assert_eq!(None, iter.next());

        let seq = Sequence { points: vec![Point::new(0, 2), Point::new(0, 0)] };
        let mut iter = seq.iter_pos();
        assert_eq!(Some(Point::new(0, 2)), iter.next());
        assert_eq!(Some(Point::new(0, 1)), iter.next());
        assert_eq!(Some(Point::new(0, 0)), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_seq_iter_multiple() {
        let seq = Sequence { points: vec![Point::new(0, 0), Point::new(0, 2), Point::new(2, 2), Point::new(2, 0)] };
        let mut iter = seq.iter_pos();
        assert_eq!(Some(Point::new(0, 0)), iter.next());
        assert_eq!(Some(Point::new(0, 1)), iter.next());
        assert_eq!(Some(Point::new(0, 2)), iter.next());
        assert_eq!(Some(Point::new(1, 2)), iter.next());
        assert_eq!(Some(Point::new(2, 2)), iter.next());
        assert_eq!(Some(Point::new(2, 1)), iter.next());
        assert_eq!(Some(Point::new(2, 0)), iter.next());
        assert_eq!(None, iter.next());
    }
}

fn get_bounds(inputs: &[Sequence]) -> Bounds {
    let max_row = inputs.iter().filter_map(|s| s.points.iter().map(|p| p.row()).max()).max().unwrap();
    let cols = inputs.iter().flat_map(|s| s.points.iter().map(|p| p.col()));
    match cols.minmax() {
        MinMaxResult::MinMax(min, max) => Bounds {
            top: 0,
            bottom: max_row + 1,
            left: min,
            right: max + 1,
        },
        _ => panic!("should have a max and a min"),
    }
}

impl SandPit {
    fn init(bounds: Bounds) -> Self {
        SandPit {
            grid: MyGrid(Grid::init((bounds.bottom + 2) as usize,
                                    (bounds.right + 2) as usize,
                                    Content::Air)),
            bounds,
        }
    }

    fn add_rocks(&mut self, seq: &Sequence) {
        seq.iter_pos().for_each(|p| self.grid[&p] = Content::Rock)
    }

    fn out_of_bounds(&self, pos: &Point) -> bool {
        pos.y > self.bounds.bottom || pos.x < self.bounds.left || pos.x > self.bounds.right
    }

    fn drop_sand(&mut self) -> bool {
        let mut pos = Point::new(500, 0);
        if self.grid[&pos] == Content::Sand {
            return false;
        }
        loop {
            let down = pos.down();
            match self.grid[&down] {
                Content::Air => { pos = down }
                _ => {
                    let down_left = pos.down_left();
                    match self.grid[&down_left] {
                        Content::Air => { pos = down_left }
                        _ => {
                            let down_right = pos.down_right();
                            match self.grid[&down_right] {
                                Content::Air => { pos = down_right }
                                _ => {
                                    self.grid[&pos] = Content::Sand;
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
            if self.out_of_bounds(&pos) {
                return false;
            }
        }
    }

    fn fill_sand(&mut self) -> usize {
        let mut i = 0;
        while self.drop_sand() {
            i += 1;
        }
        i
    }
}

impl Debug for SandPit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for r in self.bounds.top..self.bounds.bottom {
            write!(f, "{} ", r)?;
            for c in self.bounds.left..self.bounds.right {
                write!(f, "{}", match self.grid.0[(r as usize, c as usize)] {
                    Content::Air => '.',
                    Content::Rock => '#',
                    Content::Sand => '0',
                })?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<Vec<Sequence>> for SandPit {
    fn from(seqs: Vec<Sequence>) -> Self {
        let bounds = get_bounds(&seqs);
        let mut res = Self::init(bounds);
        seqs.iter().for_each(|seq| res.add_rocks(seq));
        res
    }
}

fn parse(input: &str) -> Vec<Sequence> {
    input.lines().map(|line| {
        line.parse().unwrap_or_else(|_| panic!("Problem parsing line: {}", line))
    }).collect()
}

pub fn part1(input: &str) -> usize {
    let mut pit = SandPit::from(parse(input));
    pit.fill_sand()
}

pub fn part2(input: &str) -> usize {
    let mut seqs = parse(input);
    let bounds = get_bounds(&seqs);
    // Add the bottom line
    seqs.push(Sequence {
        points: vec![
            Point::new(0, bounds.bottom + 1),
            Point::new(bounds.right + bounds.bottom, bounds.bottom + 1),
        ]
    });
    let mut pit = SandPit::from(seqs);
    pit.fill_sand()
}

#[test]
fn test() {
    crate::test_2022!(14, 24, 93)
}
