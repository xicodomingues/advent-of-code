#![allow(dead_code)]

extern crate core;

mod utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

macro_rules! run {
    ($day:ident) => {{
        use crate::utils::load_file;
        let tmp = load_file(&format!("{}.txt", stringify!($day)));
        println!("Day {}", stringify!($day).strip_prefix("day").unwrap());
        println!("Part 1: {}", $day::part1(&tmp));
        println!("Part 2: {}", $day::part2(&tmp));
        println!();
    }};
}

fn all() {
    run!(day1);
    run!(day2);
    run!(day3);
    run!(day4);
    run!(day5);
}

fn main() {
    run!(day5);
}
