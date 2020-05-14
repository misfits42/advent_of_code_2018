use std::collections::HashMap;
use regex::Regex;

struct PlantSim {
    plant_pots: HashMap<i64, bool>,
    plant_recipes: HashMap<u64, bool>,
    left_most_pot: i64,
    right_most_pot: i64,
    total_gen: u64,
}

impl PlantSim {
    pub fn new(initial_plant_state: Vec<bool>, plant_recipes: HashMap<u64, bool>) -> Self {
        Self {
            // Insert the initial plant state with additional two empty pots at either end to start
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
            total_gen: 0
        }
    }

    /// Creates a duplicate of the PlantSim.
    pub fn duplicate(&self) -> PlantSim {
        return PlantSim {
            plant_pots: {
                let mut map = HashMap::<i64, bool>::new();
                map.extend(self.plant_pots.iter());
                map
            },
            plant_recipes: {
                let mut map = HashMap::<u64, bool>::new();
                map.extend(self.plant_recipes.iter());
                map
            },
            left_most_pot: self.left_most_pot,
            right_most_pot: self.right_most_pot,
            total_gen: 0
        };
    }

    /// Adds up the total of all pot numbers for the pots containing plants
    pub fn get_plant_pot_sum(&self) -> i64 {
        let mut sum = 0;
        for (pot_num, plant_state) in self.plant_pots.iter() {
            if *plant_state == true {
                sum += pot_num;
            }
        }
        return sum;
    }

    /// Given a pot number, this function calculates the corresponding recipe number for the pot
    /// based on the state of the surrounding pots. The outcome of the corresponding recipe
    /// determines if the given plant pot will contain a plant (or not) in the next generation.
    pub fn calculate_plant_recipe_num(&self, pot_num: i64) -> u64 {
        let right_bound = pot_num + 2;
        let mut recipe_num = 0;
        // Calculate the recipe number as a binary representation added to decimal
        for pow in (0..5).rev() {
            // Calculate surrounding pot number to check and see if it is within bounds
            let test_pot_num = right_bound - (pow as i64);
            if test_pot_num >= self.left_most_pot && test_pot_num <= self.right_most_pot {
                if *self.plant_pots.get(&test_pot_num).unwrap() == true {
                    recipe_num += (2 as u64).pow(pow);
                }
            }
        }
        return recipe_num;
    }

    pub fn conduct_generation(&mut self) {
        // Increment to the next generation
        self.total_gen += 1;
        // Make empty hashmap to record next state of plant pots
        let mut new_plants = HashMap::<i64, bool>::new();
        // Maintain check if additional pot is needed at left and right bounds
        let mut expand_left = false;
        let mut expand_right = false;
        // Test each plant pot from left most to right most
        for pot_num in self.left_most_pot..(self.right_most_pot + 1) {
            let recipe_num = self.calculate_plant_recipe_num(pot_num);
            if *self.plant_recipes.get(&recipe_num).unwrap() == true {
                new_plants.insert(pot_num, true);
                // Check if we are at left or right bound and need to expand out to a new pot
                if pot_num == self.left_most_pot {
                    expand_left = true;
                    new_plants.insert(self.left_most_pot - 1, false);
                } else if pot_num == self.right_most_pot {
                    expand_right = true;
                    new_plants.insert(self.right_most_pot + 1, false);
                }
            } else {
                new_plants.insert(pot_num, false);
            }
        }
        // Expand the left and right boundaries - if needed.
        if expand_left {
            self.left_most_pot -= 1;
        }
        if expand_right {
            self.right_most_pot += 1;
        }
        // Set the new plants to the current state
        self.plant_pots = new_plants;
    }
}

#[aoc_generator(day12)]
fn generate_input(input: &str) -> PlantSim {
    // Initialise variables to store parse input file data
    let mut plant_pots = Vec::<bool>::new();
    let mut plant_recipes = HashMap::<u64, bool>::new();
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
                    plant_recipes.insert(recipe_number, true);
                } else {
                    plant_recipes.insert(recipe_number, false);
                }
                break;
            }
        }
        line_count += 1;
    }
    return PlantSim::new(plant_pots, plant_recipes);
}

#[aoc(day12, part1)]
fn solve_part_1(input: &PlantSim) -> i64 {
    // Create the PlantSim
    let mut plant_sim = input.duplicate();
    // Conduct 20 generations
    for _ in 0..20 {
        plant_sim.conduct_generation();
    }
    // Get the sum of all plant pot numbers bearing plants
    return plant_sim.get_plant_pot_sum();
}

#[aoc(day12, part2)]
fn solve_part_2(input: &PlantSim) -> i64 {
    // Create the PlantSim
    let mut plant_sim = input.duplicate();
    unimplemented!();
}

/// Converts the raw recipe number into an integer, modelling the raw string as a binary number. In
/// this manner, '#' represents 1 and '.' represents 0.
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
