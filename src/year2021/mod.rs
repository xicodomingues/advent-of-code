#[macro_export]
macro_rules! test_2021 {
    ($day:literal, $($params:expr),+) => {$crate::test_year_day!(2021, $day, $($params),+)};
}

mod day1;
mod day2;
mod day3;

macro_rules! run {
    ($day:ident) => {$crate::run_year!(2021, $day)};
}

pub fn all() {
    run!(day1);
    run!(day2);
    run!(day3);
}

pub fn run() {
    run!(day3);
}