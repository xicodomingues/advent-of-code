use std::collections::{HashMap, HashSet};

use grid::Grid;
use itertools::Itertools;

use crate::my_dbg;
use crate::utils::{Direction, MyGrid, Point};

use crate::utils::Direction::*;

const BLA: &str = r###"
function A_Star(start, goal, h)
    // The set of discovered nodes that may need to be (re-)expanded.
    // Initially, only the start node is known.
    // This is usually implemented as a min-heap or priority queue rather than a hash-set.
    openSet := {start}

    // For node n, cameFrom[n] is the node immediately preceding it on the cheapest path from the start
    // to n currently known.
    cameFrom := an empty map

    // For node n, gScore[n] is the cost of the cheapest path from start to n currently known.
    gScore := map with default value of Infinity
    gScore[start] := 0

    // For node n, fScore[n] := gScore[n] + h(n). fScore[n] represents our current best guess as to
    // how cheap a path could be from start to finish if it goes through n.
    fScore := map with default value of Infinity
    fScore[start] := h(start)

    while openSet is not empty
        // This operation can occur in O(Log(N)) time if openSet is a min-heap or a priority queue
        current := the node in openSet having the lowest fScore[] value
        if current = goal
            return reconstruct_path(cameFrom, current)

        openSet.Remove(current)
        for each neighbor of current
            // d(current,neighbor) is the weight of the edge from current to neighbor
            // tentative_gScore is the distance from start to the neighbor through current
            tentative_gScore := gScore[current] + d(current, neighbor)
            if tentative_gScore < gScore[neighbor]
                // This path to neighbor is better than any previous one. Record it!
                cameFrom[neighbor] := current
                gScore[neighbor] := tentative_gScore
                fScore[neighbor] := tentative_gScore + h(neighbor)
                if neighbor not in openSet
                    openSet.add(neighbor)

    // Open set is empty but goal was never reached
    return failure
"###;

type Path = (Point, Direction);

fn do_a_star(grid: &MyGrid<u8>) -> u16 {
    fn get_cheapest(open_set: &HashSet<Path>, f_score: &HashMap<Path, u16>) -> Path {
        let mut min = u16::MAX;
        let mut res = None;
        for p in open_set {
            if f_score[p] < min {
                min = f_score[p];
                res = Some(p.clone());
            }
        }
        res.unwrap()
    }

    fn get_neighbours(grid: &MyGrid<u8>, path: &Path) -> Vec<(Path, u16)> {
        let (point, dir) = path;

        let calculate_cost = |dire, times| {
            let mut total = 0;
            for i in 1..=times {
                total += grid[&point.move_in(&dire, i)];
            }
            total as u16
        };

        let rleft = dir.rotate(crate::utils::Rotation::AntiClockwise);
        let rright = dir.rotate(crate::utils::Rotation::Clockwise);
        let all = vec![
            (point.move_in(&rleft, 4), rleft, 4),
            (point.move_in(&rleft, 5), rleft, 5),
            (point.move_in(&rleft, 6), rleft, 6),
            (point.move_in(&rleft, 7), rleft, 7),
            (point.move_in(&rleft, 8), rleft, 8),
            (point.move_in(&rleft, 9), rleft, 9),
            (point.move_in(&rleft, 10), rleft, 10),
            (point.move_in(&rright, 4), rright, 4),
            (point.move_in(&rright, 5), rright, 5),
            (point.move_in(&rright, 6), rright, 6),
            (point.move_in(&rright, 7), rright, 7),
            (point.move_in(&rright, 8), rright, 8),
            (point.move_in(&rright, 9), rright, 9),
            (point.move_in(&rright, 10), rright, 10),
        ];

        all.into_iter()
            .filter(|(p, _, _)| grid.contains(p))
            .map(|(p, d, times)| ((p, d), calculate_cost(d, times)))
            .collect()
    }

    fn get_complete_path(came_from: &HashMap<Path, Path>, end: &Path) -> Vec<Path> {
        if !came_from.contains_key(end) {
            return vec![];
        }
        let mut tmp = get_complete_path(came_from, &came_from[end]);
        tmp.push(end.clone());
        tmp
    }

    // FIXME: can start in teo directions, not only one.
    let start = (Point::new(0, 0), Up);
    let end = Point::new(grid.rows() as isize - 1, grid.cols() as isize - 1);
    let h = |path: &Path| path.0.manhathan_dist(&end) as u16;

    let mut open_set = HashSet::<Path>::new();
    open_set.insert(start.clone());

    let mut came_from = HashMap::<Path, Path>::new();
    let mut g_score = HashMap::<Path, u16>::new();
    g_score.insert(start.clone(), 0);

    let mut f_score = HashMap::<(Point, Direction), u16>::new();
    f_score.insert(start.clone(), h(&start));

    while !open_set.is_empty() {
        let path = get_cheapest(&open_set, &f_score);
        if path.0 == end {
            // Maybe there is a bug here, should check for the min of all ends
            let end_state = g_score
                .iter()
                .filter(|((p, _), _)| *p == end)
                .map(|(_, v)| v)
                .min()
                .unwrap();
            // unroll the points
            my_dbg!(get_complete_path(&came_from, &path));

            return *end_state;
        }

        open_set.remove(&path);

        for (neighbor, cost) in get_neighbours(&grid, &path) {
            let tentative_g_score = g_score[&path] + cost;
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or_else(|| &u16::MAX) {
                came_from.insert(neighbor.clone(), path.clone());
                g_score.insert(neighbor.clone(), tentative_g_score);
                f_score.insert(neighbor.clone(), tentative_g_score + h(&neighbor));
                if !open_set.contains(&neighbor) {
                    open_set.insert(neighbor);
                }
            }
        }
    }
    panic!("should have found a solution")
}

pub fn part1(input: &str) -> usize {
    let grid = MyGrid::parse(input, |x| x - b'0');
    do_a_star(&grid) as usize
}

pub fn part2(input: &str) -> usize {
    let grid = MyGrid::parse(input, |x| x - b'0');
    do_a_star(&grid) as usize
}

#[test]
fn test() {
    test_2023!(17, 102);
}
