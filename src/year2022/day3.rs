use std::collections::HashSet;

use itertools::Itertools;

fn to_set(data: &str) -> HashSet<char> {
    HashSet::from_iter(data.chars())
}

fn score(c: char) -> i64 {
    match c {
        'a'..='z' => c as i64 - 'a' as i64 + 1,
        'A'..='Z' => c as i64 - 'A' as i64 + 27,
        _ => 0
    }
}

pub fn part1(input: &str) -> i64 {
    input.lines().map(|line| {
        let size = line.len() / 2;
        score(*to_set(&line[..size])
            .intersection(&to_set(&line[size..]))
            .into_iter().next().unwrap())
    }).sum()
}

fn get_badge_score(elves: Vec<&str>) -> i64 {
    assert_eq!(elves.len(), 3);
    let intersection = &(&to_set(elves[0]) & &to_set(elves[1])) & &to_set(elves[2]);
    score(intersection.into_iter().next().unwrap())
}

pub fn part2(input: &str) -> i64 {
    input.lines().chunks(3).into_iter().map(|group| {
        get_badge_score(group.collect())
    }).sum()
}

#[test]
fn test() {
    crate::test_2022!(3, 157, 70);
}
