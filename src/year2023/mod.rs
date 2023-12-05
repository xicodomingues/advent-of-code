#[macro_export]
macro_rules! test_2023 {
    ($day:literal, $($params:expr),+) => {$crate::test_year_day!(2023, $day, $($params),+)};
}

mod day1;
mod day2;
mod day3;
mod day4;

macro_rules! run {
    ($day:ident) => {$crate::run_year!(2023, $day)};
}

pub fn all() {
    run!(day1);
    run!(day2);
    run!(day3);
    run!(day4);
}

pub fn run() {
    run!(day4);
}
