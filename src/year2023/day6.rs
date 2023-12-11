use itertools::Itertools;

#[derive(Debug)]
struct Race {
    time: usize,
    dist: usize,
}

impl Race {
    fn count_options(&self) -> usize {
        let mut start = 1;
        // the numbers are simetrical and ordered, so we can do a binary search for the 'inversion point'
        let mut end = self.time / 2;
        loop {
            if end <= start {
                break;
            }
            let middle = (start + end) / 2;
            if middle * (self.time - middle) < self.dist {
                start = middle + 1;
            } else {
                end = middle;
            }
        }
        // check if the start value is actually valid
        if start * (self.time - start) <= self.dist {
            start += 1;
        }
        // get the amount from the inflection point
        self.time - 2 * start + 1
    }
}

#[test]
fn test_race() {
    fn r(time: usize, dist: usize) -> Race {
        Race { time, dist }
    }
    assert_eq!(r(7, 9).count_options(), 4);
    assert_eq!(r(15, 40).count_options(), 8);
    assert_eq!(r(30, 200).count_options(), 9);
}

fn parse_part1(input: &str) -> Vec<Race> {
    let tmp = input
        .lines()
        .map(|line| line.split_whitespace().skip(1).collect_vec())
        .collect_vec();
    let mut res = vec![];
    for i in 0..tmp[0].len() {
        res.push(Race {
            time: tmp[0][i].parse().unwrap(),
            dist: tmp[1][i].parse().unwrap(),
        })
    }
    res
}

fn parse_part2(input: &str) -> Race {
    let tmp = input
        .lines()
        .flat_map(|line| line.split(':').skip(1))
        .map(|x| x.replace(' ', "").parse().unwrap())
        .collect_vec();
    Race {
        time: tmp[0],
        dist: tmp[1],
    }
}

pub fn part1(input: &str) -> usize {
    parse_part1(input).iter().map(Race::count_options).product()
}

pub fn part2(input: &str) -> usize {
    parse_part2(input).count_options()
}

#[test]
fn test() {
    test_2023!(6, 288, 71503);
}
