use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref FORBIDDEN_SET: HashSet<(char, char)> =
        vec![('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')]
            .into_iter()
            .collect();
}

fn is_vowel(c: &char) -> bool {
    c == &'a' || c == &'e' || c == &'i' || c == &'o' || c == &'u'
}

fn has_3_diff_vowels(input: &str) -> bool {
    input.chars().filter(is_vowel).count() >= 3
}

fn has_letter_twice_in_row(input: &str) -> bool {
    input.chars().tuple_windows::<(_, _)>().any(|(a, b)| a == b)
}

fn has_no_forbidden_pairs(input: &str) -> bool {
    !input
        .chars()
        .tuple_windows::<(_, _)>()
        .any(|pair| FORBIDDEN_SET.contains(&pair))
}

fn pair_appears_twice(input: &str) -> bool {
    let pairs = input.chars().tuple_windows::<(_, _)>();
    let mut positions: HashMap<(char, char), Vec<usize>> = HashMap::new();
    for (i, val) in pairs.enumerate() {
        positions
            .entry(val)
            .and_modify(|list| {
                if list.last() != Some(&(i - 1)) {
                    list.push(i)
                }
            })
            .or_insert(vec![i]);
    }
    positions.values().any(|list| list.len() >= 2)
}

fn has_tripplet_letters(input: &str) -> bool {
    let mut tripplets = input.chars().tuple_windows::<(_, _, _)>();
    tripplets.any(|(a, _, b)| a == b)
}

fn solve(input: &str, functions: Vec<fn(&str) -> bool>) -> usize {
    input
        .lines()
        .filter(|line| {
            functions.iter().all(|func| func(line))
        })
        .count()
}

pub fn part1(input: &str) -> usize {
    solve(input, vec![has_3_diff_vowels, has_letter_twice_in_row, has_no_forbidden_pairs])
}

pub fn part2(input: &str) -> usize {
    solve(input, vec![pair_appears_twice, has_tripplet_letters])
}

#[test]
fn test() {
    crate::test_day!(5, 2, 2)
}
