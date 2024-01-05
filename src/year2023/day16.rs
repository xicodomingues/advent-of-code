use bit_set::BitSet;
use grid::Grid;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use crate::utils::Direction::*;
use crate::utils::{Direction, MyGrid, Point};

type Beam = (Point, Direction);

fn calculate_interception(c: u8, (pos, dir): Beam) -> impl Iterator<Item = Beam> {
    fn it(pos: Point, dir: Direction) -> std::vec::IntoIter<Beam> {
        vec![(pos.move1(&dir), dir)].into_iter()
    }

    fn it2((p1, d1): Beam, (p2, d2): Beam) -> std::vec::IntoIter<Beam> {
        vec![(p1.move1(&d1), d1), (p2.move1(&d2), d2)].into_iter()
    }

    match (c, dir) {
        // No change
        (b'|', Down | Up) => it(pos, dir),
        (b'-', Left | Right) => it(pos, dir),
        // Split
        (b'-', Down | Up) => it2((pos.clone(), Left), (pos, Right)),
        (b'|', Left | Right) => vec![(pos.clone(), Up), (pos, Down)].into_iter(),
        // Reflect
        (b'/', Up) => it(pos, Right),
        (b'/', Down) => it(pos, Left),
        (b'/', Left) => it(pos, Down),
        (b'/', Right) => it(pos, Up),

        // Reflect
        (b'\\', Up) => it(pos, Left),
        (b'\\', Down) => it(pos, Right),
        (b'\\', Left) => it(pos, Up),
        (b'\\', Right) => it(pos, Down),

        _ => panic!("Not a valid combination"),
    }
}

fn shine_from(grid: &MyGrid<u8>, beam: Beam, visited: &mut MyGrid<BitSet>) -> usize {
    let mut beams = vec![beam];

    'outer: while let Some(beam) = beams.pop() {
        let (mut pos, dir) = beam;
        loop {
            //println!("{} {}", pos, dir);
            if !grid.contains(&pos) {
                continue 'outer;
            }
            if visited[&pos].contains(dir as usize) {
                continue 'outer;
            }
            visited[&pos].insert(dir as usize);
            match grid[&pos] {
                b'.' => {}
                c => {
                    beams.extend(calculate_interception(c, (pos, dir)));
                    continue 'outer;
                }
            }
            pos = pos.move1(&dir);
        }
    }

    visited.iter().filter(|x| !x.is_empty()).count()
}

fn p(r: isize, c: isize) -> Point {
    Point::from((r, c))
}

pub fn part1(input: &str) -> usize {
    let grid = MyGrid::parse(input, |x| x);
    let mut visited = MyGrid(Grid::from_vec(
        vec![BitSet::new(); grid.cols() * grid.rows()],
        grid.cols(),
    ));

    shine_from(&grid, (p(0, 0), Right), &mut visited)
}

pub fn part2(input: &str) -> usize {
    let grid = MyGrid::parse(input, |x| x);

    let visited = MyGrid(Grid::from_vec(
        vec![BitSet::new(); grid.cols() * grid.rows()],
        grid.cols(),
    ));

    let mut beams = vec![];
    (0..grid.rows()).for_each(|row| {
        let r = row as isize;
        beams.push((p(r, 0), Right));
        beams.push((p(r, (grid.cols() - 1) as isize), Left));
    });
    (0..grid.cols()).for_each(|col| {
        let c = col as isize;
        beams.push((p(0, c), Down));
        beams.push((p((grid.rows() - 1) as isize, c), Up));
    });

    beams
        .par_iter()
        .map(|beam| {
            shine_from(
                &MyGrid(grid.0.clone()),
                beam.clone(),
                &mut MyGrid(visited.0.clone()),
            )
        })
        .max()
        .unwrap()
}

#[test]
fn test() {
    test_2023!(16, 46, 51);
}
