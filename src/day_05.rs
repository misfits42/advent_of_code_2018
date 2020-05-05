#[aoc_generator(day5)]
fn generate_input(input: &str) -> Vec<char> {
    return input.trim().chars().collect::<Vec<char>>();
}

#[aoc(day5, part1)]
fn solve_part_1(input: &Vec<char>) -> usize {
    let mut polym = input.clone();
    // Start with first pair of characters
    let mut i = 0;
    loop {
        // Terminate when end of string is reached
        if i == polym.len() - 1 || polym.len() <= 2 {
            break;
        }
        // If pair destroys each other, remove the pair and decrement pair index
        if polym[i] != polym[i + 1] 
            && (polym[i + 1] == polym[i].to_ascii_uppercase()
                || polym[i + 1] == polym[i].to_ascii_lowercase()) {
            polym.remove(i);
            polym.remove(i);
            if i > 0 {
                i -= 1;
            }
        } else { // If pair does not destroy each other, increment pair index
            i += 1;
        }
    }
    return polym.len();
}
