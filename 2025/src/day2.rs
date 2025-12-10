use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

fn main() {
    println!("----- Day 2 -----");

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

fn read_file_contents(path: &str) -> String {
    let file = File::open(path).expect("Unable to open input file");
    let mut input_reader = BufReader::new(file);

    let mut input_str = String::new();
    input_reader
        .read_line(&mut input_str)
        .expect("No input lines found");

    input_str
}

fn parse_id_ranges<'a>(input: &'a str) -> impl Iterator<Item = (&'a str, &'a str)> {
    input
        .trim()
        .split(",")
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.split_once("-"))
}

/// Determines which IDs contain 2 repeated halves, e.g. 113113, then sums them together.
fn solve_part_1(input_path: &str) -> u64 {
    let input = read_file_contents(input_path);
    let id_ranges_iter = parse_id_ranges(&input);

    let mut total = 0;

    for (range_start_str, range_end_str) in id_ranges_iter {
        // println!("{}-{}", range_start_str, range_end_str);

        // Determine where to start looking for invalid IDs
        let start_digits = range_start_str.chars().count();
        let start_middle_digit = start_digits / 2;

        let invalid_start = if start_digits % 2 == 0 {
            // Even Case: Split at middle digit
            // - If left >= right, start at left, e.g. 1402 => start at 14
            // - Otherwise, start at left + 1, e.g. 1425 => start at 15
            let (left, right) = range_start_str.split_at(start_middle_digit);
            let left_parsed = left.parse::<u32>().unwrap();

            if left >= right {
                left_parsed
            } else {
                left_parsed + 1
            }
        } else {
            // Odd Case: Start at 10 ^ (middle digit index)
            // e.g. 12345 => start at 100
            10_u32.pow(start_middle_digit as u32)
        };

        // Determine where to stop looking for invalid IDs
        let end_digits = range_end_str.chars().count();
        let end_middle_digit = end_digits / 2;

        let invalid_end = if end_digits % 2 == 0 {
            // Even Case: Split at middle digit
            // - If left <= right, end at left, e.g. 1470 => end at 14
            // - Otherwise, end at left - 1,    e.g. 1402 => end at 13
            let (left, right) = range_end_str.split_at(end_middle_digit);
            let left_parsed = left.parse::<u32>().unwrap();

            if left <= right {
                left_parsed
            } else {
                left_parsed - 1
            }
        } else {
            // Odd Case: Start at 10 ^ (middle digit index) - 1
            // e.g. 12345 => end at 99
            10_u32.pow(end_middle_digit as u32) - 1
        };

        for i in invalid_start..=invalid_end {
            // NOTE: Can definitely do this without string parsing, too lazy LOL
            let invalid_id = format!("{}{}", i, i).parse::<u64>().unwrap();
            // println!("- Invalid: {}", invalid_id);

            total += invalid_id;
        }
    }

    total
}

/// More generalised version of part 1, where it looks at repeating chunks of different sizes.
fn solve_part_2(input_path: &str) -> u64 {
    let input = read_file_contents(input_path);
    let id_ranges_iter = parse_id_ranges(&input);

    // Keep track of unique IDs, as duplicates can be encountered
    // e.g. 2222 is encountered when chunk counts are:
    // - 4: 2, 2, 2, 2
    // - 2: 22, 22
    let mut invalid_ids: HashSet<u64> = HashSet::new();

    /*
     * Looking back at my solution, I think it became complicated because I iterated over chunk
     * count instead of chunk size. But it worked in the end, so ¯\_(ツ)_/¯
     */
    for (range_start_str, range_end_str) in id_ranges_iter {
        let range_start = range_start_str.parse::<u64>().unwrap();
        let range_end = range_end_str.parse::<u64>().unwrap();

        // println!("{}-{}", range_start_str, range_end_str);
        let max_chunk_count = std::cmp::max(range_start_str.len(), range_end_str.len());

        for chunk_count in 2..=max_chunk_count {
            // println!("- Chunk Count: {}", chunk_count);

            let invalid_start = get_starting_number(range_start_str, chunk_count);
            let invalid_end = get_ending_number(range_end_str, chunk_count);

            for i in invalid_start..=invalid_end {
                let invalid_id_str = i.to_string().repeat(chunk_count);
                let invalid_id = invalid_id_str.parse::<u64>().unwrap();

                if invalid_id >= range_start && invalid_id <= range_end {
                    // println!("  - Invalid: {}", invalid_id);
                    invalid_ids.insert(invalid_id);
                }
            }
        }
    }

    invalid_ids.iter().sum()
}

/// More generalised way of getting the starting number to check.
fn get_starting_number(range_start: &str, chunk_count: usize) -> u32 {
    /*
     * Examples:
     * - 12345678, Chunk Count 2
     *   - Chunk Cutoff: Up to 4th Digit => 1234
     *   - Chunks are split exactly
     *     - 1st Chunk = 1234, 2nd Chunk = 5678
     *     - 1st Chunk <= 2nd Chunk, so start at 1234 + 1
     * - 12345678, Chunk Count 3
     *   - Chunk Cutoff: Up to 2nd Digit => 12
     *   - Chunks aren't split exactly
     *     - Start at 10 ^ Cutoff => 10^2 = 100
     */
    let digit_count = range_start.chars().count();
    let cutoff_idx = digit_count / chunk_count;

    if digit_count % chunk_count == 0 {
        // Exact Split Case
        // - If 1st chunk >= 2nd chunk, start at 1st chunk
        // - Otherwise, start at 1st chunk + 1
        let chunk_1 = &range_start[0..cutoff_idx];
        let chunk_2 = &range_start[cutoff_idx..(cutoff_idx * 2)];

        let chunk_1_parsed = chunk_1.parse::<u32>().unwrap();
        if chunk_1 >= chunk_2 {
            chunk_1_parsed
        } else {
            chunk_1_parsed + 1
        }
    } else {
        // Non-Exact Split Case
        // Start at 10 ^ Cutoff
        10_u32.pow(cutoff_idx as u32)
    }
}

/// More generalised way of getting the ending number to check.
fn get_ending_number(range_end: &str, chunk_count: usize) -> u32 {
    /*
     * Examples:
     * - 12345678, Chunk Count 2
     *   - Chunk Cutoff: Up to 4th Digit => 1234
     *   - Chunks are split exactly
     *     - 1st Chunk = 1234, 2nd Chunk = 5678
     *     - 1st Chunk <= 2nd Chunk, so end at 1234
     * - 12345678, Chunk Count 3
     *   - Chunk Cutoff: Up to 2nd Digit => 12
     *   - Chunks aren't split exactly
     *     - End at 10 ^ Cutoff - 1 => 10^2 - 1 = 99
     */
    let digit_count = range_end.chars().count();
    let cutoff_idx = digit_count / chunk_count;

    if digit_count % chunk_count == 0 {
        // Exact Split Case
        // - If 1st chunk <= 2nd chunk, end at 1st chunk
        // - Otherwise, end at 1st chunk - 1
        let chunk_1 = &range_end[0..cutoff_idx];
        let chunk_2 = &range_end[cutoff_idx..(cutoff_idx * 2)];

        let chunk_1_parsed = chunk_1.parse::<u32>().unwrap();
        if chunk_1 <= chunk_2 {
            chunk_1_parsed
        } else {
            chunk_1_parsed - 1
        }
    } else {
        // Non-Exact Split Case
        // End at 10 ^ Cutoff - 1
        10_u32.pow(cutoff_idx as u32) - 1
    }
}
