use std::{collections::HashSet, env, process};

use advent_of_code_2025::*;
use regex::Regex;

fn main() {
    println!("----- Day 10 -----");

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

#[derive(PartialEq, Eq, Hash, Debug)]
struct LightState {
    lights: Vec<bool>,
}

impl LightState {
    pub fn all_off(light_count: usize) -> Self {
        LightState {
            lights: vec![false; light_count],
        }
    }
}

#[derive(Debug)]
struct Button {
    light_indexes: HashSet<usize>,
}

impl Button {
    pub fn generate_state(&self, current_state: &LightState) -> LightState {
        let lights = current_state
            .lights
            .iter()
            .enumerate()
            .map(|(i, state)| {
                if self.light_indexes.contains(&i) {
                    !*state
                } else {
                    *state
                }
            })
            .collect::<Vec<_>>();

        LightState { lights }
    }
}

#[derive(Debug)]
struct Machine {
    light_count: usize,
    target_state: LightState,
    buttons: Vec<Button>,
    required_joltage: Vec<usize>,
}

fn parse_machines(input_lines: &mut impl Iterator<Item = String>) -> Vec<Machine> {
    let target_state_regex = Regex::new(r"\[([.#]+)\]").unwrap();
    let button_regex = Regex::new(r"\([\d,]+\)").unwrap();
    let required_joltage_regex = Regex::new(r"\{[\d,]+\}").unwrap();

    let mut machines = Vec::new();

    for line in input_lines {
        let target_state_str = target_state_regex.find(&line).unwrap().as_str();
        let target_state = parse_target_state(target_state_str);

        let buttons = button_regex
            .find_iter(&line)
            .map(|m| parse_button(m.as_str()))
            .collect::<Vec<_>>();

        let required_joltage_str = required_joltage_regex.find(&line).unwrap().as_str();
        let required_joltage = parse_required_joltage(required_joltage_str);

        machines.push(Machine {
            light_count: target_state.lights.len(),
            target_state,
            buttons,
            required_joltage,
        });
    }

    machines
}

fn parse_target_state(input: &str) -> LightState {
    let lights = input
        .trim_start_matches("[")
        .trim_end_matches("]")
        .chars()
        .map(|c| match c {
            '.' => false,
            '#' => true,
            x => panic!("Unexpected light state {}", x),
        })
        .collect();

    LightState { lights }
}

fn parse_button(input: &str) -> Button {
    let light_indexes = input
        .trim_start_matches("(")
        .trim_end_matches(")")
        .split(",")
        .filter_map(|idx_str| idx_str.parse::<usize>().ok())
        .collect();

    Button { light_indexes }
}

fn parse_required_joltage(input: &str) -> Vec<usize> {
    input
        .trim_start_matches("{")
        .trim_end_matches("}")
        .split(",")
        .filter_map(|joltage_str| joltage_str.parse::<usize>().ok())
        .collect()
}

fn calculate_min_presses(machine: &Machine) -> usize {
    let initial_state = LightState::all_off(machine.light_count);
    if initial_state == machine.target_state {
        return 0;
    }

    let mut button_presses = 0;
    let mut unexplored_states = HashSet::from([initial_state]);
    let mut seen_states: HashSet<LightState> = HashSet::new();

    while !unexplored_states.is_empty() {
        button_presses += 1;

        let mut starting_states = unexplored_states.drain().collect::<Vec<_>>();
        for button in &machine.buttons {
            for starting_state in &starting_states {
                let derived_state = button.generate_state(&starting_state);

                if derived_state == machine.target_state {
                    return button_presses;
                } else {
                    unexplored_states.insert(derived_state);
                }
            }
        }

        seen_states.extend(starting_states.drain(..));
    }

    panic!("Could not find target state");
}

/// Determines the minimum number of button presses for each machine
fn solve_part_1(input_path: &str) -> usize {
    let mut input_lines = read_lines(input_path);
    let machines = parse_machines(&mut input_lines);

    let mut total_min_presses = 0;

    for machine in machines {
        total_min_presses += calculate_min_presses(&machine);
    }

    total_min_presses
}
