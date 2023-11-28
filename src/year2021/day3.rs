use itertools::enumerate;

#[derive(Clone, Debug)]
struct Counter {
    zeros: u32,
    ones: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { zeros: 0, ones: 0 }
    }

    fn add(&mut self, val: &u8) {
        match val {
            0 => self.zeros += 1,
            1 => self.ones += 1,
            _ => panic!("Unexpected char"),
        }
    }

    fn max(&self) -> u8 {
        if self.zeros > self.ones {
            0
        } else {
            1
        }
    }

    fn min(&self) -> u8 {
        if self.zeros <= self.ones {
            0
        } else {
            1
        }
    }
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn to_number(v: impl Iterator<Item = u8>) -> u32 {
    v.fold(0, |acc, c| acc * 2 + c as u32)
}

pub fn part1(input: &str) -> u32 {
    let entries = parse(input);
    let mut counters = vec![Counter::new(); entries[0].len()];
    entries.iter().for_each(|vals| {
        for (i, x) in enumerate(vals) {
            counters[i].add(x)
        }
    });
    let gamma = to_number(counters.iter().map(|c| c.max()));
    let epsilon = to_number(counters.iter().map(|c| c.min()));
    gamma * epsilon
}

fn get_oxigen(entries: Vec<Vec<u8>>, i: usize, f: fn(&Counter) -> u8) -> Vec<u8> {
    let mut counter = Counter::new();
    entries.iter().for_each(|x| counter.add(&x[i]));
    let valid_entries: Vec<_> = entries
        .iter()
        .filter(|x| x[i] == f(&counter))
        .map(|x| x.clone())
        .collect();
    if valid_entries.len() <= 1 {
        return valid_entries[0].clone();
    }
    return get_oxigen(valid_entries, i + 1, f);
}

pub fn part2(input: &str) -> u32 {
    let entries = parse(input);
    let oxigen = to_number(get_oxigen(entries, 0, Counter::max).into_iter());
    let entries = parse(input);
    let co2 = to_number(get_oxigen(entries, 0, Counter::min).into_iter());
    co2 * oxigen
}

#[test]
fn test() {
    test_2021!(3, 198, 230);
}
