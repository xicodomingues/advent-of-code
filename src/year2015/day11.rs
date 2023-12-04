use std::collections::HashSet;

use itertools::Itertools;

fn is_valid_pass(pass: &[u8]) -> bool {
    let increasing = pass.iter().tuple_windows::<(_, _, _)>().any(|(a, b, c)| *a + 1 == *b && *b + 1 == *c);
    let no_iol = pass.iter().all(|x| *x != b'i' && *x != b'o' && *x != b'l');
    let pairs = 2 <= HashSet::<(&u8, &u8)>::from_iter(pass.iter().tuple_windows::<(_, _)>().filter(|(a, b)| a == b)).len();
    increasing && no_iol && pairs
}

fn next_pass(pass: &mut Vec<u8>) {
    let mut pos = (pass.len() - 1) as isize;
    while pos >= 0 {
        let c = pass[pos as usize];
        if c != b'z' {
            pass[pos as usize] = c + 1;
            break;
        } else {
            pass[pos as usize] = b'a';
        }
        pos -= 1;
    }
}

pub fn part1(input: &str) -> String {
    let mut pass = input.as_bytes().to_vec();
    loop {
        next_pass(&mut pass);
        if is_valid_pass(&pass) {
            return String::from_utf8_lossy(&pass).to_string();
        }
    }
}

pub fn part2(input: &str) -> String {
    part1(&part1(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid_pass("hijklmmn".as_bytes()), false);
        assert_eq!(is_valid_pass("abbceffg".as_bytes()), false);
        assert_eq!(is_valid_pass("abbcegjk".as_bytes()), false);
        assert_eq!(is_valid_pass("abcdffaa".as_bytes()), true);
        assert_eq!(is_valid_pass("ghjaabcc".as_bytes()), true);
    }
    
    #[test]
    fn test_next_pass() {
        let mut pass = "aaaaa".as_bytes().to_vec();
        next_pass(&mut pass);
        assert_eq!(String::from_utf8_lossy(&pass), "aaaab");
        
        let mut pass = "aaaaz".as_bytes().to_vec();
        next_pass(&mut pass);
        assert_eq!(String::from_utf8_lossy(&pass), "aaaba");

        let mut pass = "azzzz".as_bytes().to_vec();
        next_pass(&mut pass);
        assert_eq!(String::from_utf8_lossy(&pass), "baaaa");
    }
    
    #[test]
    fn test_part1() {
        assert_eq!(part1("abcdefgh"), "abcdffaa");
        assert_eq!(part1("ghijklmn"), "ghjaabcc");
    }
}