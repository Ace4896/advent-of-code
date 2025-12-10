use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
    process,
};

fn main() {
    println!("----- Day 5 -----");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("No input file specified");
        process::exit(1);
    }

    let input_path = &args[1];
    println!("Input File: {}", input_path);

    println!("Part 1: {}", solve_part_1(input_path));
    println!("Part 2: {}", solve_part_2(input_path));
}

#[derive(Default)]
struct Database {
    // NOTE: For better efficiency, could index by the start of the range
    //       This would reduce how many entries we need to search through
    id_ranges: Vec<RangeInclusive<u64>>,
}

impl Database {
    pub fn add_id_range(&mut self, id_range: RangeInclusive<u64>) {
        // Remove any ranges that overlap with the input range
        let overlapping_ranges = self.id_ranges.extract_if(.., |current_range| {
            current_range.contains(id_range.end())
                || current_range.contains(id_range.end())
                || id_range.contains(current_range.start())
                || id_range.contains(current_range.end())
        });

        // Then merge with the input range
        let merged_range = overlapping_ranges.fold(id_range.clone(), |acc, x| {
            let merged_start = *acc.start().min(x.start());
            let merged_end = *acc.end().max(x.end());

            merged_start..=merged_end
        });

        self.id_ranges.push(merged_range);
    }

    pub fn contains_id(&self, id: u64) -> bool {
        self.id_ranges.iter().any(|id_range| id_range.contains(&id))
    }
}

fn read_lines(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).expect("Unable to open input file");
    let reader = BufReader::new(file);

    reader.lines().filter_map(|l| l.ok())
}

fn parse_input(input_path: &str) -> (Database, Vec<u64>) {
    let mut input_iter = read_lines(input_path);

    // Parse ID ranges up until 1st empty line
    let mut database = Database::default();

    while let Some(id_range_line) = input_iter.next() {
        if let Some((start_str, end_str)) = id_range_line.trim().split_once("-") {
            let start = start_str.parse::<u64>().unwrap();
            let end = end_str.parse::<u64>().unwrap();

            database.add_id_range(start..=end);
        } else {
            break;
        }
    }

    // Parse available IDs
    let available_ids = input_iter
        .filter_map(|l| l.trim().parse::<u64>().ok())
        .collect();

    (database, available_ids)
}

/// Finds how many IDs in the input list are fresh, i.e. are present in the database.
fn solve_part_1(input_path: &str) -> usize {
    let (database, available_ids) = parse_input(input_path);

    available_ids
        .iter()
        .filter(|id| database.contains_id(**id))
        .count()
}

/// Finds how many IDs in the database are fresh.
fn solve_part_2(input_path: &str) -> u64 {
    let (database, _) = parse_input(input_path);

    // Overlapping ID ranges are merged during parsing, so we can just sum the differences
    database
        .id_ranges
        .iter()
        .map(|r| *r.end() - *r.start() + 1)
        .sum()
}
