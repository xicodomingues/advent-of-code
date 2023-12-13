use std::collections::{HashSet, HashMap};

use itertools::Itertools;

use crate::utils::{Direction, MyGrid, Point};

static EMPTY_CHAR: char = '.';

fn next(grid: &MyGrid<char>, point: &Point, prev: &Point) -> Point {
    match grid[point] {
        '-' => {
            if &point.left() == prev {
                point.right()
            } else {
                point.left()
            }
        }
        '|' => {
            if &point.up() == prev {
                point.down()
            } else {
                point.up()
            }
        }
        '7' => {
            if &point.left() == prev {
                point.down()
            } else {
                point.left()
            }
        }
        'J' => {
            if &point.up() == prev {
                point.left()
            } else {
                point.up()
            }
        }
        'L' => {
            if &point.up() == prev {
                point.right()
            } else {
                point.up()
            }
        }
        'F' => {
            if &point.down() == prev {
                point.right()
            } else {
                point.down()
            }
        }
        x => panic!("unrecognized char {}", x),
    }
}

fn find_connected(grid: &MyGrid<char>, point: &Point) -> ((Point, Point), char) {
    let mut res = vec![];
    let left = point.left();
    let mut dirs = vec![];
    if grid.contains(&left) && "-FL".contains(grid[left]) {
        res.push(left);
        dirs.push(Direction::Left);
    }
    let right = point.right();
    if grid.contains(&right) && "-7J".contains(grid[right]) {
        res.push(right);
        dirs.push(Direction::Right);
    }
    let up = point.up();
    if grid.contains(&up) && "|F7".contains(grid[up]) {
        res.push(up);
        dirs.push(Direction::Up);
    }
    let down = point.down();
    if grid.contains(&down) && "|LJ".contains(grid[down]) {
        res.push(down);
        dirs.push(Direction::Down);
    }
    assert!(res.len() == 2);
    let points = (res[0], res[1]);
    let new_char = match (dirs[0], dirs[1]) {
        (Direction::Left, Direction::Right) => '-',
        (Direction::Up, Direction::Down) => '|',
        (Direction::Left, Direction::Up) => 'J',
        (Direction::Left, Direction::Down) => '7',
        (Direction::Right, Direction::Up) => 'L',
        (Direction::Right, Direction::Down) => 'F',
        _ => panic!("should not happen!"),
    };
    (points, new_char)
}

fn get_line_points(grid: &MyGrid<char>, point: Point) -> HashSet<Point> {
    let mut prev_p1 = point;
    let ((mut p1, _), _) = find_connected(grid, &point);
    let mut res = HashSet::new();
    while p1 != point {
        let n = next(grid, &p1, &prev_p1);
        res.insert(prev_p1);
        prev_p1 = p1;
        p1 = n;
    }
    res.insert(prev_p1);
    res
}

fn clean_grid(grid: &mut MyGrid<char>, points: &HashSet<Point>) {
    for r in 0..grid.size().0 {
        for c in 0..grid.size().1 {
            let p = Point::from((r, c));
            if !points.contains(&p) {
                grid[(r, c)] = EMPTY_CHAR
            }
        }
    }
}

fn is_inside(grid: &MyGrid<char>, point: (usize, usize)) -> bool {
    fn val(counts: &HashMap<char, usize>, val: char) -> isize {
        *counts.get(&val).unwrap_or(&0) as isize
    }
    if grid[point] != EMPTY_CHAR {
        return false;
    }
    let tmp: String = grid
        .to_edge(&Point::from(point), Direction::Up)
        .filter(|x| x != &&EMPTY_CHAR)
        .filter(|x| x != &&'|')
        .collect();
    if tmp.is_empty() {
        return false;
    }
    let tmp = tmp.replace("LF", "")
        .replace("J7", "")
        .replace("L7", "-")
        .replace("JF", "-");
    assert!(tmp.chars().all(|x| x == '-'));
    tmp.len() % 2 == 1
}

#[test]
fn test_fns() {
    use indoc::indoc;
    fn p(r: usize, c: usize) -> Point {
        Point::from((r, c))
    }
    let grid = MyGrid::from_str(indoc! {"
        .....
        .F-7.
        .|.|.
        .L-J.
        .....
    "});

    assert_eq!(next(&grid, &p(1, 1), &p(1, 2)), p(2, 1));
    assert_eq!(next(&grid, &p(1, 1), &p(2, 1)), p(1, 2));
    assert_eq!(next(&grid, &p(3, 3), &p(2, 3)), p(3, 2));

    assert_eq!(find_connected(&grid, &p(1, 1)), ((p(1, 2), p(2, 1)), 'F'));
    assert_eq!(find_connected(&grid, &p(3, 2)), ((p(3, 1), p(3, 3)), '-'));
}

pub fn part1(input: &str) -> usize {
    let grid = MyGrid::from_str(input);
    let start = Point::from(grid.find('S').unwrap());
    get_line_points(&grid, start).len() / 2
}

pub fn part2(input: &str) -> usize {
    let mut grid = MyGrid::from_str(input);
    let start = Point::from(grid.find('S').unwrap());
    let points = get_line_points(&grid, start);
    grid[start] = find_connected(&grid, &start).1;
    clean_grid(&mut grid, &points);
    let points = grid.indexed_iter()
        .filter(|(p, _)| is_inside(&grid, *p))
        .map(|(p, _)| p)
        .collect_vec();
    for p in &points {
        grid[*p] = 'X';
    }
    points.len()
}

#[test]
fn test() {
    use indoc::indoc;
    assert_eq!(
        part1(indoc! {"
            -L|F7
            7S-7|
            L|7||
            -L-J|
            L|-JF
        "}),
        4
    );

    assert_eq!(
        part1(indoc! {"
            7-F7-
            .FJ|7
            SJLL7
            |F--J
            LJ.LJ
        "}),
        8
    );

    assert_eq!(
        part2(indoc! {"
            -L|F7
            7S-7|
            L|7||
            -L-J|
            L|-JF
        "}),
        1
    );

    assert_eq!(
        part2(indoc! {"
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
        "}),
        4
    );

    assert_eq!(
        part2(indoc! {"
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
        "}),
        8
    );

    assert_eq!(
        part2(indoc! {"
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
        "}),
        10
    );
}
