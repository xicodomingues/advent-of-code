use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

use crate::utils::Direction;
use crate::utils::Direction::*;
use crate::utils::Point;

lazy_static! {
    static ref MOVE_RE: Regex = Regex::new(r"([UDLR]) (\d+)").unwrap();
}

fn parse(input: &str) -> impl Iterator<Item = (Direction, u8)> + '_ {
    input.lines().filter_map(|line| {
        MOVE_RE.captures(line).and_then(|cap| {
            let steps = cap.get(2)?.as_str().parse().ok()?;
            match cap.get(1)?.as_str() {
                "U" => Some((Up, steps)),
                "D" => Some((Down, steps)),
                "L" => Some((Left, steps)),
                "R" => Some((Right, steps)),
                _ => None,
            }
        })
    })
}

fn move_node(new: &Point, tail: &Point) -> Point {
    if new.square_dist(tail) <= 1 {
        return tail.clone();
    }
    let mut res = new.clone();
    res.x += match new.x - tail.x {
        2 => -1,
        -2 => 1,
        _ => 0,
    };
    res.y += match new.y - tail.y {
        2 => -1,
        -2 => 1,
        _ => 0,
    };
    res
}

// could be improved by not creating new Vec every iteration and do it in place,
// but this is fast enough
fn update_propagation(rope: &Vec<Point>, new_head: Point) -> Vec<Point> {
    let mut res = vec![Point::ZERO; 10];
    res[0] = new_head;
    for i in 1..rope.len() {
        res[i] = move_node(&res[i - 1], &rope[i]);
    }
    res
}

pub fn solve(input: &str, rope_size: usize) -> u64 {
    let mut rope = vec![Point::ZERO; rope_size];
    let mut all_tail_positions = HashSet::new();
    parse(input).for_each(|(dir, steps)| {
        for _ in 0..steps {
            let new_head = match dir {
                Up => rope[0].up(),
                Down => rope[0].down(),
                Left => rope[0].left(),
                Right => rope[0].right(),
            };
            rope = update_propagation(&rope, new_head);
            all_tail_positions.insert(rope[rope_size - 1].clone());
        }
    });
    all_tail_positions.len() as u64
}

pub fn part1(input: &str) -> u64 {
    solve(input, 2)
}

pub fn part2(input: &str) -> u64 {
    solve(input, 10)
}

#[test]
fn test() {
    assert_eq!(part1("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2"), 13);
    crate::test_2022!(9, 88, 36)
}
