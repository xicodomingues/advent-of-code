use md5::{Context, Digest};

/// test_fn recieves a [u8] where each u8 represents 2 chars in the md5 output
fn find_hash(input: &str, test_fn: fn(Digest) -> bool) -> u64 {
    let mut i = 1;
    let mut ctx = Context::new();
    ctx.consume(input.as_bytes());
    
    loop {
        let mut test = ctx.clone();
        test.consume(format!("{}", i));
        let res = test.compute();
        if test_fn(res) {
            return i;
        }
        i += 1;
    }
}

pub fn part1(input: &str) -> u64 {
    find_hash(input, |hash| hash[0] == 0 && hash[1] == 0 && hash[2] < 8)
}

pub fn part2(input: &str) -> u64 {
    find_hash(input, |hash| hash.starts_with(b"\0\0\0"))
}

#[test]
fn test() {
    assert_eq!(part1("abcdef"), 609043);
    assert_eq!(part1("pqrstuv"), 1048970);
    
    assert_eq!(part2("abcdef"), 6742839);
}
