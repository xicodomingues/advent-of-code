use std::collections::HashSet;

use crate::utils::Rotation::*;
use crate::utils::{Direction, Point, Rotation};

struct Turn {
    r: Rotation,
    n: u8,
}

fn parse(input: &str) -> impl Iterator<Item = Turn> + '_ {
    input.split(',').map(|it| {
        let d = it.trim().bytes().next().unwrap();
        let n = it.trim()[1..].parse().unwrap();
        match d {
            b'L' => Turn {
                r: AntiClockwise,
                n,
            },
            b'R' => Turn { r: Clockwise, n },
            _ => panic!("unknown turn"),
        }
    })
}

pub fn part1(input: &str) -> isize {
    let mut pos = Point::ZERO;
    let mut dir = Direction::Up;
    for Turn { r, n } in parse(input) {
        dir = dir.rotate(r);
        pos = pos.move_in(&dir, n as isize);
    }
    pos.manhathan_dist(&Point::ZERO)
}

pub fn part2(input: &str) -> isize {
    let mut pos = Point::ZERO;
    let mut dir = Direction::Up;
    let mut visited = HashSet::<Point>::new();
    visited.insert(pos);
    'outer: for Turn { r, n } in parse(input) {
        dir = dir.rotate(r);
        for _ in 0..n {
            pos = pos.move1(&dir);
            if visited.contains(&pos) {
                break 'outer;
            }
            visited.insert(pos);
        }
    }
    pos.manhathan_dist(&Point::ZERO)
}

#[test]
fn test() {
    assert_eq!(part1("R2, L3"), 5);
    assert_eq!(part1("R2, R2, R2"), 2);
    assert_eq!(part1("R5, L5, R5, R3"), 12);

    assert_eq!(part2("R8, R4, R4, R8"), 4);
}
