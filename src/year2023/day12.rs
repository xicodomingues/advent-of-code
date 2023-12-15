use core::panic;
use std::{iter::repeat, collections::HashMap};

use itertools::Itertools;

#[derive(Debug)]
struct HotSpring {
    record: Vec<u8>,
    current: String,
}

fn skip_first(input: &str) -> &str {
    let mut new_str = input.chars();
    new_str.next();
    new_str.as_str()
}

impl HotSpring {
    fn parse(line: &str, times: usize) -> Self {
        let (state, rec) = line.trim().split_once(' ').unwrap();

        let record: Vec<u8> = repeat(rec.split(',').map(|x| x.parse().unwrap()))
            .take(times)
            .flatten()
            .collect();
        Self {
            current: format!("{}.", repeat(state).take(times).join("?")),
            record,
        }
    }

    fn count_matches_rec(&self, str_idx: usize, vec_idx: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
        fn can_match(input: &str, mut size: u8) -> bool {
            let mut bytes = input.bytes();
            while size > 0 {
                if bytes.next().unwrap() == b'.' {
                    return false;
                }
                size -= 1;
            }
            bytes.next().unwrap() != b'#'
        }

        let save = |cache: &mut HashMap<(usize, usize), usize>, res| {
            cache.insert((str_idx, vec_idx), res);
            res
        };

        if let Some(res) = cache.get(&(str_idx, vec_idx)) {
            return *res;
        }

        let new_str = &self.current[str_idx..];
        if vec_idx == self.record.len() {
            return save(cache, new_str.bytes().all(|x| x != b'#').into());
        }
        if new_str.is_empty() {
            return save(cache, 0);
        }
        let spring_size = self.record[vec_idx];

        let count_sharp_matches = |cache_| {
            if can_match(new_str, spring_size) {
                self.count_matches_rec(str_idx + spring_size as usize + 1, vec_idx + 1, cache_)
            } else {
                0
            }
        };
        let res = match new_str.bytes().next().unwrap() {
            b'#' => count_sharp_matches(cache),
            b'?' => self.count_matches_rec(str_idx + 1, vec_idx, cache) + count_sharp_matches(cache),
            b'.' => self.count_matches_rec(str_idx + 1, vec_idx, cache),
            _ => panic!("wtf!!"),
        };

        save(cache, res)
    }

    fn count_matches(&self) -> usize {
        self.count_matches_rec(0, 0, &mut HashMap::new())
    }
}

#[test]
fn test_spring() {
    fn h(input: &str, times: usize) -> HotSpring {
        HotSpring::parse(input, times)
    }

    assert_eq!(h("???.### 1,1,3", 1).count_matches(), 1);
    assert_eq!(h(".??. 1", 1).count_matches(), 2);
    assert_eq!(h("?###???????? 3,2,1", 1).count_matches(), 10);

    assert_eq!(h("???.### 1,1,3", 5).count_matches(), 1);
    assert_eq!(h(".??..??...?##. 1,1,3", 5).count_matches(), 16384);
    assert_eq!(h("?###???????? 3,2,1", 5).count_matches(), 506250);
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| HotSpring::parse(line, 1))
        .map(|hs| hs.count_matches())
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| HotSpring::parse(line, 5))
        .map(|hs| hs.count_matches())
        .sum()
}

#[test]
fn test() {
    test_2023!(12, 21, 525152);
}
