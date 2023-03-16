
fn decode_count(input: &str) -> usize {
    let mut count = 0;
    let mut i = 0;
    let chars: Vec<_> = input.chars().collect();
    while i < input.len() {
        match chars[i] {
            '"' => if i != 0 && i != input.len() - 1 { count += 1; }
            '\\' => if chars[i+1] == 'x' {count += 1; i += 3} else {count += 1; i += 1}
            _ => count += 1
        };
        i += 1;
    }
    count
}

fn encode_count(input: &str) -> usize {
    input.chars().map(|c| match c {
        '"' => 2,
        '\\' => 2,
        _ => 1
    }).sum::<usize>() + 2 // for the outer ""
}

pub fn part1(input: &str) -> usize {
    let chars: usize = input.lines().map(|line| line.len()).sum();
    let real_chars: usize = input.lines().map(decode_count).sum();
    chars - real_chars
}

pub fn part2(input: &str) -> usize {
    let chars: usize = input.lines().map(|line| line.len()).sum();
    let encoded: usize = input.lines().map(encode_count).sum();
    encoded - chars
}

#[test]
fn test() {
    crate::test_2015!(8, 12, 19)
}
