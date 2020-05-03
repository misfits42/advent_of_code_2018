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
pub fn solve_part1(input: &[i32]) -> i32 {
    let mut freq_result = 0;
    for val in input {
        freq_result += val;
    }
    return freq_result;
}