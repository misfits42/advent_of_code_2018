use std::collections::HashSet;

#[aoc_generator(day5)]
fn generate_input(input: &str) -> Vec<char> {
    return input.trim().chars().collect::<Vec<char>>();
}

#[aoc(day5, part1)]
fn solve_part_1(input: &Vec<char>) -> usize {
    let mut polymer = react_polymer(input);
    return polymer.len();
}

#[aoc(day5, part2)]
fn solve_part_2(input: &Vec<char>) -> usize {
    // Get all unique unit types in the input polymer
    let mut unit_types: HashSet<char> = input
        .into_iter()
        .map(|x| x.to_ascii_lowercase())
        .collect::<HashSet<char>>();
    // Keep track of min length observed
    let mut min_polymer_length = usize::MAX;
    for unit_type in unit_types {
        let mut test_polymer = input.clone();
        // Remove all instances of current unit type from test polymer
        test_polymer.retain(|x| *x != unit_type && *x != unit_type.to_ascii_uppercase());
        // Fully react the test polymer and check if it is the shortest seen so far
        let reacted_polymer = react_polymer(&test_polymer);
        if reacted_polymer.len() < min_polymer_length {
            min_polymer_length = reacted_polymer.len();
        }
    }
    return min_polymer_length;
}

/// Fully reacts the given input polymer, progressively destroying all type-pairs of opposite
/// polarity.
fn react_polymer(input_poly: &Vec<char>) -> Vec<char> {
    let mut polymer = input_poly.clone();
    // Start with first pair of characters
    let mut i = 0;
    loop {
        // Terminate when end of string is reached
        if i == polymer.len() - 1 || polymer.len() <= 2 {
            break;
        }
        // If pair destroys each other, remove the pair and decrement pair index
        if polymer[i] != polymer[i + 1]
            && (polymer[i + 1] == polymer[i].to_ascii_uppercase()
                || polymer[i + 1] == polymer[i].to_ascii_lowercase())
        {
            polymer.remove(i);
            polymer.remove(i);
            if i > 0 {
                i -= 1;
            }
        } else {
            // If pair does not destroy each other, increment pair index
            i += 1;
        }
    }
    return polymer;
}
