use std::fs;

pub fn load_file(filename: &str) -> String {
    fs::read_to_string("data/".to_string() + filename)
        .expect("Should have been able to read the file")
}
