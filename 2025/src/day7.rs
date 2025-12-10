use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

fn main() {
    println!("----- Day 7 -----");

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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum GridCell {
    Start,
    Empty,
    Beam,
    Splitter,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct RowCol(usize, usize);

#[derive(Clone, Debug)]
struct Grid {
    data: Vec<Vec<GridCell>>,
    rows: usize,
    cols: usize,
}

impl TryFrom<char> for GridCell {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use GridCell::*;

        match value {
            'S' => Ok(Start),
            '.' => Ok(Empty),
            '|' => Ok(Beam),
            '^' => Ok(Splitter),
            _ => Err(()),
        }
    }
}

impl Grid {
    pub fn new(data: Vec<Vec<GridCell>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();

        Grid { data, rows, cols }
    }
}

fn read_lines(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).expect("Unable to open input file");
    let reader = BufReader::new(file);

    reader.lines().filter_map(|l| l.ok())
}

fn parse_grid(input_lines: &mut impl Iterator<Item = String>) -> Grid {
    let data = input_lines
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.chars()
                .filter_map(|c| GridCell::try_from(c).ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Grid::new(data)
}

fn generate_possible_beam_paths(grid: &mut Grid) -> usize {
    let mut split_counter = 0;

    for row_idx in 0..(grid.rows - 1) {
        for col_idx in 0..grid.cols {
            let cell = grid.data[row_idx][col_idx];
            match cell {
                GridCell::Start => {
                    // Add beam below the starting point
                    grid.data[row_idx + 1][col_idx] = GridCell::Beam;
                }
                GridCell::Beam => {
                    // If splitter is below this beam, add beams to left + right of splitter
                    // Otherwise, let beam pass through
                    if grid.data[row_idx + 1][col_idx] == GridCell::Splitter {
                        if col_idx > 0 {
                            grid.data[row_idx + 1][col_idx - 1] = GridCell::Beam;
                        }

                        if col_idx < grid.cols - 1 {
                            grid.data[row_idx + 1][col_idx + 1] = GridCell::Beam;
                        }

                        split_counter += 1;
                    } else {
                        grid.data[row_idx + 1][col_idx] = GridCell::Beam;
                    }
                }
                _ => {}
            }
        }
    }

    split_counter
}

/*
 * Part 2 took me a while to figure out...
 *
 * A naive approach of depth-first tree traversal is technically correct, but is far too slow.
 * Eventually, I came up with a memoized approach that uses the populated grid as the starting point.
 *
 * Simplified example:
 *
 * .......S.......
 * .......|.......     1.1: To Root
 * ......|^|......
 * ......|.|......     2.1: To 1.1;    2.2: To 1.1
 * .....|^|^|.....
 * .....|.|.|.....     3.1: To 2.1;    3.2: To 2.1, 2.2;   3.3: To 2.2
 * ....|^|^|^|....
 * ....|.|.|.|....     4.1: To 3.1;    4.2: To 3.1, 3.2;   4.3: To 3.2, 3.3;   4.4: To 3.3
 *
 * Beam 1.1:
 * - Connecting: None
 * - Path Count: 1 (default if there are no connecting)
 *
 * Beam 2.1:
 * - Connecting: 1.1
 * - Path Count: count(1.1) = 1
 *
 * Beam 2.2:
 * - Connecting: 1.1
 * - Path Count: count(1.1) = 1
 *
 * Beam 3.1:
 * - Connecting: 2.1
 * - Path Count: count(2.1) = 1
 *
 * Beam 3.2:
 * - Connecting: 2.1, 2.2
 * - Path Count: count(2.1) + count(2.2) = 1 + 1 = 2
 *
 * Beam 3.3:
 * - Connecting: 2.2
 * - Path Count: count(2.2) = 1
 *
 * Beam 4.1:
 * - Connecting: 3.1
 * - Path Count: count(3.1) = 1
 *
 * Beam 4.2:
 * - Connecting: 3.1, 3.2
 * - Path Count: count(3.1) + count(3.2) = 1 + 2 = 3
 *
 * Beam 4.3:
 * - Connecting: 3.2, 3.3
 * - Path Count: count(3.2) + count(3.3) = 2 + 1 = 3
 *
 * Beam 4.4:
 * - Connecting: 3.3
 * - Path Count: count(3.3) = 1
 *
 * To get the final answer, sum the counts for all beams at the bottom of the grid:
 * - count(4.1) + count(4.2) + count(4.3) + count(4.4) = 1 + 3 + 3 + 1 = 8
*/

/// Counts how many times the tachyon beam splits as it moves through the manifold.
fn solve_part_1(input_path: &str) -> usize {
    let mut input_lines = read_lines(input_path);
    let mut grid = parse_grid(&mut input_lines);

    generate_possible_beam_paths(&mut grid)
}

/// Counts how many possible paths the tachyon particle could take as it moves through the manifold.
fn solve_part_2(input_path: &str) -> usize {
    let mut input_lines = read_lines(input_path);
    let mut grid = parse_grid(&mut input_lines);

    generate_possible_beam_paths(&mut grid);

    // Find the last row of each beam in the grid
    let mut beam_end_indexes: Vec<RowCol> = Vec::new();

    // For beams in the middle of the grid, look for splitters with an incoming beam and record their position
    for row_idx in 1..grid.rows - 1 {
        for col_idx in 0..grid.cols {
            let cell = grid.data[row_idx][col_idx];
            let cell_above = grid.data[row_idx - 1][col_idx];

            if cell == GridCell::Splitter && cell_above == GridCell::Beam {
                beam_end_indexes.push(RowCol(row_idx, col_idx));
            }
        }
    }

    // Then add beams at the bottom of the grid
    let bottom_row_idx = grid.rows - 1;
    for col_idx in 0..grid.cols {
        let cell = grid.data[bottom_row_idx][col_idx];
        if cell != GridCell::Beam {
            continue;
        }

        beam_end_indexes.push(RowCol(bottom_row_idx, col_idx));
    }

    // Moving left-to-right, top-to-bottom, count how many ways we can reach the end of each beam
    let mut beam_path_counts: HashMap<RowCol, usize> =
        HashMap::with_capacity(beam_end_indexes.len());

    for RowCol(beam_row, beam_col) in beam_end_indexes.iter() {
        let mut connecting_beam_indexes: Vec<RowCol> = Vec::new();
        let mut row_idx = *beam_row - 1;

        // Traverse up the beam and note the ends of any connecting beams
        while grid.data[row_idx][*beam_col] == GridCell::Beam {
            if *beam_col > 0 {
                let left_pos = RowCol(row_idx, *beam_col - 1);
                if beam_end_indexes.contains(&left_pos) {
                    connecting_beam_indexes.push(left_pos);
                }
            }

            if *beam_col < grid.cols - 1 {
                let right_pos = RowCol(row_idx, *beam_col + 1);
                if beam_end_indexes.contains(&right_pos) {
                    connecting_beam_indexes.push(right_pos);
                }
            }

            row_idx -= 1;
        }

        // Calculate total ways in which connecting beams can be reached
        // If there are no connecting beams, default to 1
        let connecting_path_count: usize = connecting_beam_indexes
            .iter()
            .filter_map(|idx| beam_path_counts.get(idx))
            .sum();
        let beam_path_count = connecting_path_count.max(1);

        beam_path_counts.insert(RowCol(*beam_row, *beam_col), beam_path_count);
    }

    // Sum everything at the bottom of the grid
    beam_path_counts
        .iter()
        .filter(|(idx, _)| idx.0 == bottom_row_idx)
        .map(|(_, count)| *count)
        .sum()
}
