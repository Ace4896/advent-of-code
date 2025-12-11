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
    println!("Part 2: {}", solve_part_2(input_path));
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

fn count_paths(
    start_node: &str,
    end_node: &str,
    adjacency_list: &HashMap<String, Vec<String>>,
) -> u32 {
    // Invert the graph, filtering to nodes that are relevant to search
    let mut unexplored_nodes: Vec<&str> = vec![start_node];
    let mut inverted_adjacency_list: HashMap<&str, Vec<&str>> = HashMap::new();

    while !unexplored_nodes.is_empty() {
        let nodes = unexplored_nodes.drain(..).collect::<Vec<_>>();

        for node in nodes {
            if let Some(connecting_nodes) = adjacency_list.get(node) {
                for connecting_node in connecting_nodes.iter().map(|n| n.as_str()) {
                    if let Some(parent_nodes) = inverted_adjacency_list.get_mut(connecting_node) {
                        parent_nodes.push(node);
                    } else {
                        inverted_adjacency_list.insert(connecting_node, vec![node]);

                        if connecting_node != end_node {
                            unexplored_nodes.push(connecting_node);
                        }
                    }
                }
            }
        }
    }

    // Then recursively count the total incoming paths leading to the end node
    count_paths_inner(
        start_node,
        end_node,
        &inverted_adjacency_list,
        &mut HashMap::with_capacity(inverted_adjacency_list.len()),
    )
}

fn count_paths_inner<'a>(
    root_node: &'a str,
    node: &'a str,
    inverted_adjacency_list: &HashMap<&'a str, Vec<&'a str>>,
    memoized_counts: &mut HashMap<&'a str, u32>,
) -> u32 {
    if node == root_node {
        // Base case (to allow recursion to complete)
        1
    } else if let Some(count) = memoized_counts.get(&node) {
        // Already calculated for this node
        *count
    } else {
        if let Some(parent_nodes) = inverted_adjacency_list.get(&node) {
            // Sum the total from each parent node and cache value
            let mut total = 0;

            for parent_node in parent_nodes {
                total += count_paths_inner(
                    root_node,
                    parent_node,
                    inverted_adjacency_list,
                    memoized_counts,
                );
            }

            memoized_counts.insert(node, total);
            total
        } else {
            0
        }
    }
}

/// Counts how many paths there are from 'you' to 'out'.
fn solve_part_1(input_path: &str) -> u32 {
    const START_NODE: &'static str = "you";
    const END_NODE: &'static str = "out";

    let input_lines = read_lines(input_path);
    let adjacency_list = parse_input(input_lines);

    count_paths(START_NODE, END_NODE, &adjacency_list)
}

/// Counts how many paths there are from 'svr' to 'out' which include 'dac' and 'fft'.
fn solve_part_2(input_path: &str) -> u64 {
    const START_NODE: &'static str = "svr";
    const DAC_NODE: &'static str = "dac";
    const FFT_NODE: &'static str = "fft";
    const END_NODE: &'static str = "out";

    let input_lines = read_lines(input_path);
    let adjacency_list = parse_input(input_lines);

    // Input guarantees that there are no loops in the graph
    // This means that when both dac and fft are visited, the order they're reached is always the same
    let path_count_dac_fft = count_paths(DAC_NODE, FFT_NODE, &adjacency_list) as u64;
    if path_count_dac_fft > 0 {
        // Paths are of form svr -> ... -> dac -> ... -> fft -> ... -> out
        let path_count_start_dac = count_paths(START_NODE, DAC_NODE, &adjacency_list) as u64;
        let path_count_fft_end = count_paths(FFT_NODE, END_NODE, &adjacency_list) as u64;

        path_count_start_dac * path_count_dac_fft * path_count_fft_end
    } else {
        // Paths are of form svr -> ... -> fft -> ... -> dac -> ... -> out
        let path_count_start_fft = count_paths(START_NODE, FFT_NODE, &adjacency_list) as u64;
        let path_count_fft_dac = count_paths(FFT_NODE, DAC_NODE, &adjacency_list) as u64;
        let path_count_dac_end = count_paths(DAC_NODE, END_NODE, &adjacency_list) as u64;

        path_count_start_fft * path_count_fft_dac * path_count_dac_end
    }
}
