use std::collections::HashMap;
use regex::Regex;

struct PlantSim {
    plant_pots: HashMap<i64, bool>,
    plant_recipes: HashMap<u64, bool>,
    left_most_pot: i64,
    right_most_pot: i64,
}

impl PlantSim {
    pub fn new(initial_plant_state: Vec<bool>, plant_recipes: HashMap<u64, bool>) -> Self {
        Self {
            plant_pots: {
                let mut new_pots = HashMap::<i64, bool>::new();
                for i in 0..initial_plant_state.len() {
                    new_pots.insert(i as i64, initial_plant_state[i]);
                }
                new_pots.insert(-2, false);
                new_pots.insert(-1, false);
                new_pots.insert(initial_plant_state.len() as i64, false);
                new_pots.insert((initial_plant_state.len() + 1) as i64, false);
                new_pots
            },
            plant_recipes: plant_recipes,
            left_most_pot: -2,
            right_most_pot: (initial_plant_state.len() + 1) as i64,
        }
    }
}

#[aoc_generator(day12)]
fn generate_input(input: &str) -> (Vec<bool>, HashMap<u64, bool>) {
    // Initialise variables to store parse input file data
    let mut plant_pots = Vec::<bool>::new();
    let mut plant_reciples = HashMap::<u64, bool>::new();
    // Initialise regexes to extract data from lines
    let initial_state_regex = Regex::new(r"initial state: (.?+)").unwrap();
    let recipe_regex = Regex::new("(.?+) => (.?)").unwrap();
    let mut line_count = 0;
    for line in input.lines() {
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }
        if line_count == 0 { // Initial state line
            for capture in initial_state_regex.captures_iter(line) {
                let initial_plant_raw = capture[1].to_owned();
                for c in initial_plant_raw.chars() {
                    if c == '#' {
                        plant_pots.push(true);
                    } else {
                        plant_pots.push(false);
                    }
                }
                break;
            }
        } else {
            for capture in recipe_regex.captures_iter(line) {
                // Extract the raw recipe number and outcome
                let state_raw = capture[1].to_owned();
                let outcome_raw = capture[2].to_owned();
                let recipe_number = calculate_recipe_number(state_raw);
                // Check if the recipe outcome is plant or not
                if outcome_raw == "#" {
                    plant_reciples.insert(recipe_number, true);
                } else {
                    plant_reciples.insert(recipe_number, false);
                }
                break;
            }
        }
        line_count += 1;
    }
    return (plant_pots, plant_reciples);
}

#[aoc(day12, part2)]
fn solve_part_1(input: &(Vec<bool>, HashMap<u64, bool>)) -> i64 {
    println!("{:?}", input.0);
    println!("{:?}", input.1);
    return -1;
}

// Converts the raw recipe number into an integer, modelling the raw string as a binary number. In
// this manner, '#' represents 1 and '.' represents 0.
fn calculate_recipe_number(recipe_num_raw: String) -> u64 {
    let mut state = 0;
    let mut pow = 0;
    for c in recipe_num_raw.chars().rev() {
        if c == '#' {
            state += (2 as u64).pow(pow);
        }
        pow += 1;
    }
    return state;
}
