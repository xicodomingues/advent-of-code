use std::cmp::{max, min};

use itertools::{enumerate, Itertools};

use crate::utils::MyGrid;

static EMPTY_SPACE: u8 = b'.';

#[derive(Debug)]
struct Universe {
    grid: MyGrid<u8>,
    expand_cols: Vec<usize>,
    expand_rows: Vec<usize>,
}

impl Universe {
    fn parse(input: &str) -> Self {
        fn get_empty<'a>(iter: impl Iterator<Item = impl Iterator<Item = &'a u8>>) -> Vec<usize> {
            enumerate(iter)
                .filter_map(|(c, mut x)| {
                    if x.all(|x| x == &EMPTY_SPACE) {
                        Some(c)
                    } else {
                        None
                    }
                })
                .collect()
        }
        let grid = MyGrid::bparse(input);
        let expand_cols = get_empty(grid.iter_cols());
        let expand_rows = get_empty(grid.iter_rows());
        Self {
            grid,
            expand_cols,
            expand_rows,
        }
    }

    fn get_distances(&self, expand_factor: usize) -> usize {
        let get_dist = |(a, b): (usize, usize), expand: &[usize]| -> usize {
            let gaps = expand
                .iter()
                .skip_while(|x| x < &&min(a, b))
                .take_while(|x| x < &&max(a, b))
                .count();
            gaps * (expand_factor - 1) + (a as isize - b as isize).unsigned_abs()
        };

        let calculate_dist = |((ar, ac), (br, bc)): ((usize, usize), (usize, usize))| -> usize {
            get_dist((ar, br), &self.expand_rows) + get_dist((ac, bc), &self.expand_cols)
        };

        self.grid
            .find_all(b'#')
            .combinations(2)
            .map(|v| calculate_dist((v[0], v[1])))
            .sum()
    }
}

pub fn part1(input: &str) -> usize {
    Universe::parse(input).get_distances(2)
}

pub fn part2(input: &str) -> usize {
    Universe::parse(input).get_distances(1_000_000)
}

#[test]
fn test() {
    use crate::utils::load_test_file;

    test_2023!(11, 374);

    let input = load_test_file(2023, 11);
    assert_eq!(Universe::parse(&input).get_distances(10), 1030);
    assert_eq!(Universe::parse(&input).get_distances(100), 8410);
}
