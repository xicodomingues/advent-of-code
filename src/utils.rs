use std::cmp::max;
use std::fs;
use std::ops::{Index, IndexMut};

use grid::Grid;

pub fn load_file(filename: &str) -> String {
    fs::read_to_string("data/".to_string() + filename)
        .expect("Should have been able to read the file")
}

#[macro_export]
macro_rules! test_day {
    ($day:literal, $first:expr) => {{
        use $crate::utils::load_file;
        let tmp = load_file(&format!("test{}.txt", $day));
        assert_eq!(part1(&tmp), $first);
    }};

    ($day:literal, $first:expr, $second:expr) => {{
        use $crate::utils::load_file;
        let tmp = load_file(&format!("test{}.txt", $day));
        assert_eq!(part1(&tmp), $first);
        assert_eq!(part2(&tmp), $second);
    }};
}

/// Based on the dbg macro, but without pretty format and without return value
#[macro_export]
macro_rules! my_dbg {
    () => {
        eprintln!("[{}:{}]", file!(), line!());
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                eprintln!("[{}:{}] {} = {:?}",
                    file!(), line!(), stringify!($val), &tmp);
            }
        };
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::my_dbg!($val)),+,);
    };
}


#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Point {
    pub(crate) x: isize,
    pub(crate) y: isize,
}

impl From<(isize, isize)> for Point {
    fn from(value: (isize, isize)) -> Self {
        Point { x: value.0, y: value.1 }
    }
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Point {
            x: isize::try_from(value.0).unwrap(),
            y: isize::try_from(value.1).unwrap(),
        }
    }
}

impl Point {
    pub const ZERO: Point = Point { x: 0, y: 0 };

    pub fn up(&self) -> Self {
        Point { x: self.x, y: self.y + 1 }
    }

    pub fn up_left(&self) -> Self {
        Point { x: self.x - 1, y: self.y + 1 }
    }

    pub fn up_right(&self) -> Self {
        Point { x: self.x + 1, y: self.y + 1 }
    }

    pub fn down(&self) -> Self {
        Point { x: self.x, y: self.y - 1 }
    }

    pub fn down_left(&self) -> Self {
        Point { x: self.x - 1, y: self.y - 1 }
    }

    pub fn down_right(&self) -> Self {
        Point { x: self.x + 1, y: self.y - 1 }
    }

    pub fn left(&self) -> Self {
        Point { x: self.x - 1, y: self.y }
    }

    pub fn right(&self) -> Self {
        Point { x: self.x + 1, y: self.y }
    }

    pub fn neighbors(&self) -> Vec<Self> {
        vec![
            self.up(),
            self.down(),
            self.left(),
            self.right(),
        ]
    }

    pub fn square_dist(&self, other: &Point) -> isize {
        max((self.x - other.x).abs(), (self.y - other.y).abs())
    }
}

pub trait InGrid {
    fn contains(&self, point: &Point) -> bool;
}

impl<T> InGrid for Grid<T> {
    fn contains(&self, point: &Point) -> bool {
        point.x >= 0
            && point.y >= 0
            && (point.x as usize) < self.rows()
            && (point.y as usize) < self.cols()
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self[index.x as usize][index.y as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self[index.x as usize][index.y as usize]
    }
}