use std::cmp::{max, min};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct Range {
    start: u64,
    end: u64, // inclusive
}

impl Range {
    // (before this, in this, after this)
    fn intersect(&self, other: &Range) -> (Option<Range>, Option<Range>, Option<Range>) {
        // before
        let mut before = None;
        let mut inter = None;
        let mut after = None;
        if other.start < self.start {
            before = Some(Range {
                start: other.start,
                end: min(self.start - 1, other.end),
            });
        }
        // in
        if other.start <= self.end && other.end >= self.start {
            inter = Some(Range {
                start: max(self.start, other.start),
                end: min(self.end, other.end),
            });
        }
        // after
        if other.end > self.end {
            after = Some(Range {
                start: max(self.end + 1, other.start),
                end: other.end,
            });
        }
        (before, inter, after)
    }
}

#[test]
fn test_inter() {
    fn cr(start: u64, end: u64) -> Range {
        Range { start, end }
    }
    fn sr(start: u64, end: u64) -> Option<Range> {
        Some(cr(start, end))
    }
    let base = cr(5, 10);
    assert_eq!(base.intersect(&cr(1, 3)), (sr(1, 3), None, None));
    assert_eq!(base.intersect(&cr(1, 4)), (sr(1, 4), None, None));
    assert_eq!(base.intersect(&cr(1, 5)), (sr(1, 4), sr(5, 5), None));
    assert_eq!(base.intersect(&cr(4, 10)), (sr(4, 4), sr(5, 10), None));
    assert_eq!(base.intersect(&cr(10, 11)), (None, sr(10, 10), sr(11, 11)));
    assert_eq!(
        base.intersect(&cr(4, 15)),
        (sr(4, 4), sr(5, 10), sr(11, 15))
    );
    assert_eq!(base.intersect(&cr(10, 15)), (None, sr(10, 10), sr(11, 15)));
    assert_eq!(base.intersect(&cr(20, 25)), (None, None, sr(20, 25)));
}

#[derive(Debug)]
struct MapPart {
    source: u64,
    destination: u64,
    span: u64,
    range: Range,
}

impl MapPart {
    fn parse(line: &str) -> MapPart {
        let (destination, source, range) = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect_tuple()
            .unwrap();
        MapPart {
            source,
            destination,
            span: range,
            range: Range {
                start: source,
                end: source + range - 1,
            },
        }
    }

    fn contains(&self, val: u64) -> bool {
        val >= self.source && val < self.source + self.span
    }

    fn map(&self, val: u64) -> u64 {
        (self.destination as i64 - self.source as i64 + val as i64) as u64
    }

    fn map_range(&self, range: Range) -> Vec<Range> {
        let (before, inter, after) = self.range.intersect(&range);
        let mut res = vec![];
        if let Some(x) = before {
            res.push(x);
        }
        if let Some(x) = after {
            res.push(x);
        }
        res
    }
}

#[derive(Debug)]
struct CompleteMap<'a> {
    info: &'a str,
    parts: Vec<MapPart>,
}

impl<'a> CompleteMap<'a> {
    fn parse(input: &'a str) -> Self {
        let mut lines = input.lines();
        let info = lines.next().unwrap();
        CompleteMap {
            info,
            parts: lines
                .map(MapPart::parse)
                .sorted_by(|a, b| Ord::cmp(&a.source, &b.source))
                .collect(),
        }
    }

    fn map(&self, val: u64) -> u64 {
        self.parts
            .iter()
            .find(|m| m.contains(val))
            .map(|m| m.map(val))
            .unwrap_or(val)
    }
}

#[derive(Debug)]
struct Alamanac<'a> {
    start: Vec<u64>,
    maps: Vec<CompleteMap<'a>>,
}

impl<'a> Alamanac<'a> {
    fn parse(input: &'a str) -> Self {
        let mut blocks = input.split("\n\n");
        let entries = blocks
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect_vec();
        Alamanac {
            start: entries,
            maps: blocks.map(CompleteMap::parse).collect(),
        }
    }
}

fn map_numbers(ns: impl Iterator<Item = u64>, maps: Vec<CompleteMap>) -> u64 {
    let mut min = u64::MAX;
    for seed in ns {
        let mut start = seed;
        for map in &maps {
            start = map.map(start)
        }
        if start < min {
            min = start;
        }
    }
    min
}

pub fn part1(input: &str) -> u64 {
    let almanac = Alamanac::parse(input);
    map_numbers(almanac.start.into_iter(), almanac.maps)
}

pub fn part2(input: &str) -> u64 {
    let almanac = Alamanac::parse(input);
    let ranges = almanac.start.chunks(2).flat_map(|v| v[0]..v[0] + v[1]);
    map_numbers(ranges, almanac.maps)
}

#[test]
fn test() {
    test_2023!(5, 35, 46);
}
