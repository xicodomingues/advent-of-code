#![allow(dead_code)]

mod utils;
mod day1;
mod day2;

fn main() {
    day2::run();
}

#[test]
fn test() {
    print!("{:?}", "100 200  300".split(' ').collect::<Vec<&str>>());
}

