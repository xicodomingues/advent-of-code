use crate::utils::MyGrid;

fn parse(input: &str) -> impl Iterator<Item = MyGrid<u8>> + '_ {
    input.split("\n\n").map(|entries| MyGrid::parse(entries, |x| x))
}

/// row 0 means that the relfection point is between row 0 and 1
fn is_reflection_part1(grid: &MyGrid<u8>, row: usize) -> bool {
    let mut start: isize = row as isize;
    let mut end = row + 1;

    while start >= 0 && end < grid.rows() {
        if !grid
            .iter_row(start as usize).copied()
            .zip(grid.iter_row(end).copied())
            .all(|(a, b)| a == b) {
                return false;
            }
        start -= 1;
        end += 1;
    }
    true
}

/// row 0 means that the relfection point is between row 0 and 1
fn is_reflection_part2(grid: &MyGrid<u8>, row: usize) -> bool {
    let mut start: isize = row as isize;
    let mut end = row + 1;
    let mut mutation = 1_isize;

    while start >= 0 && end < grid.rows() {
        let pairs = grid
            .iter_row(start as usize).copied()
            .zip(grid.iter_row(end).copied());
        let diff = pairs.filter(|(a, b)| a != b).count();
        match (diff, mutation) {
            (1, 1) => mutation -= 1,
            (1, 0) => return false,
            (0, _) => {}
            _ => return false
        }
        start -= 1;
        end += 1;
    }
    mutation == 0
}

fn find_reflection(grid: &mut MyGrid<u8>, is_reflection: fn(grid: &MyGrid<u8>, row: usize) -> bool) -> usize {

    fn get_reflection_pos(grid: &MyGrid<u8>, is_reflection: fn(grid: &MyGrid<u8>, row: usize) -> bool) -> usize {
        for row in 0..grid.rows() - 1 {
            if is_reflection(grid, row) {
                return row + 1;
            }
        }
        0
    }
    let res = get_reflection_pos(grid, is_reflection) * 100;
    if res > 0 {
        return res;
    }
    grid.transpose();
    let res = get_reflection_pos(grid, is_reflection);
    assert!(res != 0, "There should be a reflection");
    res
}

#[test]
fn test_reflect() {
    use indoc::indoc;
    let mut grid = MyGrid::parse(indoc! {"
        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    "}, |x| x);
    assert_eq!(find_reflection(&mut grid, is_reflection_part1), 400);
    assert_eq!(find_reflection(&mut grid, is_reflection_part2), 100);
}

pub fn part1(input: &str) -> usize {
    parse(input).map(|mut grid| find_reflection(&mut grid, is_reflection_part1)).sum()
}

pub fn part2(input: &str) -> usize {
    parse(input).map(|mut grid| find_reflection(&mut grid, is_reflection_part2)).sum()
}

#[test]
fn test() {
    test_2023!(13, 405, 400);
}
