use std::collections::HashSet;

use crate::utils::Point;

fn navigate(chars: impl Iterator<Item=char>, positions: &mut HashSet<Point>) {
    let mut current_pos = Point::ZERO;
    positions.insert(current_pos.clone());

    for c in chars {
        let new_pos = match c {
            '^' => current_pos.up(),
            'v' => current_pos.down(),
            '>' => current_pos.right(),
            '<' => current_pos.left(),
            _ => panic!("This should not happen"),
        };
        positions.insert(new_pos.clone());
        current_pos = new_pos;
    }
}

pub fn part1(input: &str) -> usize {
    let mut positions = HashSet::<Point>::new();
    navigate(input.chars(), &mut positions);
    positions.len()
}

pub fn part2(input: &str) -> usize {
    let mut positions = HashSet::<Point>::new();
    navigate(input.chars().step_by(2), &mut positions);
    navigate(input.chars().skip(1).step_by(2), &mut positions);
    positions.len()
}

#[test]
fn test() {
    assert_eq!(part1(">"), 2);
    assert_eq!(part1("^>v<"), 4);
    assert_eq!(part1("^v^v^v^v^v"), 2);

    assert_eq!(part2("^v"), 3);
    assert_eq!(part2("^>v<"), 3);
    assert_eq!(part2("^v^v^v^v^v"), 11);
}
