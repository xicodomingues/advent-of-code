enum Operation<'a> {
    Add(&'a str, u8),
    Remove(&'a str),
}

impl<'a> Operation<'a> {
    fn parse(input: &'a str) -> Operation<'a> {
        if input.contains('=') {
            let (id, lens) = input.split_once('=').unwrap();
            Self::Add(id, lens.parse().unwrap())
        } else {
            let id = input.trim_end_matches('-');
            Self::Remove(id)
        }
    }
}

struct Boxes<'a>(Vec<Vec<(&'a str, u8)>>);

impl<'a> Boxes<'a> {
    fn new() -> Self {
        Self(vec![vec![]; 256])
    }

    fn add(&mut self, id: &'a str, lens: u8) {
        let h = hash(id);
        let entry = self.0[h as usize].iter().position(|x| x.0 == id);
        match entry {
            Some(pos) => self.0[h as usize][pos] = (id, lens),
            None => self.0[h as usize].push((id, lens)),
        }
    }

    fn remove(&mut self, id: &'a str) {
        let h = hash(id);
        let entry = self.0[h as usize].iter().position(|x| x.0 == id);
        if let Some(pos) = entry {
            self.0[h as usize].remove(pos);
        }
    }

    fn focusing_power(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .map(|(i, box_)| {
                (i + 1)
                    * box_
                        .iter()
                        .enumerate()
                        .map(|(i, (_, lens))| (i + 1) * *lens as usize)
                        .sum::<usize>()
            })
            .sum()
    }
}

fn hash(input: &str) -> u8 {
    let mut hash = 0_u8;
    for x in input.bytes() {
        hash = hash.wrapping_add(x);
        hash = hash.wrapping_mul(17);
    }
    hash
}

#[test]
fn test_hash() {
    assert_eq!(hash("HASH"), 52);
    assert_eq!(hash("rn"), 0);
}

pub fn part1(input: &str) -> usize {
    input.trim().split(',').map(|x| hash(x) as usize).sum()
}

pub fn part2(input: &str) -> usize {
    let mut boxes = Boxes::new();

    input
        .trim()
        .split(',')
        .map(Operation::parse)
        .for_each(|op| match op {
            Operation::Add(id, x) => boxes.add(id, x),
            Operation::Remove(id) => boxes.remove(id),
        });

    boxes.focusing_power()
}

#[test]
fn test() {
    test_2023!(15, 1320, 145);
}
