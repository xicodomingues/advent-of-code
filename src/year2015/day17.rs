use itertools::Itertools;

fn parse(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .sorted()
        .collect()
}

fn fill(containers: &[u32], to_fill: u32) -> Vec<Vec<u32>> {
    if to_fill == 0 {
        return vec![vec![]];
    }
    if containers.len() == 0 {
        return vec![];
    }
    let mut res = vec![];
    for (i, c) in containers.iter().enumerate() {
        if to_fill < *c {
            break;
        }
        res.extend(
            fill(&containers[i + 1..], to_fill - c)
                .into_iter()
                .map(|mut v| {
                    v.push(*c);
                    v
                }),
        )
    }
    res
}

pub fn part1(input: &str) -> usize {
    fill(&parse(input), 150).len()
}

pub fn part2(input: &str) -> usize {
    let res = fill(&parse(input), 150);
    let min_len = res.iter().map(|v| v.len()).min().unwrap();
    res.iter().filter(|v| v.len() == min_len).count()
}

#[test]
fn test() {
    crate::test_2015!(17, 4, 3);
}
