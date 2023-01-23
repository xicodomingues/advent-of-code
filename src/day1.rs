use crate::utils::load_file;

fn parse(input: Vec<String>) -> Vec<i64> {
    let mut res = vec![];
    let mut temp = 0;

    for entry in input {
        if entry.is_empty() {
            res.push(temp);
            temp = 0;
        } else {
            temp += entry.parse::<i64>().unwrap()
        }
    }
    res
}

fn part1(input: Vec<String>) -> i64 {
    let groups = parse(input);
    groups.iter().max().unwrap().to_owned()
}

fn part2(input: Vec<String>) -> i64 {
    let mut groups = parse(input);
    groups.sort();
    groups.reverse();
    groups.iter().take(3).sum()
}

pub fn run() {
    println!("{}", part1(load_file("day1.txt")));
    println!("{}", part2(load_file("day1.txt")));
}

#[test]
fn test() {
    assert_eq!(part1("100 200  400".split(' ').map(|s| s.to_string()).collect()), 400);
    assert_eq!(part1(load_file("test1.txt")), 24000)
}

