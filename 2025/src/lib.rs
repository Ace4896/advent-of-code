use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Helper for creating an iterator over the lines in a file.
pub fn read_lines(input_path: &str) -> impl Iterator<Item = String> {
    let file = File::open(input_path).expect("Unable to open input file");
    let reader = BufReader::new(file);

    reader.lines().filter_map(|l| l.ok())
}
