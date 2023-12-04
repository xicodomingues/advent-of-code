use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NUMBER_RE: Regex = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
    static ref NUMBER_RE_REV: Regex = Regex::new(r"\d|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno").unwrap();
}

fn get_calibration(line: &str) -> u32 {
    let pos1 = line.find(|c: char| c.is_ascii_digit()).unwrap();
    let mut res = line.chars().nth(pos1).unwrap().to_digit(10).unwrap() * 10;
    
    let pos2 = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
    res += line.chars().nth(pos2).unwrap().to_digit(10).unwrap();
    res
}


fn to_number(nbrstr: &str) -> u32 {
    let first_char = nbrstr.chars().next().unwrap();
    if first_char.is_ascii_digit() {
        return first_char.to_digit(10).unwrap();
    }
    match nbrstr {
        "one" | "eno" => 1,
        "two" | "owt" => 2,
        "three" | "eerht" => 3,
        "four" | "ruof" => 4,
        "five" | "evif"=> 5,
        "six" | "xis" => 6,
        "seven" | "neves" => 7,
        "eight" | "thgie" => 8,
        "nine" | "enin" => 9,
        _ => panic!("Invalid number string")
    }
}

fn get_calibration_part2(line: &str) -> u32 {
    let mut res = to_number(NUMBER_RE.find(line).unwrap().as_str()) * 10;
    let line_rev: String = line.chars().rev().collect();
    res += to_number(NUMBER_RE_REV.find(&line_rev).unwrap().as_str());
    res
}

pub fn part1(input: &str) -> u32 {
    input.lines()
    .map(get_calibration)
    .sum()
}

pub fn part2(input: &str) -> u32 {
    input.lines()
    .map(get_calibration_part2)
    .sum()
}

#[test]
fn test() {
    use indoc::indoc;

    assert_eq!(142, part1(indoc!{"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "}));
    
    assert_eq!(281 + 58, part2(indoc!{"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
        asdfiveightasd
    "}));
}
