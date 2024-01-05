use std::cmp::{min, Ordering};
use std::collections::BinaryHeap;
use std::ops::Index;

use grid::Grid;

use crate::utils::{Direction, MyGrid, Point};

use crate::utils::Direction::*;

type Path = (Point, Direction);

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: u16,
    position: Path,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_neighbors_fn(start: isize, end: isize) -> impl Fn(&MyGrid<u8>, &Path) -> Vec<(Path, u16)> {
    move |grid: &MyGrid<u8>, path: &Path| -> Vec<(Path, u16)> {
        let (point, dir) = path;

        let get_costs = |dir| -> Vec<(Path, u16)> {
            let mut points = vec![];

            let mut cost = 0;
            let mut curr = 1;
            let mut pos = point.clone();
            while curr <= end {
                pos = pos.move1(&dir);
                if !grid.contains(&pos) {
                    break;
                }

                cost += grid[&pos] as u16;
                if curr >= start {
                    points.push(((pos.clone(), dir), cost));
                }
                curr += 1;
            }
            points
        };

        let mut points = get_costs(dir.rotate(crate::utils::Rotation::AntiClockwise));
        points.extend(get_costs(dir.rotate(crate::utils::Rotation::Clockwise)));
        points
    }
}

struct Score {
    grid: MyGrid<(u16, u16, u16, u16)>,
}

impl Score {
    fn new(grid: &MyGrid<u8>) -> Score {
        let x = Grid::from_vec(
            vec![(u16::MAX, u16::MAX, u16::MAX, u16::MAX); grid.rows() * grid.cols()],
            grid.cols(),
        );
        Score { grid: MyGrid(x) }
    }

    fn insert(&mut self, path: &Path, cost: u16) {
        let curr = self.grid[&path.0];
        let new = match path.1 {
            Up => (cost, curr.1, curr.2, curr.3),
            Down => (curr.0, cost, curr.2, curr.3),
            Left => (curr.0, curr.1, cost, curr.3),
            Right => (curr.0, curr.1, curr.2, cost),
        };
        self.grid[&path.0] = new;
    }

    fn get_min_score(&self, point: &Point) -> u16 {
        let (a, b, c, d) = self.grid[point];
        min(min(min(a, b), c), d)
    }
}

impl Index<&Path> for Score {
    type Output = u16;

    fn index(&self, index: &Path) -> &Self::Output {
        match index.1 {
            Up => &self.grid[&index.0].0,
            Down => &self.grid[&index.0].1,
            Left => &self.grid[&index.0].2,
            Right => &self.grid[&index.0].3,
        }
    }
}

fn do_dijkstra(
    grid: &MyGrid<u8>,
    get_neighbours: impl Fn(&MyGrid<u8>, &Path) -> Vec<(Path, u16)>,
) -> u16 {
    let start1 = (Point::new(0, 0), Up);
    let start2 = (Point::new(0, 0), Left);
    let end = Point::new(grid.rows() as isize - 1, grid.cols() as isize - 1);

    let mut open_set = BinaryHeap::<State>::new();
    open_set.push(State {
        cost: 0,
        position: start1.clone(),
    });
    open_set.push(State {
        cost: 0,
        position: start1.clone(),
    });

    let mut g_score = Score::new(grid);
    g_score.insert(&start1, 0);
    g_score.insert(&start2, 0);

    while let Some(State {
        position: path,
        cost,
    }) = open_set.pop()
    {
        if path.0 == end {
            return g_score.get_min_score(&path.0);
        }

        if cost > g_score[&path] {
            continue;
        }

        for (neighbor, distance) in get_neighbours(grid, &path) {
            let tentative_g_score = g_score[&path] + distance;
            if tentative_g_score < g_score[&neighbor] {
                g_score.insert(&neighbor, tentative_g_score);
                open_set.push(State {
                    cost: tentative_g_score,
                    position: neighbor,
                });
            }
        }
    }
    panic!("should have found a solution")
}

pub fn part1(input: &str) -> usize {
    let grid = MyGrid::parse(input, |x| x - b'0');
    do_dijkstra(&grid, get_neighbors_fn(1, 3)) as usize
}

pub fn part2(input: &str) -> usize {
    let grid = MyGrid::parse(input, |x| x - b'0');
    do_dijkstra(&grid, get_neighbors_fn(4, 10)) as usize
}

#[test]
fn test() {
    test_2023!(17, 102, 94);
}
