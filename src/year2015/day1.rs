use core::panic;


fn map_to_vals(input: &str) -> impl Iterator<Item=i64> + '_ {
	input.chars().map(|c| match c {
		'(' => 1,
		')' => -1,
		_ => panic!("There should only be '(' and ')' in the input")
	})
}

pub fn part1(input: &str) -> i64 {
	map_to_vals(input).sum()
}

pub fn part2(input: &str) -> i64 {
	let mut sum = 0;
	let mut i = 0;
	for v in map_to_vals(input) {
		sum += v;
		i += 1;
		if sum == -1 {
			return i
		}
	}
	panic!("It should never reach this point");
}

#[test]
fn test() {
	assert_eq!(part1("(())"), 0);
	assert_eq!(part1("()()"), 0);
	assert_eq!(part1("((("), 3);
	assert_eq!(part1("(()(()("), 3);
	assert_eq!(part1("))((((("), 3);
	assert_eq!(part1("())"), -1);
	assert_eq!(part1("))("), -1);
	assert_eq!(part1(")))"), -3);
	assert_eq!(part1(")())())"), -3);
	
	
	assert_eq!(part2(")"), 1);
	assert_eq!(part2("()())"), 5);
}