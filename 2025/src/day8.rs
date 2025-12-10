use std::{
    cmp::Reverse,
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

fn main() {
    println!("----- Day 8 -----");

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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coordinate {
    x: u32,
    y: u32,
    z: u32,
}

impl Coordinate {
    // Calculating square root is slow and not necessary for direct comparisons, so I left it in squared form
    pub const fn euclidean_distance_squared(&self, other: &Coordinate) -> u64 {
        let x_diff = self.x.abs_diff(other.x) as u64;
        let y_diff = self.y.abs_diff(other.y) as u64;
        let z_diff = self.z.abs_diff(other.z) as u64;

        (x_diff * x_diff) + (y_diff * y_diff) + (z_diff * z_diff)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Edge {
    start: Coordinate,
    end: Coordinate,
    euclid_dist_sq: u64,
}

impl Edge {
    pub const fn new(start: Coordinate, end: Coordinate) -> Self {
        let euclid_dist_sq = start.euclidean_distance_squared(&end);

        Edge {
            start,
            end,
            euclid_dist_sq,
        }
    }
}

#[derive(Clone, Debug)]
struct Circuit {
    junction_boxes: HashSet<Coordinate>,
    edges: HashSet<Edge>,
}

impl Circuit {
    pub fn from_junction_box(junction_box: Coordinate) -> Self {
        let mut junction_boxes = HashSet::new();
        junction_boxes.insert(junction_box);

        Circuit {
            junction_boxes,
            edges: HashSet::new(),
        }
    }

    pub fn from_edge(edge: Edge) -> Self {
        let mut junction_boxes = HashSet::new();
        junction_boxes.insert(edge.start);
        junction_boxes.insert(edge.end);

        let mut edges = HashSet::new();
        edges.insert(edge);

        Circuit {
            junction_boxes,
            edges,
        }
    }

    pub fn merge(&mut self, other: &Circuit) {
        self.junction_boxes.extend(&other.junction_boxes);
        self.edges.extend(&other.edges);
    }
}

fn read_lines(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).expect("Unable to open input file");
    let reader = BufReader::new(file);

    reader.lines().filter_map(|l| l.ok())
}

fn parse_input(input_lines: &mut impl Iterator<Item = String>) -> Vec<Coordinate> {
    input_lines
        .map(|l| {
            let mut coordinates = l.trim().split(",");
            let x = coordinates.next().unwrap().parse::<u32>().unwrap();
            let y = coordinates.next().unwrap().parse::<u32>().unwrap();
            let z = coordinates.next().unwrap().parse::<u32>().unwrap();

            Coordinate { x, y, z }
        })
        .collect()
}

fn generate_all_edges(junction_box_coords: &[Coordinate]) -> Vec<Edge> {
    let mut edges = Vec::new();

    for i in 0..junction_box_coords.len() {
        let coord1 = junction_box_coords[i];

        for j in (i + 1)..junction_box_coords.len() {
            let coord2 = junction_box_coords[j];
            edges.push(Edge::new(coord1, coord2));
        }
    }

    edges
}

fn add_edge(circuits: &mut Vec<Circuit>, edge: Edge) {
    // Remove circuits with matching start + end coordinate
    let existing_circuits = circuits.extract_if(.., |c| {
        c.junction_boxes.contains(&edge.start) || c.junction_boxes.contains(&edge.end)
    });

    // Then merge with new circuit
    let merged_circuit = existing_circuits.fold(Circuit::from_edge(edge), |mut acc, c| {
        acc.merge(&c);
        acc
    });

    circuits.push(merged_circuit);
}

/// Finds the three largest circuits when connecting junction boxes with the shortest edges only,
/// then calculates the product of the circuit sizes.
fn solve_part_1(input_path: &str) -> usize {
    let mut input_lines = read_lines(input_path);
    let junction_box_coords = parse_input(&mut input_lines);

    // Calculate all unique edges and their euclidean distances
    // Then sort by increasing distance
    let mut edges = generate_all_edges(&junction_box_coords);
    edges.sort_by_key(|f| f.euclid_dist_sq);

    // Setup initial circuits where only a single junction box is present
    let mut circuits: Vec<Circuit> = junction_box_coords
        .iter()
        .map(|c| Circuit::from_junction_box(*c))
        .collect();

    // Then add all edges for required number of pairs
    // Strangely, the example wants '10' (count / 2), but main input wants '1000' (count)
    if input_path.contains("example") {
        edges.truncate(10);
    } else {
        edges.truncate(1000);
    }

    for edge in edges {
        add_edge(&mut circuits, edge);
    }

    // Find the top 3 circuit sizes
    const TOP_CIRCUIT_COUNT: usize = 3;
    let mut circuit_sizes = circuits
        .iter()
        .map(|c| c.junction_boxes.len())
        .collect::<Vec<_>>();

    circuit_sizes.sort_by_key(|s| Reverse(*s));
    circuit_sizes.truncate(TOP_CIRCUIT_COUNT);

    circuit_sizes
        .iter()
        .cloned()
        .reduce(|acc, s| acc * s)
        .unwrap()
}

/// Continuously adds edges until all circuits have at least 1 edge, then multiplies the X
/// coordinates of the junction boxes in the final edge that was added.
fn solve_part_2(input_path: &str) -> u64 {
    let mut input_lines = read_lines(input_path);
    let junction_box_coords = parse_input(&mut input_lines);

    // Calculate all unique edges and their euclidean distances
    // Then sort by increasing distance
    let mut edges = generate_all_edges(&junction_box_coords);
    edges.sort_by_key(|f| f.euclid_dist_sq);

    // Setup initial circuits where only a single junction box is present
    let mut circuits: Vec<Circuit> = junction_box_coords
        .iter()
        .map(|c| Circuit::from_junction_box(*c))
        .collect();

    for edge in edges {
        add_edge(&mut circuits, edge);

        if circuits.iter().all(|c| !c.edges.is_empty()) {
            return edge.start.x as u64 * edge.end.x as u64;
        }
    }

    0
}
