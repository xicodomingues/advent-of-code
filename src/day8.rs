use std::ops::{Index};
use itertools::Itertools;

#[derive(Debug)]
struct Forest {
    size: usize,
    trees: Vec<Vec<u8>>,
}

fn to_digit(c: char) -> u8 {
    c as u8 - b'0'
}

impl Forest {
    fn from(input: &str) -> Forest {
        Forest {
            size: input.find('\n').unwrap(),
            trees: input.lines().map(|line| line.chars().map(to_digit).collect()).collect(),
        }
    }
}

impl Index<usize> for Forest {
    type Output = Vec<u8>;
    fn index(&self, index: usize) -> &Vec<u8> {
        &self.trees[index]
    }
}

pub fn part1(input: &str) -> u64 {
    let f = Forest::from(input);
    let mut total = 0;
    for r in 0..f.size {
        for c in 0..f.size {
            let height = f[r][c];
            let to_up = (0..r).map(|i| f[i][c]).all(|x| x < height);
            let to_down = (r+1..f.size).map(|i|f[i][c]).all(|x| x < height);
            let to_left = (0..c).map(|i|f[r][i]).all(|x| x < height);
            let to_right = (c+1..f.size).map(|i|f[r][i]).all(|x| x < height);
            if to_left || to_right || to_up || to_down {
                total += 1;
            }
        }
    }
    total
}

fn get_offset(mut iter: impl Iterator<Item=u8>, height: u8) -> Option<usize> {
    iter.find_position(|h| *h >= height).map(|(i, _)|i + 1)
}

pub fn part2(input: &str) -> u64 {
    let f = Forest::from(input);
    let mut max = 0;
    for r in 1..f.size - 1 {
        for c in 1..f.size - 1 {
            let height = f[r][c];
            let to_up = get_offset((0..r).rev().map(|i| f[i][c]), height).unwrap_or(r);
            let to_down = get_offset((r+1..f.size).map(|i|f[i][c]), height).unwrap_or(f.size - 1 - r);
            let to_left = get_offset((0..c).rev().map(|i|f[r][i]), height).unwrap_or(c);
            let to_right = get_offset((c+1..f.size).map(|i|f[r][i]), height).unwrap_or(f.size - 1 - c);
            let new_max = to_left * to_right * to_up * to_down;
            if new_max > max {
                max = new_max
            }
        }
    }
    max as u64
}

#[test]
fn test() {
    crate::test_day!(8, 21, 8)
}
