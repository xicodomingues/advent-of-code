use std::collections::{BTreeSet, VecDeque};
use std::num::ParseIntError;
use std::str::FromStr;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use snafu::{Backtrace, ErrorCompat, GenerateImplicitData, Snafu};

lazy_static! {
    static ref MONKEY_RE: Regex = Regex::new(r"(?xm)
    Monkey\ (\d+):\n
    \s+Starting\ items:\ ([\d,\ ]+)\n
    \s+Operation:\ new\ =\ old\ ([+*])\ (old|\d+)\n
    \s+Test:\ divisible\ by\ (\d+)\n
    \s+If\ true:\ throw\ to\ monkey\ (\d+)\n
    \s+If\ false:\ throw\ to\ monkey\ (\d+)
    ").unwrap();

    static ref OPERATION_REGEX: Regex = Regex::new(r"new = old ([+*]) (old|\d+)").unwrap();
}

type MResult<T> = Result<T, MonkeyParseError>;

#[derive(Debug)]
struct Monkey {
    id: usize,
    queue: VecDeque<u64>,
    operation: (String, u64),
    test_val: u64,
    targets: (usize, usize),
    inspections: usize,
}

impl Monkey {
    fn _do_op(&self, old: u64) -> u64 {
        match self.operation.0.as_str() {
            "+" => old + self.operation.1,
            "*" => old * self.operation.1,
            "^" => old * old,
            _ => panic!("This should never happen, it should have failed in the parsing")
        }
    }

    fn inspect(&mut self, input: u64, worry_divisor: u64) -> (usize, u64) {
        self.inspections += 1;
        let worry = self._do_op(input) / worry_divisor;
        let target = if worry % self.test_val == 0 {
            self.targets.0
        } else {
            self.targets.1
        };
        (target, worry)
    }
}

#[derive(Debug, Snafu)]
enum MonkeyParseError {
    #[snafu(display("Could not match string to Monkey"))]
    NoMatch { backtrace: Backtrace },
    #[snafu(display("Could not get capture group {}", "group"))]
    NoCaptureGroup { group: usize, backtrace: Backtrace },
    #[snafu(display("Problem parsing int: {}", source))]
    ParseInt { source: ParseIntError, backtrace: Backtrace },
    #[snafu(display("Could not parse the operation: {}", "op"))]
    OperationParsing { backtrace: Backtrace },
}

type MPError = MonkeyParseError;

impl MonkeyParseError {
    fn capture(group: usize) -> Self {
        MonkeyParseError::NoCaptureGroup { group, backtrace: Backtrace::generate() }
    }
    fn no_match() -> Self {
        MonkeyParseError::NoMatch { backtrace: Backtrace::generate() }
    }
    fn operation() -> Self {
        MonkeyParseError::OperationParsing { backtrace: Backtrace::generate() }
    }
}

impl From<ParseIntError> for MonkeyParseError {
    fn from(source: ParseIntError) -> Self {
        MonkeyParseError::ParseInt { source, backtrace: Backtrace::generate() }
    }
}

impl FromStr for Monkey {
    type Err = MonkeyParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        fn get_group<'a>(capture: &'a Captures<'a>, group: usize) -> MResult<&'a str> {
            capture.get(group)
                .ok_or_else(|| MPError::capture(group))
                .map(|g| g.as_str())
        }

        fn get_int<T>(capture: &Captures, group: usize) -> Result<T, MonkeyParseError>
            where MonkeyParseError: From<<T as FromStr>::Err>,
                  T: FromStr {
            Ok(get_group(capture, group)?.parse()?)
        }

        fn parse_queue(queue_str: &str) -> MResult<VecDeque<u64>> {
            queue_str.replace(',', "")
                .split_whitespace()
                .map(|str| str.parse().map_err(MonkeyParseError::from))
                .collect()
        }

        MONKEY_RE.captures(input)
            .ok_or_else(MPError::no_match)
            .and_then(|cap| {
                let op = match (get_group(&cap, 3)?, get_group(&cap, 4)?) {
                    ("*", "old") => Ok(("^".to_string(), 0)),
                    ("*", nbr) => Ok(("*".to_string(), nbr.parse()?)),
                    ("+", nbr) => Ok(("+".to_string(), nbr.parse()?)),
                    _ => Err(MPError::operation())
                };
                Ok(Monkey {
                    id: get_int(&cap, 1)?,
                    queue: parse_queue(get_group(&cap, 2)?)?,
                    operation: op?,
                    test_val: get_int(&cap, 5)?,
                    targets: (get_int(&cap, 6)?, get_int(&cap, 7)?),
                    inspections: 0,
                })
            })
    }
}

fn get_troop(input: &str) -> Vec<Monkey> {
    let parse_res = input.lines().chunks(7).into_iter()
        .map(|mut x| {
            let monkey = x.join("\n");
            Monkey::from_str(&monkey)
        }).collect::<MResult<_>>();
    match parse_res {
        Ok(x) => x,
        Err(e) => {
            if let Some(backtrace) = ErrorCompat::backtrace(&e) {
                println!("{}", backtrace);
            }
            panic!("Problem parsing the monkeys: {}", e);
        }
    }
}

fn solve(input: &str, loops: usize, worry_divisor: u64) -> usize {
    let mut monkeys = get_troop(input);
    let all_monkeys_modulo: u64 = monkeys.iter().map(|m| m.test_val).product();
    for _ in 0..loops {
        for monkey in 0..monkeys.len() {

            while let Some(item) = monkeys[monkey].queue.pop_front() {
                let (target, val) = monkeys[monkey].inspect(item, worry_divisor);
                monkeys[target].queue.push_back(val % all_monkeys_modulo);
            }
        };
    }
    let inspected = monkeys.iter().map(|m| m.inspections).collect::<BTreeSet<_>>();
    inspected.into_iter().rev().take(2).product()
}

pub fn part1(input: &str) -> usize {
    solve(input, 20, 3)
}

pub fn part2(input: &str) -> usize {
    solve(input, 10_000, 1)
}

#[test]
fn test() {
    crate::test_2022!(11, 10605, 2713310158)
}
