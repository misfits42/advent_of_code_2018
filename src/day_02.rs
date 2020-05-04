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

#[aoc(day2, part2)]
fn solve_part_2(input: &[(String, HashMap<u32, Vec<char>>)]) -> String {
    // Starting from first, compare current box_id to all following
    for current_index in 0..input.len() {
        for target_index in (current_index+1)..input.len() {
            // Compare each character of the pair
            let mut mismatch_count = 0;
            let mut match_str = String::new();
            let current_id = &input[current_index].0;
            let target_id = &input[target_index].0;
            for i in 0..current_id.len() {
                // Get current character pair
                let current_char = current_id.get(i..i+1).unwrap();
                let target_char = target_id.get(i..i+1).unwrap();
                // Check if characters do not match
                if current_char != target_char {
                    mismatch_count += 1;
                } else { // Characters match, so add to match string
                    match_str.push_str(current_char);
                }
                // Check if we have exceeded valid mismatch total
                if mismatch_count >= 2 {
                    break;
                }
            }
            // Check if we have found the valid pair of box IDs
            if mismatch_count == 1 {
                return match_str;
            }
        }
    }
    panic!("D2_P2 - shouldn't get here!");
}
