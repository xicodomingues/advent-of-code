
#[macro_export]
macro_rules! test_2015 {
    ($day:literal, $($params:expr),+) => {$crate::test_year_day!(2015, $day, $($params),+)};
}

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

macro_rules! run {
    ($day:ident) => {$crate::run_year!(2015, $day)};
}

fn all() {
    run!(day1);
    run!(day2);
    run!(day3);
    run!(day4);
    run!(day5);
    run!(day6);
}

pub fn run() {
    run!(day6);
}