fn parse(input: &str) -> Vec<i64> {
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

pub fn part1(input: &str) -> i64 {
    let groups = parse(input);
    groups.iter().max().unwrap().to_owned()
}

pub fn part2(input: &str) -> i64 {
    let mut groups = parse(input);
    groups.sort();
    groups.reverse();
    groups.iter().take(3).sum()
}

#[test]
fn test() {
    assert_eq!(part1("100\n200\n\n400"), 400);
    crate::test_2022!(1, 24000, 45000)
}
