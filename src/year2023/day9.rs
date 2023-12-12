
fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace().map(|x| x.parse()).map(|x| x.unwrap()).collect()
}

fn solve_line(line: &str, extrapolate: fn(&[Vec<i32>]) -> i32) -> i32 {
    let mut temp = vec![];
    let curr = parse_line(line);
    temp.push(curr);
    while !temp.last().unwrap().iter().all(|x| x == &0) {
        let mut new = vec![];
        let curr = temp.last().unwrap();
        for i in 1..curr.len() {
            new.push(curr[i] - curr[i-1]);
        }
        temp.push(new);
    }
    temp.reverse();
    extrapolate(&temp)
}

pub fn part1(input: &str) -> i32 {
    fn add_last(lines: &[Vec<i32>]) -> i32 {
        lines.iter().skip(1).fold(0, |acc, x| acc + x.last().unwrap())
    }
    input.lines().map(|line| solve_line(line, add_last)).sum()
}

pub fn part2(input: &str) -> i32 {
    fn add_first(lines: &[Vec<i32>]) -> i32 {
        lines.iter().skip(1).fold(0, |acc, x| x[0] - acc)
    }
    input.lines().map(|line| solve_line(line, add_first)).sum()
}

#[test]
fn test() {
    test_2023!(9, 114, 2)
}
