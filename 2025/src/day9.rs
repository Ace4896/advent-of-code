use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

use geo::{Coord, LineString, Polygon, PreparedGeometry, Rect, Relate};

fn main() {
    println!("----- Day 9 -----");

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

#[derive(Clone, Copy, Debug)]
struct Coordinate {
    x: u64,
    y: u64,
}

impl From<Coordinate> for Coord<f64> {
    fn from(value: Coordinate) -> Self {
        Coord {
            x: value.x as f64,
            y: value.y as f64,
        }
    }
}

impl From<&Coordinate> for Coord<f64> {
    fn from(value: &Coordinate) -> Self {
        Coord {
            x: value.x as f64,
            y: value.y as f64,
        }
    }
}

impl Coordinate {
    pub const fn rectangle_area(&self, other: &Coordinate) -> u64 {
        let width = self.x.abs_diff(other.x) + 1;
        let height = self.y.abs_diff(other.y) + 1;

        width * height
    }
}

fn read_lines(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).expect("Unable to open input file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.is_empty())
}

fn parse_input(input_lines: &mut impl Iterator<Item = String>) -> Vec<Coordinate> {
    input_lines
        .map(|l| {
            let mut coordinates = l.trim().split(",");
            let x = coordinates.next().unwrap().parse::<u64>().unwrap();
            let y = coordinates.next().unwrap().parse::<u64>().unwrap();

            Coordinate { x, y }
        })
        .collect()
}

/// Finds the largest rectangular area that can be made using two of the red tiles in the grid.
fn solve_part_1(input_path: &str) -> u64 {
    let mut input_lines = read_lines(input_path);
    let coordinates = parse_input(&mut input_lines);

    let mut largest_rectangle_area = 0;

    for i in 0..coordinates.len() {
        let coord1 = coordinates[i];

        for j in (i + 1)..coordinates.len() {
            let coord2 = coordinates[j];
            let rectangle_area = coord1.rectangle_area(&coord2);

            largest_rectangle_area = largest_rectangle_area.max(rectangle_area);
        }
    }

    largest_rectangle_area
}

/// Finds the largest rectangular area where:
/// - The 2 selected corners are red tiles
/// - Only red + green tiles are swapped
fn solve_part_2(input_path: &str) -> u64 {
    let mut input_lines = read_lines(input_path);
    let coordinates = parse_input(&mut input_lines);

    // Unfortunately, I couldn't figure out a way to do this without libraries...
    //
    // I had an idea about iterating over 4 adjacent coordinates at a time and looking for "U"
    // shapes, but wasn't able to get it working on the example input
    let polygon = Polygon::new(LineString::from_iter(coordinates.iter()), Vec::new());
    let prepared_polygon = PreparedGeometry::from(polygon);

    let mut largest_rectangle_area = 0;

    for i in 0..coordinates.len() {
        let coord1 = coordinates[i];

        for j in (i + 1)..coordinates.len() {
            let coord2 = coordinates[j];

            let rectangle = Rect::new(coord1, coord2);
            let intersection_matrix = prepared_polygon.relate(&rectangle);

            if intersection_matrix.is_covers() {
                let area = coord1.rectangle_area(&coord2);
                largest_rectangle_area = largest_rectangle_area.max(area);
            }
        }
    }

    largest_rectangle_area
}
