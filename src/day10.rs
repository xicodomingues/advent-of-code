use itertools::enumerate;

use crate::day10::Operation::*;

#[derive(Debug)]
enum Operation {
    Noop,
    AddX(i64),
}

impl Operation {
    fn cycles(self) -> usize {
        match self {
            Noop => 1,
            AddX(_) => 2,
        }
    }
}

fn parse(input: &str) -> impl Iterator<Item=Operation> + '_ {
    input.lines().filter_map(|line| {
        match line {
            x if x.starts_with("noop") => Some(Noop),
            x if x.starts_with("addx") => Some(AddX(x.strip_prefix("addx ")?.parse().unwrap())),
            _ => None
        }
    })
}

fn clock_vals(input: &str) -> Vec<i64> {
    let mut clocked_x = Vec::with_capacity(260);
    let mut x: i64 = 1;
    clocked_x.push(1);
    for op in parse(input) {
        match op {
            Noop => clocked_x.push(x),
            AddX(to_add) => {
                clocked_x.push(x);
                clocked_x.push(x + to_add);
                x += to_add;
            }
        }
    };
    clocked_x.pop();
    clocked_x
}

pub fn part1(input: &str) -> i64 {
    let stop_at: [i64; 6] = [20, 60, 100, 140, 180, 220];
    let clocks = clock_vals(input);
    stop_at.into_iter().map(|c| c * clocks[(c - 1) as usize]).sum()
}

pub fn part2(input: &str) -> String {
    let mut res = String::with_capacity(260);
    for (i, x) in enumerate(clock_vals(input)) {
        if i % 40 == 0 { res.push('\n') }
        if (x - 1..=x + 1).contains(&((i % 40) as i64)) {
            res.push('#');
        } else {
            res.push(' ');
        }
    }
    res
}

#[test]
fn test() {
    crate::test_day!(10, 13140, ("
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....").replace('.', " "))
}
