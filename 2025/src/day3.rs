use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

fn main() {
    println!("----- Day 3 -----");

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

fn read_non_empty_lines(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).expect("Unable to open input file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|result| result.ok().filter(|line| !line.is_empty()))
}

/// Finds the largest possible joltage from each bank when only two batteries are enabled.
fn solve_part_1(input_path: &str) -> u32 {
    let input_lines = read_non_empty_lines(input_path);
    let mut total = 0;

    for input_line in input_lines {
        let digits = input_line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();

        let n = digits.len();

        // Find lowest index x of largest digit between [0, n-1]
        let (x, first_digit) = digits
            .iter()
            .enumerate()
            .take(n - 1)
            .min_by(|(idx1, digit1), (idx2, digit2)| {
                digit1.cmp(digit2).reverse().then(idx1.cmp(idx2))
            })
            .unwrap();

        // Find largest digit between [x+1, n]
        let second_digit = digits.iter().skip(x + 1).max().unwrap();

        let max_bank_joltage = (*first_digit * 10) + *second_digit;
        // println!(
        //     "{} -> {}{} ({})",
        //     input_line, *first_digit, *second_digit, max_bank_joltage
        // );

        total += max_bank_joltage;
    }

    total
}

/// Finds the largest possible joltage from each bank when 12 batteries are enabled.
fn solve_part_2(input_path: &str) -> u64 {
    let input_lines = read_non_empty_lines(input_path);
    let mut total = 0;

    const BATTERY_COUNT: usize = 12;

    for input_line in input_lines {
        let digits = input_line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect::<Vec<_>>();

        let n = digits.len();

        let mut battery_joltage = 0;
        let mut previous_idx: Option<usize> = None;

        for i in (0..BATTERY_COUNT).rev() {
            // If first selection, start at 0, otherwise prev + 1
            // Then leave enough space at the end for subsequent batteries
            let start_idx = previous_idx.map(|idx| idx + 1).unwrap_or(0);
            let end_idx = n - i;

            // Find lowest index of largest digit between [start, end]
            let (idx, digit) = digits
                .iter()
                .enumerate()
                .take(end_idx)
                .skip(start_idx)
                .min_by(|(idx1, digit1), (idx2, digit2)| {
                    digit1.cmp(digit2).reverse().then(idx1.cmp(idx2))
                })
                .unwrap();

            previous_idx = Some(idx);
            battery_joltage = (battery_joltage * 10) + *digit;
        }

        // println!(
        //     "{} -> {}",
        //     input_line, battery_joltage
        // );

        total += battery_joltage;
    }

    total
}
