use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

fn main() {
    println!("----- Day 4 -----");

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

const ACCESSIBLE_THRESHOLD: usize = 4;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum CellType {
    Empty,
    PaperRoll,
}

impl TryFrom<char> for CellType {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use CellType::*;

        match value {
            '.' => Ok(Empty),
            '@' => Ok(PaperRoll),
            _ => Err("Invalid grid cell"),
        }
    }
}

/// 2D vector wrapper (to make on-the-fly grid modifications easier).
struct Vec2d<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> From<Vec<Vec<T>>> for Vec2d<T> {
    fn from(value: Vec<Vec<T>>) -> Self {
        if value.is_empty() {
            Vec2d {
                data: Vec::new(),
                rows: 0,
                cols: 0,
            }
        } else {
            let rows = value.len();
            let cols = value[0].len();

            Vec2d {
                data: value.into_iter().flatten().collect(),
                rows,
                cols,
            }
        }
    }
}

impl<T> Vec2d<T> {
    const fn calculate_idx(&self, row: usize, col: usize) -> usize {
        (self.rows * row) + col
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        let idx = self.calculate_idx(row, col);
        &self.data[idx]
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        let idx = self.calculate_idx(row, col);
        &mut self.data[idx]
    }
}

#[derive(Debug)]
struct GridCell {
    cell_type: CellType,
    occupied_above: u8,
    occupied_sides: u8,
    occupied_below: u8,
}

impl GridCell {
    pub const fn new(cell_type: CellType) -> Self {
        GridCell {
            cell_type,
            occupied_above: 0,
            occupied_sides: 0,
            occupied_below: 0,
        }
    }

    pub const fn is_accessible(&self) -> bool {
        self.occupied_above + self.occupied_sides + self.occupied_below < ACCESSIBLE_THRESHOLD as u8
    }
}

fn read_non_empty_lines(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).expect("Unable to open input file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|result| result.ok().filter(|line| !line.is_empty()))
}

/*
 * While working on part 1, I guessed that part 2 would involve finding all accessible paper rolls.
 * I was supposed to use GridCell in part 1, but then I got hit by the borrow checker haha.
 *
 * Eventually, I realised I could get around it by using a flattened representation of the grid.
 */

/// Finds number of paper rolls that have fewer than 4 adjacent paper rolls in the 8 surrounding cells.
fn solve_part_1(input_path: &str) -> u32 {
    let input_lines = read_non_empty_lines(input_path);

    // Parse the input grid
    let mut grid: Vec<Vec<CellType>> = Vec::new();

    for input_line in input_lines {
        let row = input_line
            .trim()
            .chars()
            .map(|c| CellType::try_from(c).unwrap())
            .collect::<Vec<_>>();

        grid.push(row);
    }

    let mut total_accessible = 0;

    for row_idx in 0..grid.len() {
        let current_row = &grid[row_idx];
        let row_above = if row_idx > 0 {
            Some(&grid[row_idx - 1])
        } else {
            None
        };

        let row_below = if row_idx < grid.len() - 1 {
            Some(&grid[row_idx + 1])
        } else {
            None
        };

        for col_idx in 0..current_row.len() {
            let current_cell = current_row[col_idx];
            if current_cell != CellType::PaperRoll {
                continue;
            }

            let mut adjacent_occupied = 0;

            // Count occupied cells to left
            if col_idx > 0 {
                if row_above.is_some_and(|r| r[col_idx - 1] == CellType::PaperRoll) {
                    adjacent_occupied += 1;
                }

                if current_row[col_idx - 1] == CellType::PaperRoll {
                    adjacent_occupied += 1;
                }

                if row_below.is_some_and(|r| r[col_idx - 1] == CellType::PaperRoll) {
                    adjacent_occupied += 1;
                }
            }

            // Count occupied cells above + below
            if row_above.is_some_and(|r| r[col_idx] == CellType::PaperRoll) {
                adjacent_occupied += 1;
            }

            if row_below.is_some_and(|r| r[col_idx] == CellType::PaperRoll) {
                adjacent_occupied += 1;
            }

            // Count occupied cells to right
            if col_idx < current_row.len() - 1 {
                if row_above.is_some_and(|r| r[col_idx + 1] == CellType::PaperRoll) {
                    adjacent_occupied += 1;
                }

                if current_row[col_idx + 1] == CellType::PaperRoll {
                    adjacent_occupied += 1;
                }

                if row_below.is_some_and(|r| r[col_idx + 1] == CellType::PaperRoll) {
                    adjacent_occupied += 1;
                }
            }

            if adjacent_occupied < ACCESSIBLE_THRESHOLD {
                total_accessible += 1;
            }
        }
    }

    total_accessible
}

/// Incrementally finds all accessible paper rolls.
fn solve_part_2(input_path: &str) -> usize {
    let input_lines = read_non_empty_lines(input_path);

    // Parse the input grid
    let mut grid: Vec<Vec<GridCell>> = Vec::new();

    for input_line in input_lines {
        let row = input_line
            .trim()
            .chars()
            .map(|c| CellType::try_from(c).unwrap())
            .map(GridCell::new)
            .collect::<Vec<_>>();

        grid.push(row);
    }

    let mut grid = Vec2d::from(grid);

    // Calculate initial paper roll adjacency counts
    for row_idx in 0..grid.rows {
        for col_idx in 0..grid.cols {
            let mut occupied_above = 0;
            let mut occupied_sides = 0;
            let mut occupied_below = 0;

            // Count cells in row above
            if row_idx > 0 {
                if col_idx > 0
                    && grid.get(row_idx - 1, col_idx - 1).cell_type == CellType::PaperRoll
                {
                    occupied_above += 1;
                }

                if grid.get(row_idx - 1, col_idx).cell_type == CellType::PaperRoll {
                    occupied_above += 1;
                }

                if col_idx < grid.cols - 1
                    && grid.get(row_idx - 1, col_idx + 1).cell_type == CellType::PaperRoll
                {
                    occupied_above += 1;
                }
            }

            // Count cells to sides
            if col_idx > 0 && grid.get(row_idx, col_idx - 1).cell_type == CellType::PaperRoll {
                occupied_sides += 1;
            }

            if col_idx < grid.cols - 1
                && grid.get(row_idx, col_idx + 1).cell_type == CellType::PaperRoll
            {
                occupied_sides += 1;
            }

            // Count cells in row below
            if row_idx < grid.rows - 1 {
                if col_idx > 0
                    && grid.get(row_idx + 1, col_idx - 1).cell_type == CellType::PaperRoll
                {
                    occupied_below += 1;
                }

                if grid.get(row_idx + 1, col_idx).cell_type == CellType::PaperRoll {
                    occupied_below += 1;
                }

                if col_idx < grid.cols - 1
                    && grid.get(row_idx + 1, col_idx + 1).cell_type == CellType::PaperRoll
                {
                    occupied_below += 1;
                }
            }

            let current_cell = grid.get_mut(row_idx, col_idx);
            current_cell.occupied_above = occupied_above;
            current_cell.occupied_sides = occupied_sides;
            current_cell.occupied_below = occupied_below;
        }
    }

    let mut total_accessible = 0;

    loop {
        // Determine which paper rolls are accessible
        let mut accessible_rolls: Vec<(usize, usize)> = Vec::new();

        for row_idx in 0..grid.rows {
            for col_idx in 0..grid.cols {
                let cell = grid.get(row_idx, col_idx);
                if cell.cell_type == CellType::PaperRoll && cell.is_accessible() {
                    accessible_rolls.push((row_idx, col_idx));
                }
            }
        }

        if accessible_rolls.is_empty() {
            break;
        }

        total_accessible += accessible_rolls.len();

        // Remove the accessible paper rolls from the grid
        for (row_idx, col_idx) in accessible_rolls.iter() {
            // Update cells above
            if *row_idx > 0 {
                if *col_idx > 0 {
                    let top_left_cell = grid.get_mut(*row_idx - 1, *col_idx - 1);
                    top_left_cell.occupied_below = top_left_cell.occupied_below.saturating_sub(1);
                }

                let top_mid_cell = grid.get_mut(*row_idx - 1, *col_idx);
                top_mid_cell.occupied_below = top_mid_cell.occupied_below.saturating_sub(1);

                if *col_idx < grid.cols - 1 {
                    let top_right_cell = grid.get_mut(*row_idx - 1, *col_idx + 1);
                    top_right_cell.occupied_below = top_right_cell.occupied_below.saturating_sub(1);
                }
            }

            // Update cells on left/right
            if *col_idx > 0 {
                let left_cell = grid.get_mut(*row_idx, *col_idx - 1);
                left_cell.occupied_sides = left_cell.occupied_sides.saturating_sub(1);
            }

            if *col_idx < grid.cols - 1 {
                let right_cell = grid.get_mut(*row_idx, *col_idx + 1);
                right_cell.occupied_sides = right_cell.occupied_sides.saturating_sub(1);
            }

            // Update cells below
            if *row_idx < grid.rows - 1 {
                if *col_idx > 0 {
                    let bot_left_cell = grid.get_mut(*row_idx + 1, *col_idx - 1);
                    bot_left_cell.occupied_above = bot_left_cell.occupied_above.saturating_sub(1);
                }

                let bot_mid_cell = grid.get_mut(*row_idx + 1, *col_idx);
                bot_mid_cell.occupied_above = bot_mid_cell.occupied_above.saturating_sub(1);

                if *col_idx < grid.cols - 1 {
                    let bot_right_cell = grid.get_mut(*row_idx + 1, *col_idx + 1);
                    bot_right_cell.occupied_above = bot_right_cell.occupied_above.saturating_sub(1);
                }
            }

            // Clear this cell
            grid.get_mut(*row_idx, *col_idx).cell_type = CellType::Empty;
        }
    }

    total_accessible
}
