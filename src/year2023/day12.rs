use regex::Regex;

struct HotSpring {
    record: Vec<u8>,
    current: String,
    correct_match: Regex,
    maybe_match: Regex,
}

impl HotSpring {
    fn parse(line: &str) -> Self {
        fn exact_match(input: &[u8]) -> Regex {
            let mut regex = r"^\.*".to_string();
            for x in input {
                regex.push_str(&format!("#{{{x}}}\\.+"))
            }
            regex.pop();
            regex.push_str("*$");
            Regex::new(&regex).unwrap()
        }

        fn maybe_match(input: &[u8]) -> Regex {
            let mut regex = r"^[\.?]*".to_string();
            for x in input {
                regex.push_str(&format!("[#?]{{{x}}}[\\.?]+"))
            }
            regex.pop();
            regex.push_str("*$");
            Regex::new(&regex).unwrap()
        }

        let (state, rec) = line.trim().split_once(' ').unwrap();
        let record: Vec<u8> = rec.split(',').map(|x| x.parse().unwrap()).collect();
        Self {
            current: state.to_owned(),
            correct_match: exact_match(&record),
            maybe_match: maybe_match(&record),
            record,
        }
    }

    fn is_match(&self, test: &str) -> bool {
        self.correct_match.is_match(test)
    }

    fn can_match(&self, test: &str) -> bool {
        self.maybe_match.is_match(test)
    }
        
    fn count_matches_rec(&self, input: &mut String, index: usize) -> usize {
        assert!(input.bytes().take(index).all(|x| x != b'?'));
        if let Some(next) = input.bytes().skip(index).position(|x| x == b'?') {
            let idx = next + index;
            let s_bytes: &mut [u8] = unsafe { input.as_bytes_mut() };
            s_bytes[idx] = b'.';
            let temp = if self.can_match(&input) {
                self.count_matches_rec(input, idx + 1)
            } else {
                0
            };
            let s_bytes: &mut [u8] = unsafe { input.as_bytes_mut() };
            s_bytes[idx] = b'#';
            temp + if self.can_match(&input) {
                self.count_matches_rec(input, idx + 1)
            } else {
                0
            }
        } else {
            self.is_match(&input).into()
        }
    }

    fn count_matches(&self) -> usize {
        self.count_matches_rec(&mut self.current.to_owned(), 0)
    }
}

#[test]
fn test_spring() {
    fn h(input: &str) -> HotSpring {
        HotSpring::parse(input)
    }

    let hs = h(".#......###.#..######.. 1,3,1,6");

    assert!(hs.is_match(&".#......###.#..######.."));
    assert!(hs.is_match(&"#.###.#.######"));
    assert!(!hs.is_match(&"#..##.#.######"));

    assert!(hs.can_match(&".#......###.#..######.."));
    assert!(hs.can_match(&"#.###.#.######"));
    assert!(!hs.can_match(&"#..##.#.######"));

    assert!(h("???.### 1,1,3").can_match("???.###"));
    assert!(h(".??..??...?##. 1,1,3").can_match(".??..??...?##."));
    assert!(h("?#?#?#?#?#?#?#? 1,3,1,6").can_match("?#?#?#?#?#?#?#?"));
    assert!(h("????.#...#... 4,1,1").can_match("????.#...#..."));
    assert!(h("????.######..#####. 1,6,5").can_match("????.######..#####."));
    assert!(h("?###???????? 3,2,1").can_match("?###????????"));

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

pub fn part2(_input: &str) -> usize {
    0
}

#[test]
fn test() {
    test_2023!(12, 21);
}
