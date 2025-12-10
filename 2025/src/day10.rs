use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

use advent_of_code_2025::*;

fn main() {
    println!("----- Day 10 -----");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("No input file specified");
        process::exit(1);
    }

    let input_path = &args[1];
    println!("Input File: {}", input_path);

    // println!("Part 1: {}", solve_part_1(input_path));
    // println!("Part 2: {}", solve_part_2(input_path));
}
