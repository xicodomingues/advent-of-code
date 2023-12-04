use std::collections::{HashSet, HashMap};

use itertools::Itertools;

struct Mutation<'a> {
    source: char,
    mutation: &'a str,
}

impl Mutation<'_> {
    fn parse(s: &str) -> Mutation {
        let tmp: Vec<_> = s.trim().split(" => ").collect();
        if tmp[0].len() != 1 {
            panic!("More than one char in source");
        }
        Mutation {
            source: tmp[0].chars().next().unwrap(),
            mutation: tmp[1],
        }
    }
}

fn preprocess(input: &str) -> String {
    input
        .replace("Al", "A")
        .replace("Ca", "X")
        .replace("Mg", "M")
        .replace("Si", "S")
        .replace("Th", "T")
        .replace("Ti", "I")
        .replace("Rn", "(")
        .replace("Ar", ")")
        .replace('Y', ",")
}

fn parse(input: &str) -> (&str, Vec<Mutation>) {
    let mut lines = input.lines();
    let mutations: Vec<_> = lines
        .take_while_ref(|line| line.contains("=>"))
        .map(Mutation::parse)
        .collect();
    let molecule = lines.last().unwrap().trim();
    (molecule, mutations)
}

fn create_str(mol: &str, i: usize, mutation: &Mutation) -> String {
    let mut res = mol[..i].to_string();
    res.push_str(mutation.mutation);
    res.push_str(&mol[i + 1..]);
    res
}

fn do_one_replacement(mol: &str, mutation: &Mutation) -> HashSet<String> {
    let mut res = HashSet::new();
    for (i, c) in mol.char_indices() {
        if c == mutation.source {
            res.insert(create_str(mol, i, mutation));
        }
    }
    res
}

fn do_forward_replacement(mol: &str, muts: &[Mutation]) -> HashSet<String> {
    muts.iter()
        .flat_map(|m| do_one_replacement(mol, m))
        .collect()
}

fn go_back_to_one_char(mol: &str, _muts: &[Mutation], memo: &mut HashMap<String, HashSet<String>>) -> HashSet<String> {
	if mol.len() == 1 {
		let mut res = HashSet::new();
		res.insert(mol.to_string());
		memo.insert(mol.to_string(), res.clone());
		return res;
	}
	HashSet::new()
}

pub fn part1(input: &str) -> usize {
    let modified = preprocess(input);
    let (mol, muts) = parse(&modified);
    do_forward_replacement(mol, &muts).len()
}

pub fn part2(input: &str) -> usize {
    let modified = preprocess(input);
    let (_mol, _muts) = parse(&modified);
    0
}

#[test]
fn test() {
    crate::test_2015!(19, 7);
}
