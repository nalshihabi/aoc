pub use std::fs::File;
pub use std::io::prelude::*;
pub use std::io::BufReader;

pub fn read_file(file_name: &str) -> String {
    let file = File::open(file_name).expect(format!("Failed to open file {}", file_name).as_str());
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("Failed to read to string!");
    contents
}

pub fn read_lines(input: String) -> Vec<String> {
    let mut lines: Vec<String> = input.split("\n").map(|val| String::from(val)).collect();
    lines.pop();
    lines
}
