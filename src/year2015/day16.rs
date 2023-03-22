use std::str::FromStr;

use crate::utils::ParseError;

#[derive(Debug)]
struct Sue {
    id: u32,
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

const CORRECT_SUE: Sue = Sue {
    id: 0,
    children: Some(3),
    cats: Some(7),
    samoyeds: Some(2),
    pomeranians: Some(3),
    akitas: Some(0),
    vizslas: Some(0),
    goldfish: Some(5),
    trees: Some(3),
    cars: Some(2),
    perfumes: Some(1),
};

macro_rules! has_same_prop {
    ($res:ident, $obj:ident, $other:ident, $prop:ident) => {
        if let Some(prop) = $other.$prop {
            $res = $res && match $obj.$prop {
                Some(c) => c == prop,
                None => $res,
            }
        }
    };
}

macro_rules! has_higher_prop {
    ($res:ident, $obj:ident, $other:ident, $prop:ident) => {
        if let Some(prop) = $other.$prop {
            $res = $res && match $obj.$prop {
                Some(c) => c > prop,
                None => $res,
            }
        }
    };
}

macro_rules! has_lower_prop {
    ($res:ident, $obj:ident, $other:ident, $prop:ident) => {
        if let Some(prop) = $other.$prop {
            $res = $res && match $obj.$prop {
                Some(c) => c < prop,
                None => $res,
            }
        }
    };
}

impl Sue {
    fn new(id: u32) -> Sue {
        Sue {
            id,
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None,
        }
    }
    
    fn set_prop(&mut self, name: &str, val: u32) {
        match name {
            "children" => self.children = Some(val),
            "cats" => self.cats = Some(val),
            "samoyeds" => self.samoyeds = Some(val),
            "pomeranians" => self.pomeranians = Some(val),
            "akitas" => self.akitas = Some(val),
            "vizslas" => self.vizslas = Some(val),
            "goldfish" => self.goldfish = Some(val),
            "trees" => self.trees = Some(val),
            "cars" => self.cars = Some(val),
            "perfumes" => self.perfumes = Some(val),
            _ => panic!("There is no field named {}", name)
        }
    }
    
    fn is_match_part1(&self, other: &Sue) -> bool {
        let mut res = true;
        has_same_prop!(res, self, other, children);
        has_same_prop!(res, self, other, cats);
        has_same_prop!(res, self, other, samoyeds);
        has_same_prop!(res, self, other, pomeranians);
        has_same_prop!(res, self, other, akitas);
        has_same_prop!(res, self, other, vizslas);
        has_same_prop!(res, self, other, goldfish);
        has_same_prop!(res, self, other, trees);
        has_same_prop!(res, self, other, cars);
        has_same_prop!(res, self, other, perfumes);
        res
    }
    
    fn is_match_part2(&self, other: &Sue) -> bool {
        let mut res = true;
        has_same_prop!(res, self, other, children);
        has_lower_prop!(res, self, other, cats);
        has_same_prop!(res, self, other, samoyeds);
        has_higher_prop!(res, self, other, pomeranians);
        has_same_prop!(res, self, other, akitas);
        has_same_prop!(res, self, other, vizslas);
        has_higher_prop!(res, self, other, goldfish);
        has_lower_prop!(res, self, other, trees);
        has_same_prop!(res, self, other, cars);
        has_same_prop!(res, self, other, perfumes);
        res
    }
}

impl FromStr for Sue {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let helper = line.replace(",", "").replace(":", "");
        let seq = helper.split_whitespace().collect::<Vec<_>>();
        let mut sue = Sue::new(seq[1].parse()?);
        sue.set_prop(seq[2], seq[3].parse()?);
        sue.set_prop(seq[4], seq[5].parse()?);
        sue.set_prop(seq[6], seq[7].parse()?);
        Ok(sue)
    }
}

#[test]
fn test_same_sue() {
    let mut other = Sue::new(12);
    
    other.set_prop("cats", 7);
    assert_eq!(CORRECT_SUE.is_match_part1(&other), true);
    
    other.set_prop("goldfish", 5);
    assert_eq!(CORRECT_SUE.is_match_part1(&other), true);
    
    other.set_prop("vizslas", 1);
    assert_eq!(CORRECT_SUE.is_match_part1(&other), false);
    
    other.set_prop("vizslas", 0);
    assert_eq!(CORRECT_SUE.is_match_part1(&other), true);
}

fn parse(input: &str) -> impl Iterator<Item = Sue> + '_ {
    input.lines().map(|line| Sue::from_str(line).unwrap())
}

fn solve(input: &str, match_func: fn(&Sue, &Sue) -> bool) -> u32 {
    parse(input).filter(|s| match_func(&CORRECT_SUE, s)).map(|s| s.id).next().unwrap()
}

pub fn part1(input: &str) -> u32 {
    solve(input, Sue::is_match_part1)
}

pub fn part2(input: &str) -> u32 {
    solve(input, Sue::is_match_part2)
}
