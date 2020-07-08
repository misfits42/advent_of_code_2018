use std::collections::HashSet;

#[aoc_generator(day1)]
fn generate_input(input: &str) -> Vec<i32> {
    // Empty vec to store parsed input
    let mut result: Vec<i32> = vec![];
    // Convert each line to integer
    for line in input.lines() {
        let line = line.trim();
        // Skip any lines without a number
        if line.len() == 0 {
            continue;
        }
        let num = line.parse::<i32>().unwrap();
        result.push(num);
    }
    // Return parsed input
    return result;
}

#[aoc(day1, part1)]
pub fn solve_part_1(input: &[i32]) -> i32 {
    let mut freq_result = 0;
    for val in input {
        freq_result += val;
    }
    return freq_result;
}

#[aoc(day1, part2)]
pub fn solve_part_2(input: &[i32]) -> i32 {
    // Empty hash-set to record values observed
    let mut observed_vals: HashSet<i32> = HashSet::new();
    let mut freq_result = 0;
    // Process values in a cycle
    let mut input_cycle = input.iter().cycle();
    loop {
        // Get next change value and calculate result
        let next_val = input_cycle.next().unwrap();
        freq_result += next_val;
        // Check if result has been observed before
        if observed_vals.contains(&freq_result) {
            return freq_result;
        }
        // Add new result to observed collection
        observed_vals.insert(freq_result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d01_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2018/day1.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(466, result);
    }

    #[test]
    fn test_d01_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2018/day1.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(750, result);
    }
}
