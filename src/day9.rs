use std::collections::HashSet;
use lazy_static::lazy_static;
use regex::Regex;
use crate::day9::Direction::*;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

lazy_static! {
    static ref MOVE_RE: Regex = Regex::new(r"([UDLR]) (\d+)").unwrap();
}

fn parse(input: &str) -> Vec<(Direction, u8)> {
    input.lines().filter_map(|line| {
        MOVE_RE.captures(line).and_then(|cap| {
            let steps = cap.get(2)?.as_str().parse().ok()?;
            match cap.get(1)?.as_str() {
                "U" => Some((Up, steps)),
                "D" => Some((Down, steps)),
                "L" => Some((Left, steps)),
                "R" => Some((Right, steps)),
                _ => None
            }
        })
    }).collect()
}

fn move_node(new: &(i32, i32), tail: &(i32, i32)) -> (i32, i32) {
    if (new.0 - tail.0).abs() <= 1 && (new.1 - tail.1).abs() <= 1 {
        return *tail
    }
    let mut res = (0, 0);
    res.0 = new.0 + match new.0 - tail.0 {
        2 => -1,
        -2 => 1,
        _ => 0,
    };
    res.1 = new.1 + match new.1 - tail.1 {
        2 => -1,
        -2 => 1,
        _ => 0,
    };
    res
}

// could be improved by not creating new Vec every iteration and do it in place,
// but this is fast enough
fn update_propagation(rope: &Vec<(i32, i32)>, new_head: (i32, i32)) -> Vec<(i32, i32)> {
    let mut res = vec![(0,0); 10];
    res[0] = new_head;
    for i in 1..rope.len() {
        res[i] = move_node(&res[i-1], &rope[i]);
    }
    res
}

pub fn solve(input: &str, rope_size: usize) -> u64 {
    let mut rope = vec![(0,0); rope_size];
    let mut all_tail_positions = HashSet::new();
    parse(input).iter().for_each(|(dir, steps)| {
        for _ in 0..*steps {
            let mut new_head = rope[0];
            match dir {
                Up => new_head.1 += 1,
                Down => new_head.1 -= 1,
                Left => new_head.0 -= 1,
                Right => new_head.0 += 1,
            }
            rope = update_propagation(&rope, new_head);
            all_tail_positions.insert(rope[rope_size-1]);
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
    crate::test_day!(9, 88, 36)
}
