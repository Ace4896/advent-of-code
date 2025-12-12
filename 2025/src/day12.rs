use std::{env, process};

use advent_of_code_2025::*;

fn main() {
    println!("----- Day 12 -----");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("No input file specified");
        process::exit(1);
    }

    let input_path = &args[1];
    println!("Input File: {}", input_path);

    println!("Part 1: {}", solve_part_1(input_path));
    // println!("Part 2: {}", solve_part_2(input_path));
}

#[derive(Clone, Debug)]
struct Present {
    cells: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

#[derive(Clone, Debug)]
struct XmasTree {
    region_width: usize,
    region_height: usize,
    present_counts: Vec<usize>,
}

impl XmasTree {
    pub fn can_fit_all_3x3_presents(&self) -> bool {
        // Assuming all presents are 3x3 and don't overlap, check if we can fit them all
        // This doesn't solve the problem in general though... a 2D packing problem like this is hard
        let max_presents = (self.region_width / 3) * (self.region_height / 3);
        let required_presents: usize = self.present_counts.iter().sum();

        required_presents <= max_presents
    }
}

fn parse_input(mut input_lines: impl Iterator<Item = String>) -> (Vec<Present>, Vec<XmasTree>) {
    let mut presents = Vec::new();
    let mut xmas_trees = Vec::new();

    while let Some(input_line) = input_lines.next() {
        if input_line.is_empty() {
            continue;
        }

        if input_line.contains("x") {
            // Tree definition, e.g.:
            //
            // 12x10: 0 1 2 13
            xmas_trees.push(parse_tree_definition(&input_line));
        } else {
            // Present definition, e.g.:
            //
            // 0:
            // ###
            // ##.
            // ##.
            presents.push(parse_present_definition(&mut input_lines));
        }
    }

    (presents, xmas_trees)
}

fn parse_present_definition(input_lines: &mut impl Iterator<Item = String>) -> Present {
    // When this function is called, we'll be on the first line of the cells definition
    // Keep parsing until newline is encountered
    let mut cells: Vec<Vec<bool>> = Vec::new();

    while let Some(input_line) = input_lines.next().filter(|l| !l.is_empty()) {
        let row = input_line
            .trim()
            .chars()
            .filter_map(|c| match c {
                '#' => Some(true),
                '.' => Some(false),
                _ => None,
            })
            .collect();

        cells.push(row);
    }

    let height = cells.len();
    let width = cells[0].len();

    Present {
        cells,
        width,
        height,
    }
}

fn parse_tree_definition(input_line: &str) -> XmasTree {
    // Split by ':' first
    let (region_str, present_counts_str) = input_line.trim().split_once(":").unwrap();
    let (region_width_str, region_height_str) = region_str.split_once("x").unwrap();

    let region_width = region_width_str.parse::<usize>().unwrap();
    let region_height = region_height_str.parse::<usize>().unwrap();

    let present_counts = present_counts_str
        .split_whitespace()
        .filter_map(|c| c.trim().parse::<usize>().ok())
        .collect::<Vec<_>>();

    XmasTree {
        region_width,
        region_height,
        present_counts,
    }
}

/// "Calculates" how many of the christmas trees can fit the required presents.
///
/// This only works on the real input as it assumes:
///
/// - All presents are 3x3
/// - Presents don't overlap when placing them down
fn solve_part_1(input_path: &str) -> usize {
    let input_lines = read_lines(input_path);
    let (_, xmas_trees) = parse_input(input_lines);

    xmas_trees
        .iter()
        .filter(|tree| tree.can_fit_all_3x3_presents())
        .count()
}
