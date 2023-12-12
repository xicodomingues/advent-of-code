use std::cmp::Ordering;

use itertools::{enumerate, Itertools};

// A is 12, K is 11, ....., 2 is 0
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Power {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Quad,
    Poker,
}

impl Power {
    fn classify(cards: &[u8]) -> Power {
        let tup = cards
            .iter()
            .sorted()
            .collect_tuple()
            .unwrap();
        match tup {
            (a, b, c, d, e) if (a == b && a == c && a == d && a == e) => Power::Poker,
            (a, b, c, d, _) if (a == b && a == c && a == d) => Power::Quad,
            (_, b, c, d, e) if (b == c && b == d && b == e) => Power::Quad,
            (a, b, c, d, e) if (a == b && a == c && d == e) => Power::FullHouse,
            (a, b, c, d, e) if (a == b && c == d && c == e) => Power::FullHouse,
            (a, b, c, _, _) if (a == b && a == c) => Power::Three,
            (_, b, c, d, _) if (b == c && b == d) => Power::Three,
            (_, _, c, d, e) if (c == d && c == e) => Power::Three,
            (a, b, c, d, _) if (a == b && c == d) => Power::TwoPair,
            (a, b, _, d, e) if (a == b && d == e) => Power::TwoPair,
            (_, b, c, d, e) if (b == c && d == e) => Power::TwoPair,
            (a, b, _, _, _) if (a == b) => Power::OnePair,
            (_, b, c, _, _) if (b == c) => Power::OnePair,
            (_, _, c, d, _) if (c == d) => Power::OnePair,
            (_, _, _, d, e) if (d == e) => Power::OnePair,
            (_, _, _, _, _) => Power::HighCard,
        }
    }
}

#[test]
fn test_power() {
    assert_eq!(Power::classify(&vec![1, 1, 1, 1, 1]), Power::Poker);
    assert_eq!(Power::classify(&vec![12, 12, 10, 9, 9]), Power::TwoPair);
    assert!(Power::Poker > Power::Quad);
    assert!(Power::HighCard == Power::HighCard);
}

#[derive(Debug, Eq)]
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
        let cards = split_iter.next().unwrap().chars().map(get_val).collect_vec();
        let rank = Power::classify(&cards);
        Self {
            cards,
            rank,
            bid: split_iter.next().unwrap().parse().unwrap(),
        }
    }

    fn parse_part2(line: &str) -> Self {
        fn get_rank(cards: &[u8]) -> Power {
            let power = Power::classify(cards);
            let jokers = cards.iter().filter(|x| x < &&10).count();
            match jokers {
                5 => Power::Poker,
                4 => Power::Poker,
                3 => match power {
                    Power::HighCard => Power::Quad,
                    Power::OnePair => Power::Poker,
                    _ => panic!("This should not happen"),
                },
                2 => match power {
                    Power::HighCard => Power::Three,
                    Power::OnePair => Power::Quad,
                    Power::Three => Power::Poker,
                    _ => panic!("This should not happen v2"),
                },
                1 => match power {
                    Power::HighCard => Power::OnePair,
                    Power::OnePair => Power::Three,
                    Power::TwoPair => Power::FullHouse,
                    Power::Three => Power::Quad,
                    Power::Quad => Power::Poker,
                    _ => panic!("This should not happen v3"),
                },
                0 => power,
                _ => panic!("WTF!!!"),
            }
        }

        let mut split_iter = line.split_whitespace();
        let cards = split_iter.next().unwrap().chars();
        let mut joker = 9;
        let mut cards_value = vec![];
        for c in cards {
            cards_value.push(match c {
                'A' => 22,
                'K' => 21,
                'Q' => 20,
                'T' => 18,
                '9' => 17,
                '8' => 16,
                '7' => 15,
                '6' => 14,
                '5' => 13,
                '4' => 12,
                '3' => 11,
                '2' => 10,
                'J' => {
                    joker -= 1;
                    joker
                }
                _ => panic!("WTF!!!! '{}'", c),
            });
        }
        let rank = get_rank(&cards_value);
        Self {
            cards: cards_value,
            rank,
            bid: split_iter.next().unwrap().parse().unwrap(),
        }
    }
}

#[test]
fn test_parse() {
    assert_eq!(Hand::parse("AAAAA 1").rank, Power::Poker);
    assert_eq!(Hand::parse("AATAA 1").rank, Power::Quad);
    assert_eq!(Hand::parse("77A77 1").rank, Power::Quad);
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}


impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rank.cmp(&other.rank) {
            Ordering::Equal => {}
            ord => return ord,
        }
        self.cards.cmp(&other.cards)
    }
}

fn parse(input: &str) -> Vec<Hand> {
    input.lines().map(Hand::parse).sorted().collect()
}

fn parse_part2(input: &str) -> Vec<Hand> {
    input.lines().map(Hand::parse_part2).sorted().collect()
}

pub fn part1(input: &str) -> usize {
    enumerate(parse(input))
        .map(|(i, x)| (i + 1) * x.bid as usize)
        .sum()
}

pub fn part2(input: &str) -> usize {
    enumerate(parse_part2(input))
        .map(|(i, x)| (i + 1) * x.bid as usize)
        .sum()
}

#[test]
fn test() {
    test_2023!(7, 6440, 5905);
}
