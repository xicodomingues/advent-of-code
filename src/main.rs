#![allow(dead_code)]

mod utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

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
    run!(day6);
    run!(day7);
    run!(day8);
}

fn main() {
    run!(day8)
}
