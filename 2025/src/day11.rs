use std::{collections::HashMap, env, process};

use advent_of_code_2025::*;

fn main() {
    println!("----- Day 11 -----");

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

fn parse_input(mut input_lines: impl Iterator<Item = String>) -> HashMap<String, Vec<String>> {
    let mut adjacency_list = HashMap::new();
    while let Some(line) = input_lines.next().filter(|l| !l.is_empty()) {
        let (node_str, connecting_str) = line.split_once(":").unwrap();

        let node = node_str.trim().to_string();
        let connected_nodes = connecting_str
            .split_whitespace()
            .map(|n| n.trim().to_string())
            .collect::<Vec<_>>();

        adjacency_list.insert(node, connected_nodes);
    }

    adjacency_list
}

/// Counts how many paths there are from 'you' to 'out'.
fn solve_part_1(input_path: &str) -> u32 {
    const START_NODE: &'static str = "you";
    const END_NODE: &'static str = "out";

    let input_lines = read_lines(input_path);
    let adjacency_list = parse_input(input_lines);

    // Perform BFS (without deduplication) and track how many times we reach 'out'
    let mut next_nodes: Vec<&str> = vec![&START_NODE];
    let mut path_count = 0;

    while !next_nodes.is_empty() {
        let nodes = next_nodes.drain(..).collect::<Vec<_>>();

        for node in nodes {
            if let Some(connecting_nodes) = adjacency_list.get(node) {
                for connecting_node in connecting_nodes {
                    if connecting_node == END_NODE {
                        path_count += 1;
                    } else {
                        next_nodes.push(connecting_node.as_str());
                    }
                }
            }
        }
    }

    path_count
}
