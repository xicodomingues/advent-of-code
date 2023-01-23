use std::fs;

pub fn load_file(filename: &str) -> String {
    fs::read_to_string("data/".to_string() + filename)
        .expect("Should have been able to read the file")
}

#[macro_export]
macro_rules! test_day {
    ( $day:literal, $first:literal, $second:literal) => {{
        use $crate::utils::load_file;
        let tmp = load_file(&format!("test{}.txt", $day));
        assert_eq!(part1(&tmp), $first);
        assert_eq!(part2(&tmp), $second);
    }};
}
