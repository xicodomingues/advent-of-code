mod utils;
mod day1;

fn main() {
    day1::run();
}

#[test]
fn test() {
    print!("{:?}", "100 200  300".split(' ').collect::<Vec<&str>>());
}

