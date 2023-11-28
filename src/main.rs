#![allow(dead_code)]

extern crate core;

mod utils;
mod year2015;
mod year2016;
mod year2021;
mod year2022;

fn main() {
    year2016::run();
}
