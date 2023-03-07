use std::cmp::max;
use std::fs;
use std::num::ParseIntError;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::str::FromStr;

use grid::Grid;

pub fn load_file(filename: &str) -> String {
    fs::read_to_string("data/".to_string() + filename)
        .expect("Should have been able to read the file")
}

#[macro_export]
macro_rules! test_year_day {
    ($year:literal, $day:literal, $first:expr) => {{
        use $crate::utils::load_file;
        let tmp = load_file(&format!("{}/test_files/test{}.txt", $year, $day));
        assert_eq!(part1(&tmp), $first);
    }};

    ($year:literal, $day:literal, $first:expr, $second:expr) => {{
        use $crate::utils::load_file;
        let tmp = load_file(&format!("{}/test_files/test{}.txt", $year, $day));
        assert_eq!(part1(&tmp), $first);
        assert_eq!(part2(&tmp), $second);
    }};
}

#[macro_export]
macro_rules! run_year {
    ($year:literal, $day:ident) => {{
        use $crate::utils::load_file;
        use std::time::Instant;

        let tmp = load_file(&format!("{}/{}.txt", $year, stringify!($day)));
        println!("Day {} - {}", stringify!($day).strip_prefix("day").unwrap(), $year);
        let before = Instant::now();
        println!("Part 1: {}", $day::part1(&tmp));
        println!("Part 2: {}", $day::part2(&tmp));
        println!("Took: {:.2?}", before.elapsed());
    }};
}

/// Based on the dbg macro, but without pretty format and without return value
#[macro_export]
macro_rules! my_dbg {
    () => {
        eprintln!("[{}:{}]", file!(), line!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                eprintln!("[{}:{}] {} = {:?}",
                    file!(), line!(), stringify!($val), &tmp)
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::my_dbg!($val)),+,)
    };
}

#[derive(Debug)]
pub struct ParseError;

impl From<ParseIntError> for ParseError {
    fn from(_source: ParseIntError) -> Self {
        ParseError
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Point {
    pub(crate) x: isize,
    pub(crate) y: isize,
}

impl From<(isize, isize)> for Point {
    fn from(value: (isize, isize)) -> Self {
        Point { x: value.0, y: value.1 }
    }
}

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        Point { x: value.0 as isize, y: value.1 as isize }
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

    pub fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    pub fn row(&self) -> isize {
        self.y
    }

    pub fn col(&self) -> isize {
        self.x
    }

    pub fn up(&self) -> Self {
        Point { x: self.x, y: self.y - 1 }
    }

    pub fn up_left(&self) -> Self {
        Point { x: self.x - 1, y: self.y - 1 }
    }

    pub fn up_right(&self) -> Self {
        Point { x: self.x + 1, y: self.y - 1 }
    }

    pub fn down(&self) -> Self {
        Point { x: self.x, y: self.y + 1 }
    }

    pub fn down_left(&self) -> Self {
        Point { x: self.x - 1, y: self.y + 1 }
    }

    pub fn down_right(&self) -> Self {
        Point { x: self.x + 1, y: self.y + 1 }
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

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();
        assert_eq!(coords.len(), 2);
        Ok(Point {
            x: coords[0].parse()?,
            y: coords[1].parse()?,
        })
    }
}

/// Structure that represents a grid of stuff.
///
/// The top left corner is `(0, 0)` and bottom left is `(width, height)`
#[derive(Debug)]
pub struct MyGrid<T>(pub Grid<T>);

impl<T> Deref for MyGrid<T> {
    type Target = Grid<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyGrid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> MyGrid<T> {
    pub fn contains(&self, point: &Point) -> bool {
        point.x >= 0
            && point.y >= 0
            && (point.x as usize) < self.cols()
            && (point.y as usize) < self.rows()
    }
}

impl<T> Index<Point> for MyGrid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self.0[index.y as usize][index.x as usize]
    }
}

impl<T> IndexMut<Point> for MyGrid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.0[index.y as usize][index.x as usize]
    }
}

impl<T> Index<(isize, isize)> for MyGrid<T> {
    type Output = T;

    fn index(&self, index: (isize, isize)) -> &Self::Output {
        &self.0[index.0 as usize][index.1 as usize]
    }
}

impl<T> IndexMut<(isize, isize)> for MyGrid<T> {
    fn index_mut(&mut self, index: (isize, isize)) -> &mut Self::Output {
        &mut self.0[index.0 as usize][index.1 as usize]
    }
}

impl<T> Index<(usize, usize)> for MyGrid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for MyGrid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[index.0][index.1]
    }
}