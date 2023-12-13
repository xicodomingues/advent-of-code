use std::collections::VecDeque;

use grid::Grid;
use itertools::iproduct;

use crate::utils::{MyGrid, Point};

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    content: MyGrid<u8>,
    start: Point,
    end: Point,
}

impl Map {
    fn contains(&self, point: &Point) -> bool {
        self.content.contains(point)
    }
}

fn parse(input: &str) -> Map {
    let mut grid: MyGrid<u8> = MyGrid(Grid::new(0, 0));
    let mut start = None;
    let mut end = None;
    for (r, line) in input.lines().enumerate() {
        grid.push_row(line.chars().enumerate().map(|(c, char)| {
            match char {
                'S' => {
                    start = Some(Point::from((r, c)));
                    1
                }
                'E' => {
                    end = Some(Point::from((r, c)));
                    26
                }
                'a'..='z' => char as u8 - b'a' + 1,
                _ => panic!("This character should not be here")
            }
        }).collect());
    }
    Map {
        width: grid.cols(),
        height: grid.rows(),
        content: grid,
        start: start.expect("There was no start on the provided map"),
        end: end.expect("There was no end on the provided map"),
    }
}

fn get_neighbours(coord: Point, map: &Map) -> impl Iterator<Item=Point> + '_ {
    coord.neighbors().into_iter()
        .filter(|points| map.contains(points))
}

fn can_move(map: &Map, current: Point, next: Point) -> bool {
    let from = map.content[current];
    let to = map.content[next];
    from <= to + 1
}

fn get_cost_map(map: &Map) -> MyGrid<u16> {
    let mut cost: MyGrid<u16> = MyGrid(Grid::init(map.height, map.width, u16::MAX));
    let mut to_analyze = VecDeque::new();
    to_analyze.push_back((map.end, 0));
    cost[map.end] = 0;

    while let Some((next, current_cost)) = to_analyze.pop_front() {
        get_neighbours(next, map).for_each(|point| {
            let new_cost = current_cost + 1;
            if can_move(map, next, point) && new_cost < cost[point] {
                cost[point] = new_cost;
                to_analyze.push_back((point, new_cost));
            }
        })
    }
    cost
}

pub fn part1(input: &str) -> u16 {
    let map = parse(input);
    let cost = get_cost_map(&map);
    cost[map.start]
}

pub fn part2(input: &str) -> u16 {
    let map = parse(input);
    let cost = get_cost_map(&map);
    iproduct!(0..map.height, 0..map.width)
        .filter(|point| map.content[*point] == 1)
        .map(|point| cost[point])
        .min().expect("There should be min")
}

#[test]
fn test() {
    crate::test_2022!(12, 31, 29)
}
