use std::collections::VecDeque;

use itertools::enumerate;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MOVE_RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
}

type Stacks = Vec<VecDeque<char>>;

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    n: usize,
}

#[derive(Debug)]
struct Crates {
    stacks: Vec<VecDeque<char>>,
}

fn parse_stack(raw: Vec<&str>) -> Option<Stacks> {
    let (last, others) = raw.split_last()?;
    let size: usize = last.split_whitespace().last()?.parse().ok()?;
    let mut result = vec![VecDeque::new(); size];
    for line in others {
        for (i, c) in enumerate(line.chars().skip(1).step_by(4)) {
            if c != ' ' {
                result[i].push_front(c);
            }
        }
    }
    Some(result)
}

fn parse_moves(raw: Vec<&str>) -> Vec<Move> {
    fn parse_move(line: &str) -> Option<Move> {
        MOVE_RE.captures(line).and_then(|cap| {
            let groups = (cap.get(1), cap.get(2), cap.get(3));
            match groups {
                (Some(n), Some(from), Some(to)) => Some(Move {
                    n: n.as_str().parse().ok()?,
                    from: from.as_str().parse().ok()?,
                    to: to.as_str().parse().ok()?,
                }),
                _ => None
            }
        })
    }
    raw.iter().filter_map(|line| parse_move(line)).collect()
}

fn parse(input: &str) -> (Crates, Vec<Move>) {
    let mut stack: Vec<&str> = vec![];
    let mut moves: Vec<&str> = vec![];
    let mut on_moves = false;
    for line in input.lines() {
        if line.is_empty() {
            on_moves = true;
            continue;
        }
        if !on_moves {
            stack.push(line)
        } else {
            moves.push(line)
        }
    }
    (Crates { stacks: parse_stack(stack).unwrap() }, parse_moves(moves))
}

impl Crates {
    fn move_crate(&mut self, mv: &Move) {
        let Move {n, from, to} = mv;
        for _ in 0..*n {
            let to_move = self.stacks[from - 1].pop_back().unwrap();
            self.stacks[to - 1].push_back(to_move);
        }
    }

    fn move_all_crates(&mut self, mv: &Move) {
        let Move {n, from, to} = mv;
        let mut to_move = VecDeque::new();
        for _ in 0..*n {
            to_move.push_front(self.stacks[from - 1].pop_back().unwrap());
        }
        for cr in to_move {
            self.stacks[to - 1].push_back(cr);
        }
    }

    fn get_message(&self) -> String {
        let mut res: String = "".to_string();
        for s in &self.stacks {
            res.push(*s.iter().last().unwrap());
        }
        res
    }
}

fn solve(input: &str, f: fn(&mut Crates, &Move)) -> String {
    let (mut crates, moves) = parse(input);
    for mv in moves {
        f(&mut crates, &mv);
    }
    crates.get_message()
}

pub fn part1(input: &str) -> String {
    solve(input, Crates::move_crate)
}

pub fn part2(input: &str) -> String {
    solve(input, Crates::move_all_crates)
}

#[test]
fn test() {
    crate::test_2022!(5, "CMZ", "MCD");
}
