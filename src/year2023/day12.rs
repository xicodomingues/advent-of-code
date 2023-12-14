use regex::Regex;

struct HotSpring {
    record: Vec<u8>,
    current: String,
}

impl HotSpring {
    fn parse(line: &str) -> Self {
        let (state, rec) = line.trim().split_once(' ').unwrap();
        let record: Vec<u8> = rec.split(',').map(|x| x.parse().unwrap()).collect();
        Self {
            current: state.to_owned(),
            record,
        }
    }

    // add memo later
    fn count_matches_rec(&self, input: &str, target: Vec<u8>, mut total_pounds: isize) -> usize {
        let mut x = input.trim_start_matches('.');
        let mut cpy = target.clone();
        let first = x.bytes().next().unwrap();
        // check case where first on target is zero
        match first {
            b'#' => {
                total_pounds -= 1;
                cpy[0] -= 1;
            }
        }
        0
    }

    fn count_matches(&self) -> usize {
        0
    }
}

#[test]
fn test_spring() {
    fn h(input: &str) -> HotSpring {
        HotSpring::parse(input)
    }

    let hs = h(".#......###.#..######.. 1,3,1,6");

    assert_eq!(h("???.### 1,1,3").count_matches(), 1);
    assert_eq!(h("?###???????? 3,2,1").count_matches(), 10);
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(HotSpring::parse)
        .map(|hs| hs.count_matches())
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(HotSpring::parse)
        .map(|hs| hs.count_matches())
        .sum()
}

#[test]
fn test() {
    test_2023!(12, 21);
}
