use std::cmp::{max, min};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
struct Range {
    start: u64,
    end: u64, // inclusive
}

impl Range {
    // (before this, in this, after this)
    fn intersect(&self, other: &Range) -> Option<Range> {
        // in
        if other.start <= self.end && other.end >= self.start {
            return Some(Range {
                start: max(self.start, other.start),
                end: min(self.end, other.end),
            });
        }
        None
    }
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

    fn id_map(start: u64, end: u64) -> Self {
        assert!(start <= end, "<{}, {}>", start, end);
        MapPart {
            source: start,
            destination: start,
            span: end - start + 1,
            range: Range { start, end },
        }
    }

    fn contains(&self, val: u64) -> bool {
        val >= self.source && val < self.source + self.span
    }

    fn map(&self, val: u64) -> u64 {
        (self.destination as i64 - self.source as i64 + val as i64) as u64
    }

    fn map_range(&self, other: &Range) -> Option<Range> {
        self.range.intersect(other).map(|range| Range {
            start: self.map(range.start),
            end: self.map(range.end),
        })
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
        let temp_parts = lines
            .map(MapPart::parse)
            .sorted_by(|a, b| Ord::cmp(&a.source, &b.source))
            .collect_vec();
        let mut parts = vec![];
        let mut start = 0;
        let mut end = 0;
        for part in temp_parts {
            let MapPart { source, span, .. } = part;
            if source > 0 {
                end = source - 1;
            }
            if source > 0 && start <= end {
                parts.push(MapPart::id_map(start, end));
            }
            parts.push(part);
            start = source + span;
        }
        if start - 1 != u32::MAX as u64 {
            parts.push(MapPart::id_map(start, u32::MAX as u64));
        }
        //my_dbg!(parts.iter().map(|x| x.range.clone()).collect_vec());
        CompleteMap { info, parts }
    }

    fn map(&self, val: u64) -> u64 {
        self.parts
            .iter()
            .find(|m| m.contains(val))
            .map(|m| m.map(val))
            .unwrap_or(val)
    }

    // can be improved with a binary search instead of going over all the list
    fn map_range(&self, range: &Range) -> Vec<Range> {
        self.parts
            .iter()
            .filter_map(|part| part.map_range(range))
            .sorted()
            .collect()
    }

    fn map_ranges(&self, ranges: &Vec<Range>) -> Vec<Range> {
        let mut res = vec![];
        for range in ranges {
            res.extend(self.map_range(range))
        }
        res.sort();
        res
    }
}

#[test]
fn test_inter() {
    
    use indoc::indoc;
    
    fn cm(src: u64, dest: u64, span: u64) -> MapPart {
        MapPart {
            source: src,
            destination: dest,
            span,
            range: cr(src, src + span - 1),
        }
    }
    fn cr(start: u64, end: u64) -> Range {
        Range { start, end }
    }
    fn sr(start: u64, end: u64) -> Option<Range> {
        Some(cr(start, end))
    }
    let base = cr(5, 10);
    assert_eq!(base.intersect(&cr(1, 3)), None);
    assert_eq!(base.intersect(&cr(1, 4)), None);
    assert_eq!(base.intersect(&cr(1, 5)), sr(5, 5));
    assert_eq!(base.intersect(&cr(4, 10)), sr(5, 10));
    assert_eq!(base.intersect(&cr(10, 11)), sr(10, 10));
    assert_eq!(base.intersect(&cr(4, 15)), sr(5, 10));
    assert_eq!(base.intersect(&cr(10, 15)), sr(10, 10));
    assert_eq!(base.intersect(&cr(20, 25)), None);

    let map_part = cm(10, 20, 15);
    assert_eq!(map_part.map_range(&cr(0, 9)), None);
    assert_eq!(map_part.map_range(&cr(0, 10)), sr(20, 20));
    assert_eq!(map_part.map_range(&cr(0, 20)), sr(20, 30));
    assert_eq!(map_part.map_range(&cr(0, 30)), sr(20, 34));
    assert_eq!(map_part.map_range(&cr(12, 20)), sr(22, 30));
    assert_eq!(map_part.map_range(&cr(25, 40)), None);

    /*
    seed  soil
    0     0
    1     1
    ...   ...
    48    48
    49    49
    50    52
    51    53
    ...   ...
    96    98
    97    99
    98    50
    99    51
     */
    // dest src range
    let full_map = CompleteMap::parse(indoc! {"
        seed-to-soil map:
        50 98 2
        52 50 48
    "});
    assert_eq!(full_map.map_range(&cr(0, 30)), vec![cr(0, 30)]);
    assert_eq!(full_map.map_range(&cr(0, 60)), vec![cr(0, 49), cr(52, 62)]);
    assert_eq!(full_map.map_range(&cr(0, 97)), vec![cr(0, 49), cr(52, 99)]);
    assert_eq!(
        full_map.map_range(&cr(0, 99)),
        vec![cr(0, 49), cr(50, 51), cr(52, 99),]
    );
    assert_eq!(
        full_map.map_range(&cr(0, 200)),
        vec![cr(0, 49), cr(50, 51), cr(52, 99), cr(100, 200)]
    );

    // map ranges
    assert_eq!(full_map.map_ranges(&vec![cr(0, 30)]), vec![cr(0, 30)]);
    assert_eq!(
        full_map.map_ranges(&vec![cr(0, 14), cr(15, 30)]),
        vec![cr(0, 14), cr(15, 30)]
    );
    assert_eq!(
        full_map.map_ranges(&vec![cr(0, 30), cr(31, 60)]),
        vec![cr(0, 30), cr(31, 49), cr(52, 62)]
    );
    assert_eq!(
        full_map.map_ranges(&vec![cr(0, 49), cr(50, 97), cr(98, 99)]),
        vec![cr(0, 49), cr(50, 51), cr(52, 99)]
    );
    assert_eq!(
        full_map.map_ranges(&vec![cr(60, 98)]),
        vec![cr(50, 50), cr(62, 99)]
    );
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
    let mut ranges = almanac.start.chunks(2).map(|v| Range {start: v[0], end: v[0] + v[1]}).collect_vec();
    for map in almanac.maps {
        ranges = map.map_ranges(&ranges);
    }
    ranges[0].start
}

#[test]
fn test() {
    test_2023!(5, 35, 46);
}
