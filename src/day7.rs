use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Command {
    // name
    cd(String),
    ls,
    // name
    dir(String),
    // name, size
    file(String, u64),
}

lazy_static! {
    static ref FILE_RE: Regex = Regex::new(r"(\d+) (.+)$").unwrap();
}

fn to_command(input: &str) -> Command {
    if input.starts_with("$ cd ") {
        return Command::cd(input.strip_prefix("$ cd ").unwrap().to_string());
    }
    if input.starts_with("$ ls") {
        return Command::ls;
    }
    if input.starts_with("dir ") {
        return Command::dir(input.strip_prefix("dir ").unwrap().to_string());
    }
    if FILE_RE.is_match(input) {
        return FILE_RE.captures(input).and_then(|cap| {
            match (cap.get(1), cap.get(2)) {
                (Some(size), Some(name)) => Some(Command::file(
                    name.as_str().to_string(),
                    size.as_str().parse().unwrap())),
                _ => None
            }
        }).unwrap();
    }
    panic!("This should never be reached")
}

fn cd_to(now: &mut String, to: &str) {
    match to {
        ".." => now.replace_range(now.rfind('/').unwrap().., ""),
        "/" => now.push('/'),
        _ => {
            if now != "/" { now.push('/'); }
            now.push_str(to);
        }
    }
}

fn join_dir(curr_dir: &str, name: &str) -> String {
    let mut res = curr_dir.to_string();
    if curr_dir != "/" { res.push('/'); }
    res.push_str(name);
    res
}

#[derive(Debug)]
struct FileSizes {
    size_map: HashMap<String, u64>,
    // maps a dir to its children
    dir_map: HashMap<String, HashSet<String>>,
    dirs: HashSet<String>,
}

impl FileSizes {
    fn new() -> FileSizes {
        FileSizes {
            size_map: HashMap::new(),
            dir_map: HashMap::new(),
            dirs: HashSet::from(["/".to_string()]),
        }
    }

    fn add_file(&mut self, curr_dir: &str, name: String, size: u64) {
        let complete = join_dir(curr_dir, &name);
        self.size_map.insert(complete, size);
        self.dir_map.entry(curr_dir.to_string()).or_default().insert(name);
    }

    fn add_dir(&mut self, curr_dir: &str, name: String) {
        self.dirs.insert(join_dir(curr_dir, &name));
        self.dir_map.entry(curr_dir.to_string()).or_default().insert(name);
    }

    fn get_size(&mut self, name: &str) -> u64 {
        fn internal_gz(
            sizes: &mut HashMap<String, u64>,
            dirs: &HashMap<String, HashSet<String>>,
            name: &str,
        ) -> u64 {
            if sizes.contains_key(name) {
                return *sizes.get(name).unwrap();
            }
            let mut res = 0;
            for dir in dirs.get(name).unwrap() {
                res += internal_gz(sizes, dirs, &join_dir(name, dir));
            }
            sizes.insert(name.to_string(), res);
            res
        }
        internal_gz(&mut self.size_map, &self.dir_map, name)
    }
}

fn parse(input: &str) -> FileSizes {
    let mut current_dir = String::with_capacity(100);
    let mut files = FileSizes::new();
    input.lines()
        .map(to_command)
        .for_each(|cmd| {
            match cmd {
                Command::cd(dir) => cd_to(&mut current_dir, &dir),
                Command::file(name, size) => files.add_file(&current_dir, name, size),
                Command::dir(name) => files.add_dir(&current_dir, name),
                _ => {}
            }
        });
    // populate all directory sizes
    files.get_size("/");
    files
}

pub fn part1(input: &str) -> u64 {
    let files = parse(input);
    files.dirs.iter().map(|dir| *files.size_map.get(dir).unwrap())
        .filter(|size| *size <= 100_000)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let files = parse(input);
    let total = files.size_map.get("/").unwrap();
    let to_free = total - 40_000_000;
    files.dirs.iter().map(|dir| *files.size_map.get(dir).unwrap())
        .filter(|size| *size >= to_free)
        .min().unwrap()
}

#[test]
fn test() {
    crate::test_day!(7, 95437, 24933642)
}
