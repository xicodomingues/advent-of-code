use std::str::FromStr;

use itertools::Itertools;

use crate::utils::ParseError;

#[derive(Debug)]
struct Reindeer {
    speed: u32,
    move_t: u32,
    rest_t: u32,
}

impl Reindeer {
    fn current_dist(&self, seconds: u32) -> u32 {
        let complete_moves = (seconds / (self.rest_t + self.move_t)) * self.speed * self.move_t;
        let missing_secs = seconds % (self.rest_t + self.move_t);
        let missing_moves = if missing_secs >= self.move_t {
            self.speed * self.move_t
        } else {
            missing_secs * self.speed
        };
        complete_moves + missing_moves
    }
    
    fn is_moving(&self, seconds: u32) -> bool {
        let to_check = seconds % (self.rest_t + self.move_t);
        to_check > 0 && to_check <= self.move_t
    }
}

impl FromStr for Reindeer {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let seq = line.split_whitespace().collect::<Vec<_>>();
        Ok(Reindeer {
            speed: seq[3].parse()?,
            move_t: seq[6].parse()?,
            rest_t: seq[13].parse()?,
        })
    }
}

#[test]
fn test_reindeer_dist() {
    let r = Reindeer{speed: 14, move_t: 10, rest_t: 127};
    assert_eq!(r.current_dist(1), 14);
    assert_eq!(r.current_dist(10), 140);
    assert_eq!(r.current_dist(127), 140);
    assert_eq!(r.current_dist(137), 140);
    assert_eq!(r.current_dist(138), 154);
    assert_eq!(r.current_dist(1000), 1120);
    
    assert_eq!(r.is_moving(0), false);
    assert_eq!(r.is_moving(1), true);
    assert_eq!(r.is_moving(10), true);
    assert_eq!(r.is_moving(11), false);
    assert_eq!(r.is_moving(137), false);
    assert_eq!(r.is_moving(138), true);
}

fn parse(input: &str) -> Vec<Reindeer> {
    let res: Result<Vec<_>, _> = input.lines().map(Reindeer::from_str).collect();
    res.unwrap()
}

fn max_indexes(v: &[u32]) -> impl Iterator<Item=usize> + '_ {
    let max = v.iter().max().unwrap();
    v.iter().positions(move |x| x == max)
}

pub fn part1(input: &str) -> u32 {
    parse(input).iter().map(|r| r.current_dist(2503)).max().unwrap()
}

pub fn part2(input: &str) -> u32 {
    let reindeers = parse(input);
    let mut points: Vec<u32> = vec![0; reindeers.len()];
    let mut positions: Vec<u32> = vec![0; reindeers.len()];
    
    for tick in 1..=2503 {
        for (i, r) in reindeers.iter().enumerate() {
            if r.is_moving(tick) {
                positions[i] += r.speed;
            }
        }
        for p in max_indexes(&positions) {
            points[p] += 1;
        }
    }
    points.into_iter().max().unwrap()
}

#[test]
fn test() {
    crate::test_2015!(14, 2660, 1564);
}