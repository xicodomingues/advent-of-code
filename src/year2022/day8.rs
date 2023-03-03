use grid::Grid;
use itertools::Itertools;

fn to_digit(c: char) -> u8 {
    c as u8 - b'0'
}

fn parse_forest(input: &str) -> Grid<u8> {
    let size = input.find('\n').unwrap();
    let elems = input.lines().flat_map(|line| line.chars().map(to_digit)).collect();
    Grid::from_vec(elems, size)
}

fn is_visible_from_outside(forest: &Grid<u8>, (r, c): (usize, usize)) -> bool {
    let height = forest[r][c];
    let to_up = forest.iter_col(c).take(r).all(|x| *x < height);
    let to_down = forest.iter_col(c).skip(r+1).all(|x| *x < height);
    let to_left = forest.iter_row(r).take(c).all(|x| *x < height);
    let to_right = forest.iter_row(r).skip(c+1).all(|x| *x < height);
    to_left || to_right || to_up || to_down
}

fn grid_indexes(forest: &Grid<u8>) -> impl Iterator<Item=(usize, usize)> {
    (0..forest.rows()).cartesian_product(0..forest.cols())
}

pub fn part1(input: &str) -> usize {
    let forest = parse_forest(input);
    grid_indexes(&forest)
        .filter(|index| is_visible_from_outside(&forest, *index))
        .count()
}

fn get_taller_pos<'a>(mut iter: impl Iterator<Item=&'a u8>, height: u8, default: usize) -> usize {
    iter.find_position(|h| *h >= &height).map(|(i, _)| i + 1).unwrap_or(default)
}

// scan each direction until a same size tree or taller shows up.
// if none shows up return max distance
fn get_hidden_space(forest: &Grid<u8>, (r, c): (usize, usize)) -> usize {
    let height = forest[r][c];
    let to_up = get_taller_pos(forest.iter_col(c).take(r).rev(), height, r);
    let to_down = get_taller_pos(forest.iter_col(c).skip(r+1), height, forest.rows() - 1 - r);
    let to_left = get_taller_pos(forest.iter_row(r).take(c).rev(), height, c);
    let to_right = get_taller_pos(forest.iter_row(r).skip(c+1), height, forest.cols() - 1 - c);
    to_left * to_right * to_up * to_down
}

pub fn part2(input: &str) -> usize {
    let forest = parse_forest(input);
    grid_indexes(&forest)
        .map(|index| get_hidden_space(&forest, index))
        .max().expect("There should be at least one elem")
}

#[test]
fn test() {
    crate::test_2022!(8, 21, 8)
}
