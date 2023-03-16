
fn apply_step(input: &[u8]) -> Vec<u8> {
    let mut i = 0;
    let mut res = vec![];
    while i < input.len() {
        match (input.get(i), input.get(i+1), input.get(i+2)) {
            (Some(a), Some(b), Some(c)) if a == b && b == c => {
                res.push('3' as u8);
                res.push(*a);
                i += 2;
            },
            (Some(a), Some(b), _) if a == b => {
                res.push('2' as u8);
                res.push(*a);
                i += 1;
            },
            (Some(a), _, _) => {
                res.push('1' as u8);
                res.push(*a);
            },
            _ => {}
        }
        i += 1;
    }
    res
}

fn solve(input: &str, times: usize) -> usize {
    let mut res: Vec<u8> = input.as_bytes().to_vec();
    for _ in 0..times {
        res = apply_step(&res);
    }
    res.len()
}

pub fn part1(input: &str) -> usize {
    solve(input, 40)
}

pub fn part2(input: &str) -> usize {
    solve(input, 50)
}

#[test]
fn test() {
    assert_eq!(apply_step("1".as_bytes()), "11".as_bytes());
    assert_eq!(apply_step("11".as_bytes()), "21".as_bytes());
    assert_eq!(apply_step("21".as_bytes()), "1211".as_bytes());
    assert_eq!(apply_step("1211".as_bytes()), "111221".as_bytes());
    assert_eq!(apply_step("111221".as_bytes()), "312211".as_bytes());
}
