use std::collections::VecDeque;

use itertools::iproduct;

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    content: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

fn parse(input: &str) -> Map {
    let mut res: Vec<Vec<u8>> = vec![];
    let mut start = None;
    let mut end = None;
    for (r, line) in input.lines().enumerate() {
        res.push(line.chars().enumerate().map(|(c, char)| {
            match char {
                'S' => {
                    start = Some((r, c));
                    1
                }
                'E' => {
                    end = Some((r, c));
                    26
                }
                'a'..='z' => char as u8 - b'a' + 1,
                _ => panic!("This character should not be here")
            }
        }).collect());
    }
    Map {
        width: res[0].len(),
        height: res.len(),
        content: res,
        start: start.expect("There was no start on the provided map"),
        end: end.expect("There was no end on the provided map"),
    }
}

fn get_neighbours(coord: (usize, usize), map: &Map) -> impl Iterator<Item=(usize, usize)> + '_ {
    let (r, c) = (coord.0 as i32, coord.1 as i32);
    let res = vec![
        (r, c - 1),
        (r, c + 1),
        (r + 1, c),
        (r - 1, c),
    ];
    res.into_iter()
        .filter(|(r, c)| *r >= 0 && *c >= 0 && *r < map.height as i32 && *c < map.width as i32)
        .map(|(r, c)| (r as usize, c as usize))
}

fn can_move(map: &Map, current: (usize, usize), next: (usize, usize)) -> bool {
    let from = map.content[current.0][current.1];
    let to = map.content[next.0][next.1];
    from <= to + 1
}

fn get_cost_map(map: &Map) -> Vec<Vec<u16>> {
    let mut cost: Vec<Vec<u16>> = vec![vec![u16::MAX; map.width]; map.height];
    let mut to_analyze = VecDeque::new();
    to_analyze.push_back((map.end, 0));
    cost[map.end.0][map.end.1] = 0;

    while let Some((next, current_cost)) = to_analyze.pop_front() {
        get_neighbours(next, map).for_each(|(r, c)| {
            let new_cost = current_cost + 1;
            if can_move(map, next, (r, c)) && new_cost < cost[r][c]  {
                cost[r][c] = new_cost;
                to_analyze.push_back(((r, c), new_cost));
            }
        })
    }
    cost
}

pub fn part1(input: &str) -> u16 {
    let map = parse(input);
    let cost = get_cost_map(&map);
    cost[map.start.0][map.start.1]
}

pub fn part2(input: &str) -> u16 {
    let map = parse(input);
    let cost = get_cost_map(&map);
    iproduct!(0..map.height, 0..map.width)
        .filter(|(r, c)| map.content[*r][*c] == 1)
        .map(|(r, c)| cost[r][c])
        .min().expect("There should be min")
}

#[test]
fn test() {
    crate::test_day!(12, 31, 29)
}
