use itertools::Itertools;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    fn loses_to(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock
        }
    }

    fn wins_against(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper
        }
    }
}

impl From<&str> for Shape {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => panic!("This should not happen!")
        }
    }
}

fn parse(input: &str, mapper: impl Fn(&str, &str) -> (Shape, Shape)) -> Vec<(Shape, Shape)> {
    input.lines().into_iter().map(|x| {
        let (first, second) = x.split(' ').take(2).collect_tuple().unwrap();
        mapper(first, second)
    }).collect()
}

fn score(entry: &(Shape, Shape)) -> i64 {
    let points = match entry {
        (a, b) if &a.loses_to() == b => 6,
        (a, b) if a == b => 3,
        _ => 0
    };
    points + (entry.1 as i64)
}

pub fn part1(input: &str) -> i64 {
    parse(input, |other, me| { (Shape::from(other), Shape::from(me)) })
        .iter().map(score).sum()
}

fn force_result(result: &str, other: Shape) -> Shape {
    match result {
        "X" => other.wins_against(),
        "Y" => other,
        "Z" => other.loses_to(),
        _ => panic!("This should not happen!")
    }
}

pub fn part2(input: &str) -> i64 {
    parse(input, |other, me| {
        let tmp = Shape::from(other);
        (tmp, force_result(me, tmp))
    })
        .iter().map(score).sum()
}

#[test]
fn test() {
    crate::test_day!(2, 15, 12);
}
