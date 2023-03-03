use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn solve(input: &str, buffer_size: usize) -> i64 {
    let chars: Vec<char> = input.chars().collect();
    let mut char_count = HashMap::<char, usize>::new();
    // initialize window
    for c in &chars[0..buffer_size] {
        char_count.entry(*c).and_modify(|x| *x += 1).or_insert(1);
    }
    // if there are no repetitions on initial set, return minimal value
    if char_count.values().len() == buffer_size {
        return buffer_size as i64;
    }
    for i in 0..(chars.len() - buffer_size) {
        // remove the 'deprecated' entry
        if let Entry::Occupied(mut o) = char_count.entry(chars[i]) {
            let val = o.get_mut();
            *val -= 1;
            if *val == 0 { o.remove_entry(); }
        }
        // add the new one if it was repeated
        char_count.entry(chars[i + buffer_size]).and_modify(|x| *x += 1).or_insert(1);

        // check if all entries are unique
        if char_count.values().len() == buffer_size {
            return (i + buffer_size + 1) as i64;
        }
    }
    panic!("The program should not reach here")
}

pub fn part1(input: &str) -> i64 {
    solve(input, 4)
}

pub fn part2(input: &str) -> i64 {
    solve(input, 14)
}

#[test]
fn test() {
    assert_eq!(part1("vwbjplbgvbhsrlpgdmjqwftvncz"), 4);
    assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);

    assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
}
