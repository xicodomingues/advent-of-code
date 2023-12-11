use std::cmp::Ordering;

use itertools::{enumerate, Itertools};

use crate::my_dbg;

// A is 12, K is 11, ....., 2 is 0
#[derive(Debug, Eq, Ord)]
enum Power {
    HighCard(u8, u8, u8, u8, u8),
    OnePair(u8, u8, u8, u8),
    TwoPair(u8, u8, u8),
    Three(u8, u8, u8),
    FullHouse(u8, u8),
    Quad(u8, u8),
    Poker(u8),
}

impl Power {
    fn classify(cards: &Vec<u8>) -> Power {
        let tup = cards.into_iter().sorted().map(|x| *x).collect_tuple().unwrap();
        match tup {
            (a, b, c, d, e) if (a == b && a == c && a == d && a == e) => Power::Poker(a),
            (a, b, c, d, e) if (a == b && a == c && a == d) => Power::Quad(a, e),
            (a, b, c, d, e) if (b == c && b == d && b == e) => Power::Quad(b, a),
            (a, b, c, d, e) if (a == b && a == c && d == e) => Power::FullHouse(a, d),
            (a, b, c, d, e) if (a == b && c == d && c == e) => Power::FullHouse(c, a),
            (a, b, c, d, e) if (a == b && a == c) => Power::Three(a, d, e),
            (a, b, c, d, e) if (b == c && b == d) => Power::Three(b, a, e),
            (a, b, c, d, e) if (c == d && c == e) => Power::Three(c, a, b),
            (a, b, c, d, e) if (a == b && c == d) => Power::TwoPair(a, c, e),
            (a, b, c, d, e) if (a == b && d == e) => Power::TwoPair(a, d, c),
            (a, b, c, d, e) if (b == c && d == e) => Power::TwoPair(b, d, a),
            (a, b, c, d, e) if (a == b) => Power::OnePair(a, c, d, e),
            (a, b, c, d, e) if (b == c) => Power::OnePair(b, a, d, e),
            (a, b, c, d, e) if (c == d) => Power::OnePair(c, a, b, e),
            (a, b, c, d, e) if (d == e) => Power::OnePair(d, a, b, c),
            (a, b, c, d, e) => Power::HighCard(a, b, c, d, e),
        }
    }
}

impl PartialEq for Power {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::HighCard(_, _, _, _, _), Self::HighCard(_, _, _, _, _)) => true,
            (Self::OnePair(_, _, _, _), Self::OnePair(_, _, _, _)) => true,
            (Self::TwoPair(_, _, _), Self::TwoPair(_, _, _)) => true,
            (Self::Three(_, _, _), Self::Three(_, _, _)) => true,
            (Self::FullHouse(_, _), Self::FullHouse(_, _)) => true,
            (Self::Quad(_, _), Self::Quad(_, _)) => true,
            (Self::Poker(_), Self::Poker(_)) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Power {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        fn rank(p: &Power) -> u8 {
            match p {
                Power::HighCard(_, _, _, _, _) => 0,
                Power::OnePair(_, _, _, _) => 1,
                Power::TwoPair(_, _, _) => 2,
                Power::Three(_, _, _) => 3,
                Power::FullHouse(_, _) => 4,
                Power::Quad(_, _) => 5,
                Power::Poker(_) => 6,
            }
        }
        Some(Ord::cmp(&rank(self), &rank(other)))
    }
}

#[test]
fn test_power() {
    assert_eq!(Power::classify(&vec![1, 1, 1, 1, 1]), Power::Poker(1));
    assert_eq!(
        Power::classify(&vec![12, 12, 10, 9, 9]),
        Power::TwoPair(12, 9, 10)
    );
    assert!(Power::Poker(1) > Power::Quad(12, 11));
    assert!(Power::Poker(10) == Power::Poker(9));
    assert!(Power::HighCard(12, 11, 10, 8, 7) == Power::HighCard(12, 11, 10, 9, 7));
}

#[derive(Debug, Eq, Ord)]
struct Hand {
    cards: Vec<u8>,
    rank: Power,
    bid: u32,
}

impl Hand {
    fn parse(line: &str) -> Self {
        fn get_val(c: char) -> u8 {
            match c {
                'A' => 12,
                'K' => 11,
                'Q' => 10,
                'J' => 9,
                'T' => 8,
                '9' => 7,
                '8' => 6,
                '7' => 5,
                '6' => 4,
                '5' => 3,
                '4' => 2,
                '3' => 1,
                '2' => 0,
                _ => panic!("WTF!!!! '{}'", c),
            }
        }

        let mut split_iter = line.split_whitespace();
        let cards = split_iter
            .next()
            .unwrap()
            .chars()
            .map(get_val)
            .collect();
        let rank = Power::classify(&cards);
        Self {
            cards,
            rank: rank,
            bid: split_iter.next().unwrap().parse().unwrap(),
        }
    }
}

#[test]
fn test_parse() {
    assert_eq!(Hand::parse("AAAAA 1").rank, Power::Poker(12));
    assert_eq!(Hand::parse("AATAA 1").rank, Power::Quad(12, 8));
    assert_eq!(Hand::parse("77A77 1").rank, Power::Quad(5, 12));
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.rank.partial_cmp(&other.rank) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.cards.partial_cmp(&other.cards) {
            Some(core::cmp::Ordering::Equal) => Some(Ordering::Equal),
            ord => ord,
        }
    }
}

fn parse(input: &str) -> Vec<Hand> {
    input.lines().map(Hand::parse).sorted().collect()
}

fn get_card_value(card: &u8) -> char{
    match card {
        0..=7 => (card + 2 + b'0') as char,
        8 => 'T',
        9 => 'J',
        10 => 'Q',
        11 => 'K',
        12 => 'A',
        _ => panic!("WTF!!!"),
    }
}

fn print_cards(cards: Vec<u8>) {
    println!("{}", cards.iter().map(get_card_value).join(""));
}

pub fn part1(input: &str) -> usize {
    parse(input).iter().map(|x| x.cards.clone()).for_each(print_cards);
    enumerate(parse(input))
        .map(|(i, x)| (i + 1) * x.bid as usize)
        .sum()
}

pub fn part2(input: &str) -> usize {
    0
}

#[test]
fn test() {
    test_2023!(7, 6440);
}
