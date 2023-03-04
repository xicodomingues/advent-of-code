use std::{sync::{Arc, Mutex}, thread::spawn};

use md5::{Context, Digest};

const NUM_THREADS: u8 = 10;
const CHECK_INC: u64 = 10_000;

/// test_fn receives a [u8] where each u8 represents 2 chars in the md5 output
fn find_hash(input: &str, test_fn: fn(Digest) -> bool) -> u64 {
    let mut ctx = Context::new();
    ctx.consume(input.as_bytes());

    // the tuple represents: (found_value, last_counter)
    let info = Arc::new(Mutex::new((u64::MAX, 0)));
    let mut thread_handles = vec![];

    for _ in 0..NUM_THREADS {
        let info = Arc::clone(&info);
        let ctx = ctx.clone();
        let thread = spawn(move || {
            let mut start;
            let mut res = None;

            loop {
                // get initial values for the calculations
                {
                    let mut bla = info.lock().unwrap();
                    if bla.0 != u64::MAX {
                        break;
                    }
                    start = bla.1;
                    bla.1 += CHECK_INC;
                }

                // perform the calculations
                for i in start..start + CHECK_INC {
                    let mut test = ctx.clone();
                    test.consume(format!("{}", i));
                    let digested = test.compute();
                    if test_fn(digested) {
                        res = Some(i);
                        break;
                    }
                }

                // update info, if value found
                if let Some(val) = res {
                    let mut bla = info.lock().unwrap();
                    if bla.0 > val {
                        bla.0 = val;
                    }
                    break;
                }
            }
        });
        thread_handles.push(thread);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }

    return info.lock().unwrap().0;
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
