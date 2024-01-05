use core::fmt::Display;
use std::cmp::max;
use std::fmt::Debug;
use std::fs;
use std::num::ParseIntError;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::str::FromStr;

use grid::Grid;

pub fn load_file(filename: &str) -> String {
    fs::read_to_string("data/".to_string() + filename)
        .expect("Should have been able to read the file")
}

pub fn load_test_file(year: usize, day: usize) -> String {
    fs::read_to_string(format!("data/{}/test_files/test{}.txt", year, day))
        .expect("Should have been able to read the file")
}

#[macro_export]
macro_rules! test_year_day {
    ($year:literal, $day:literal, $first:expr) => {{
        use $crate::utils::load_test_file;
        let tmp = load_test_file($year, $day);
        assert_eq!(part1(&tmp), $first);
    }};

    ($year:literal, $day:literal, $first:expr, $second:expr) => {{
        use $crate::utils::load_test_file;
        let tmp = load_test_file($year, $day);
        assert_eq!(part1(&tmp), $first);
        assert_eq!(part2(&tmp), $second);
    }};
}

#[macro_export]
macro_rules! run_year {
    ($year:literal, $day:ident) => {{
        use std::time::Instant;
        use $crate::utils::load_file;

        let tmp = load_file(&format!("{}/{}.txt", $year, stringify!($day)));
        println!(
            "Day {} of {}",
            stringify!($day).strip_prefix("day").unwrap(),
            $year
        );
        let before = Instant::now();
        println!("Part 1: {}", $day::part1(&tmp));
        println!("Part 2: {}", $day::part2(&tmp));
        println!("Took: {:.2?}", before.elapsed());
        println!();
    }};
}

/// Based on the dbg macro, but without pretty format
#[macro_export]
macro_rules! my_dbg {
    () => {
        eprintln!("[{}:{}]", file!(), line!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                eprintln!("[{}:{}] {} = {:?}",
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::my_dbg!($val)),+,)
    };
}

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    pub fn n() -> Self {
        ParseError {
            message: "Error while parsing stuff".into(),
        }
    }

    pub fn new(message: &str) -> Self {
        ParseError {
            message: message.to_string(),
        }
    }
}

impl From<ParseIntError> for ParseError {
    fn from(_source: ParseIntError) -> Self {
        ParseError::new(&_source.to_string())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub enum Rotation {
    Clockwise,
    AntiClockwise,
}

impl Direction {
    pub fn rotate(&self, way: Rotation) -> Direction {
        match way {
            Rotation::Clockwise => match self {
                Self::Up => Self::Right,
                Self::Right => Self::Down,
                Self::Down => Self::Left,
                Self::Left => Self::Up,
            },
            Rotation::AntiClockwise => match self {
                Self::Up => Self::Left,
                Self::Right => Self::Up,
                Self::Down => Self::Right,
                Self::Left => Self::Down,
            },
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        };
        write!(f, "{}", repr)?;
        Ok(())
    }
}

impl Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        };
        write!(f, "{}", repr)?;
        Ok(())
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Point {
    pub(crate) x: isize,
    pub(crate) y: isize,
}

impl From<(isize, isize)> for Point {
    /// (x, y) or (c, r)
    fn from(value: (isize, isize)) -> Self {
        Point {
            x: value.1,
            y: value.0,
        }
    }
}

impl From<(i32, i32)> for Point {
    /// (x, y) or (c, r)
    fn from(value: (i32, i32)) -> Self {
        Point {
            x: value.1 as isize,
            y: value.0 as isize,
        }
    }
}

impl From<(usize, usize)> for Point {
    /// (x, y) or (c, r)
    fn from(value: (usize, usize)) -> Self {
        Point {
            x: isize::try_from(value.1).unwrap(),
            y: isize::try_from(value.0).unwrap(),
        }
    }
}

/// The vertical axis gets positive the more down it goes.
/// The (0, 0) is at the top left corner
///
///             Up(-)
///              ^
///              |
///  left(-) <-- x --> Right(+)
///              |
///              v
///            Down(+)
///
impl Point {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn row(&self) -> isize {
        self.y
    }

    pub fn col(&self) -> isize {
        self.x
    }

    pub fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn up_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y - 1,
        }
    }

    pub fn up_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y - 1,
        }
    }

    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn down_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    pub fn down_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }

    pub fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn neighbors(&self) -> Vec<Self> {
        vec![self.up(), self.down(), self.left(), self.right()]
    }

    pub fn all_neighbors(&self) -> Vec<Self> {
        let mut res = self.neighbors();
        res.extend(vec![
            self.up_left(),
            self.up_right(),
            self.down_left(),
            self.down_right(),
        ]);
        res
    }

    pub fn move_in(&self, dir: &Direction, amount: isize) -> Self {
        Self {
            x: self.x
                + amount
                    * match dir {
                        Direction::Left => -1,
                        Direction::Right => 1,
                        _ => 0,
                    },
            y: self.y
                + amount
                    * match dir {
                        Direction::Down => 1,
                        Direction::Up => -1,
                        _ => 0,
                    },
        }
    }

    #[inline(always)]
    pub fn move1(&self, dir: &Direction) -> Self {
        let Point { x, y } = self;
        match dir {
            Direction::Up => Self { x: *x, y: y - 1 },
            Direction::Down => Self { x: *x, y: y + 1 },
            Direction::Left => Self { x: x - 1, y: *y },
            Direction::Right => Self { x: x + 1, y: *y },
        }
    }

    pub fn square_dist(&self, other: &Point) -> usize {
        max((self.x - other.x).abs(), (self.y - other.y).abs()) as usize
    }

    pub fn manhathan_dist(&self, other: &Point) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
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

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.y, self.x)?;
        Ok(())
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.y, self.x)?;
        Ok(())
    }
}

/// Structure that represents a grid of stuff.
///
/// The top left corner is `(0, 0)` and bottom right is `(width, height)`
pub struct MyGrid<T>(pub Grid<T>);

impl<T: Default> MyGrid<T> {
    pub fn contains(&self, point: &Point) -> bool {
        point.x >= 0
            && point.y >= 0
            && (point.x as usize) < self.cols()
            && (point.y as usize) < self.rows()
    }

    pub fn to_edge(&self, point: &Point, dir: Direction) -> Box<dyn Iterator<Item = &T> + '_> {
        let c = point.x as usize;
        let r = point.y as usize;
        match dir {
            Direction::Up => Box::new(self.iter_col(c).take(r).rev()),
            Direction::Down => Box::new(self.iter_col(c).skip(r + 1)),
            Direction::Left => Box::new(self.iter_row(r).take(c).rev()),
            Direction::Right => Box::new(self.iter_row(r).skip(c + 1)),
        }
    }
}

impl<T: Eq> MyGrid<T> {
    pub fn find(&self, c: T) -> Option<(usize, usize)> {
        self.indexed_iter()
            .find(|(_, x)| x == &&c)
            .map(|(idx, _)| idx)
    }

    pub fn find_all(&self, c: T) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.indexed_iter()
            .filter(move |(_, x)| x == &&c)
            .map(|(idx, _)| idx)
    }
}

impl<T: Default> MyGrid<T> {
    pub fn parse(input: &str, mapper: fn(u8) -> T) -> Self {
        let mut grid = Grid::<T>::new(0, 0);
        input.lines().for_each(|line| {
            grid.push_row(line.trim_end().bytes().map(mapper).collect());
        });
        MyGrid(grid)
    }
}

impl MyGrid<char> {
    pub fn cparse(input: &str) -> Self {
        let mut grid = Grid::new(0, 0);
        input.lines().for_each(|line| {
            grid.push_row(line.trim_end().chars().collect());
        });
        MyGrid(grid)
    }
}

impl MyGrid<u8> {
    pub fn bprint(&self) -> &Self {
        println!();
        for r in 0..self.rows() {
            print!("{:3} ", r);
            for c in 0..self.cols() {
                print!("{}", self.0[(r, c)] as char)
            }
            println!();
        }
        self
    }
}

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

impl<T> Index<&Point> for MyGrid<T> {
    type Output = T;

    fn index(&self, index: &Point) -> &Self::Output {
        &self.0[(index.y as usize, index.x as usize)]
    }
}

impl<T> IndexMut<&Point> for MyGrid<T> {
    fn index_mut(&mut self, index: &Point) -> &mut Self::Output {
        &mut self.0[(index.y as usize, index.x as usize)]
    }
}

impl<T> Index<(isize, isize)> for MyGrid<T> {
    type Output = T;

    fn index(&self, index: (isize, isize)) -> &Self::Output {
        &self.0[(index.0 as usize, index.1 as usize)]
    }
}

impl<T> IndexMut<(isize, isize)> for MyGrid<T> {
    fn index_mut(&mut self, index: (isize, isize)) -> &mut Self::Output {
        &mut self.0[(index.0 as usize, index.1 as usize)]
    }
}

impl<T> Index<(usize, usize)> for MyGrid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<(usize, usize)> for MyGrid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T> core::fmt::Debug for MyGrid<T>
where
    T: core::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for r in 0..self.rows() {
            write!(f, "{:3} ", r)?;
            for c in 0..self.cols() {
                write!(f, "{:?}", self.0[(r, c)])?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Display for MyGrid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for r in 0..self.rows() {
            write!(f, "{:3} ", r)?;
            for c in 0..self.cols() {
                write!(f, "{}", self.0[(r, c)])?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[test]
fn test_my_grid() {
    use indoc::indoc;
    use Direction::*;

    fn p(r: usize, c: usize) -> Point {
        Point::from((r, c))
    }

    let grid = MyGrid::cparse(indoc! {"
        .....
        .F-7.
        .|.|.
        .L-J.
        .....
    "});

    assert_eq!(grid.to_edge(&p(3, 1), Up).collect::<String>(), "|F.");
    assert_eq!(grid.to_edge(&p(3, 0), Right).collect::<String>(), "L-J.");
    assert_eq!(grid.to_edge(&p(1, 3), Down).collect::<String>(), "|J.");
    assert_eq!(grid.to_edge(&p(1, 4), Left).collect::<String>(), "7-F.");
}
