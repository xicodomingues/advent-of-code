use std::fs;

pub fn load_file(filename: &str) -> String {
    fs::read_to_string("data/".to_string() + filename)
        .expect("Should have been able to read the file")
}

#[macro_export]
macro_rules! test_day {
    ( $day:literal, $first:literal) => {{
        use $crate::utils::load_file;
        let tmp = load_file(&format!("test{}.txt", $day));
        assert_eq!(part1(&tmp), $first);
    }};

    ( $day:literal, $first:literal, $second:literal) => {{
        use $crate::utils::load_file;
        let tmp = load_file(&format!("test{}.txt", $day));
        assert_eq!(part1(&tmp), $first);
        assert_eq!(part2(&tmp), $second);
    }};
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        Point { x: value.0, y: value.1 }
    }
}

impl Point {
    pub const ZERO: Point = Point { x: 0, y: 0 };
    
    pub fn up(&self) -> Self {
        Point { x: self.x, y: self.y + 1 }
    }
    
    pub fn down(&self) -> Self {
        Point { x: self.x, y: self.y - 1 }
    }
    
    pub fn left(&self) -> Self {
        Point { x: self.x - 1, y: self.y }
    }
    
    pub fn right(&self) -> Self {
        Point { x: self.x + 1, y: self.y }
    }
}