use std::collections::HashMap;

/// Generates logical input from raw input for Day 2.
#[aoc_generator(day2)]
fn generate_input(input: &str) -> Vec<(String, HashMap<u32, Vec<char>>)> {
    // Empty hashmap to store parsed results
    let mut results: Vec<(String, HashMap<u32, Vec<char>>)> = vec![];
    // Process each line in the raw input
    for line in input.lines() {
        // Ignore lines with no input
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }
        // Count number of times each letter occurs on line
        let mut line_count: HashMap<char, u32> = HashMap::new();
        for c in line.chars() {
            // Check if character has been seen already
            if line_count.contains_key(&c) {
                *line_count.get_mut(&c).unwrap() += 1;
            } else { // Add character with initial count
                line_count.insert(c, 1);
            }
        }
        // Add organised characters by number of times each occurs
        let mut reverse_count: HashMap<u32, Vec<char>> = HashMap::new();
        for (k, v) in line_count.iter() {
            // Check if count field has been added already
            if reverse_count.contains_key(&v) {
                reverse_count.get_mut(&v).unwrap().push(*k);
            } else {
                reverse_count.insert(*v, vec![*k]);
            }
        }
        // Add line character count to overall results
        results.push((line.to_owned(), reverse_count));
    }
    return results;
}

#[aoc(day2, part1)]
fn solve_part_1(input: &[(String, HashMap<u32, Vec<char>>)]) -> u32 {
    // Initialise two-letter and three-letter counts
    let mut two_letters = 0;
    let mut three_letters = 0;
    // Process each entry in logical input
    for (_box_id, letter_counts) in input {
        // Check if box_id has two of a letter
        if letter_counts.contains_key(&2) {
            two_letters += 1;
        }
        // Check if box_id has three of a letter
        if letter_counts.contains_key(&3) {
            three_letters += 1;
        }
    }
    // Calculate checksum
    let checksum = two_letters * three_letters;
    return checksum;
}
