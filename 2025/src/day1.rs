use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

fn main() {
    println!("----- Day 1 -----");

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

/// Counts how many times the dial lands on 0.
fn solve_part_1(input_filepath: &str) -> i32 {
    let input_file = File::open(input_filepath).expect("Unable to open input file");
    let input_reader = BufReader::new(input_file);

    let mut position = 50;
    let mut zeroes = 0;

    for input_line in input_reader
        .lines()
        .map_while(|line| line.ok().filter(|x| !x.is_empty()))
    {
        let (direction, distance_str) = input_line.split_at(1);
        let mut distance = distance_str.parse::<i32>().expect(&format!(
            "Unable to parse distance value '{}'",
            distance_str
        ));

        // Ignore extra revolutions
        distance %= 100;

        if distance <= 0 {
            // Nothing to do
            continue;
        }

        // Handle remaining distance within range - guaranteed to be in range [1, 99]
        match direction {
            "L" => {
                position -= distance;
                if position < 0 {
                    position += 100;
                }
            }
            "R" => {
                position += distance;
                if position > 99 {
                    position -= 100;
                }
            }
            _ => panic!("Unexpected direction {}", direction),
        }

        if position == 0 {
            zeroes += 1;
        }
    }

    zeroes
}

/// Counts the number of times the dial passes or lands on 0.
fn solve_part_2(input_filepath: &str) -> i32 {
    let input_file = File::open(input_filepath).expect("Unable to open input file");
    let input_reader = BufReader::new(input_file);

    /*
       While the problem was easy to understand, I had trouble with two edge cases LOL

       L39 -> Position: 0, Zeroes: 6466
       L17 -> Position: 83, Zeroes: 6467 <- shouldn't increment here as it never transitioned to 0
       ...
       L8 -> Position: 95, Zeroes: 6452
       L95 -> Position: 0, Zeroes: 6452 <- should've incremented here as it landed on 0
    */
    let mut position = 50;
    let mut zeroes = 0;

    for input_line in input_reader
        .lines()
        .map_while(|line| line.ok().filter(|x| !x.is_empty()))
    {
        let (direction, distance_str) = input_line.split_at(1);
        let mut distance = distance_str.parse::<i32>().expect(&format!(
            "Unable to parse distance value '{}'",
            distance_str
        ));

        // Count extra revolutions that are guaranteed to go past 0
        zeroes += distance / 100;
        distance %= 100;

        if distance <= 0 {
            continue;
        }

        // Handle remaining distance within range - guaranteed to be in range [1, 99]
        let original_position = position;
        match direction {
            "L" => {
                position -= distance;

                if position == 0 {
                    // Dial landed on 0
                    zeroes += 1;
                } else if position < 0 {
                    position += 100;

                    // If we didn't start from 0, means that dial went past it
                    if original_position != 0 {
                        zeroes += 1;
                    }
                }
            }
            "R" => {
                position += distance;

                // Check if dial wrapped back around
                if position > 99 {
                    position -= 100;
                    zeroes += 1;
                }
            }
            _ => panic!("Unexpected direction {}", direction),
        }

        // println!(
        //     "{:>2} + {:<4} -> Position: {:>2}, Zeroes: {}",
        //     original_position, &input_line, position, zeroes
        // );
    }

    zeroes
}
