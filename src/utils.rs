use std::fs;

pub fn load_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string("data/".to_string() + filename)
        .expect("Should have been able to read the file");
    contents.lines().map(|s| s.to_string()).collect()
}