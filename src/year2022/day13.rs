use std::cmp::Ordering;
use std::iter::once;
use itertools::Itertools;

#[derive(Debug, Clone)]
enum Entry {
    Single(u8),
    List(Vec<Entry>),
}

fn parse_line_recursive(it: &mut impl Iterator<Item=u8>) -> Entry {
    let mut list = Vec::<Entry>::new();
    loop {
        let mut c = it.next().expect("There always should be a char");
        match c {
            b',' => {}
            b'[' => list.push(parse_line_recursive(it)),
            b']' => return Entry::List(list),
            b'0'..=b'9' => {
                let mut val = 0;
                while c.is_ascii_digit() {
                    val = val * 10 + c - b'0';
                    c = it.next().expect("There should always be a char after a digit");
                }
                list.push(Entry::Single(val));
                if c == b']' {
                    return Entry::List(list);
                }
            }
            _ => panic!("This is an invalid char: '{}'!!", (c as char).escape_debug())
        }
    }
}

impl Eq for Entry {}

impl PartialEq<Self> for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp_elems(self, other)
    }
}

fn cmp_list(left: &[Entry], right: &[Entry]) -> Ordering {
    let mut left_it = left.iter();
    let mut right_it = right.iter();
    loop {
        match (left_it.next(), right_it.next()) {
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => return Ordering::Equal,
            (Some(a), Some(b)) => {
                let res = cmp_elems(a, b);
                if res != Ordering::Equal {
                    return res;
                }
            }
        }
    }
}

fn cmp_elems(left: &Entry, right: &Entry) -> Ordering {
    match left {
        Entry::Single(a) => match right {
            Entry::Single(b) => a.cmp(b),
            Entry::List(v) => cmp_list(&[Entry::Single(*a)], v)
        },
        Entry::List(v) => match right {
            Entry::Single(b) => cmp_list(v, &[Entry::Single(*b)]),
            Entry::List(v2) => cmp_list(v, v2)
        }
    }
}

#[derive(Debug, Clone)]
struct Pair {
    left: Entry,
    right: Entry,
}

impl Pair {
    fn parse(lines: &str) -> Self {
        let mut content = lines.lines().map(|line| {
            let mut it = line.bytes();
            it.next().expect("Problem consuming the first [");
            parse_line_recursive(&mut it)
        }).collect::<Vec<_>>();
        Self {
            right: content.pop().unwrap(),
            left: content.pop().unwrap(),
        }
    }

    fn correct_order(&self) -> bool {
        cmp_elems(&self.left, &self.right) == Ordering::Less
    }
}


fn parse(input: &str) -> impl Iterator<Item=Pair> + '_ {
    input.split("\n\n").map(|pairs| {
        Pair::parse(pairs)
    })
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .enumerate()
        .filter(|(_, pair)| pair.correct_order())
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let divider = Pair::parse("[[2]]\n[[6]]");
    parse(input).chain(once(divider.clone()))
        .flat_map(|p| [p.left, p.right])
        .sorted()
        .enumerate()
        .filter(|(_, entry)| entry == &divider.left || entry == &divider.right )
        .map(|(i, _)| i + 1)
        .product()
}

#[test]
fn test() {
    crate::test_2022!(13, 13, 140)
}
