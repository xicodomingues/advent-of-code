use crate::utils::load_file;

fn parse(input: String) -> Vec<i64> {
    let mut res = vec![];
    let mut temp = 0;

    for entry in input.lines() {
        if entry.is_empty() {
            res.push(temp);
            temp = 0;
        } else {
            temp += entry.parse::<i64>().unwrap()
        }
    }
    if temp > 0 { res.push(temp) }
    res
}

fn part1(input: String) -> i64 {
    let groups = parse(input);
    groups.iter().max().unwrap().to_owned()
}

fn part2(input: String) -> i64 {
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
    assert_eq!(part1("100\n200\n\n400".to_string()), 400);
    assert_eq!(part1(load_file("test1.txt")), 24000);
    assert_eq!(part2(load_file("test1.txt")), 45000);
}
