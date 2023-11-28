use itertools::Itertools;

fn parse(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.lines().map(|x| x.parse().unwrap())
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .tuple_windows::<(_, _)>()
        .filter(|(a, b)| b > a)
        .count()
}

pub fn part2(input: &str) -> usize {
    parse(input)
        .tuple_windows::<(_, _, _)>()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows::<(_, _)>()
        .filter(|(a, b)| b > a)
        .count()
}

#[test]
fn test() {
    test_2021!(1, 7, 5);
}
