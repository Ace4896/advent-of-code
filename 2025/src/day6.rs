use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

fn main() {
    println!("----- Day 6 -----");

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

fn read_lines(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).expect("Unable to open input file");
    let reader = BufReader::new(file);

    reader.lines().filter_map(|l| l.ok())
}

/// Calculates the grand total for all problems in the worksheet.
fn solve_part_1(input_path: &str) -> u64 {
    let mut input_lines = read_lines(input_path);

    // Parse numbers that need to be added/multiplied
    let mut number_lines: Vec<Vec<u64>> = Vec::new();
    let mut grand_total = 0;

    while let Some(line) = input_lines.next() {
        let values = line.trim().split_whitespace().collect::<Vec<_>>();

        if let Ok(_) = values.first().unwrap().parse::<u64>() {
            // First value in the line is a number, so add to list of number lines
            let number_line = values
                .into_iter()
                .filter_map(|v| v.parse::<u64>().ok())
                .collect::<Vec<_>>();

            number_lines.push(number_line);
        } else {
            // First value is an operator, so start calculating grand total
            for (problem_idx, &op) in values.iter().enumerate() {
                let problem_nums = number_lines.iter().map(|l| l[problem_idx]);
                let intermediate_total = match op {
                    "+" => problem_nums.sum(),
                    "*" => problem_nums.fold(1, |acc, x| acc * x),
                    _ => panic!("Unexpected operator '{}'", op),
                };

                grand_total += intermediate_total;
            }

            break;
        }
    }

    grand_total
}

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Multiply,
}

impl TryFrom<char> for Operator {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Operator::*;

        match value {
            '+' => Ok(Add),
            '*' => Ok(Multiply),
            _ => Err(()),
        }
    }
}

impl Operator {
    pub fn calculate(&self, numbers: &[u64]) -> u64 {
        match self {
            Operator::Add => numbers.iter().sum(),
            Operator::Multiply => numbers.iter().cloned().reduce(|acc, x| acc * x).unwrap(),
        }
    }
}

/// Calculates the grand total for all problems in the worksheet using different place value interpretations.
fn solve_part_2(input_path: &str) -> u64 {
    let mut input_lines = read_lines(input_path);

    // Parse individual characters
    let mut digit_lines: Vec<Vec<Option<u8>>> = Vec::new();
    let mut operators: Vec<Operator> = Vec::with_capacity(0);

    while let Some(line) = input_lines.next() {
        // Determine what type of line to parse based on 1st non-empty character
        let first_char = line
            .chars()
            .skip_while(|c| c.is_whitespace())
            .next()
            .unwrap();

        if first_char.is_ascii_digit() {
            // Parse as a digit line
            let digit_line = line
                .trim_end()
                .chars()
                .map(|c| c.to_digit(10).map(|d| d as u8))
                .collect::<Vec<_>>();

            digit_lines.push(digit_line);
        } else {
            // Parse as an operator line
            operators = line
                .trim_end()
                .chars()
                .filter_map(|c| Operator::try_from(c).ok())
                .collect::<Vec<_>>();

            break;
        }
    }

    // .trim_end() was used to remove the newline, but this also removes normal spaces
    // So add blanks to the end of any digit list that's too short
    let expected_line_length = digit_lines.iter().map(|l| l.len()).max().unwrap();

    for digit_line in digit_lines
        .iter_mut()
        .filter(|l| l.len() < expected_line_length)
    {
        digit_line.resize(expected_line_length, Option::None);
    }

    // Loop through each digit location and either combine them or calculate the problem answer
    // Idea is that when all digits are missing, we're in-between two problems
    let mut grand_total = 0;
    let mut problem_idx = 0;
    let mut problem_numbers: Vec<u64> = Vec::new();

    for digit_idx in 0..expected_line_length {
        // Attempt to combine the digits at this index
        let problem_number = digit_lines
            .iter()
            .map(|l| l[digit_idx])
            .filter_map(|d| d)
            .map(|d| d as u64)
            .reduce(|acc, d| (acc * 10) + d);

        if let Some(number) = problem_number {
            problem_numbers.push(number)
        } else {
            // No digits in this location, so calculate answer for this problem
            let operator = operators[problem_idx];
            let problem_answer = operator.calculate(&problem_numbers);

            grand_total += problem_answer;
            problem_idx += 1;
            problem_numbers.clear();
        }
    }

    // Handle last calculation if required
    if problem_idx < operators.len() && !problem_numbers.is_empty() {
        let operator = operators[problem_idx];
        let problem_answer = operator.calculate(&problem_numbers);

        grand_total += problem_answer;
    }

    grand_total
}
