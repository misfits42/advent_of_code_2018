/// This struct represents the hot chocolate recipe scoreboard introducted in the AoC 2018 Day 14
/// challenge.
struct RecipeBoard {
    recipes: String,
    num_recipes: usize,
    elf_one_i: usize,
    elf_two_i: usize,
}

impl RecipeBoard {
    /// Creates a new RecipeBoard with the given starting recipes.
    pub fn new(orig_recipe: &str) -> Self {
        if orig_recipe.chars().count() < 2 {
            panic!("Day14 - initial recipe length is insufficient.");
        }
        Self {
            recipes: String::from(orig_recipe),
            num_recipes: orig_recipe.chars().count(),
            elf_one_i: 0,
            elf_two_i: 1
        }
    }

    /// Extracts the current recipe scores for the two elves.
    fn get_elf_current_recipes(&self) -> (usize, usize) {
        let elf_one = self.recipes.get(self.elf_one_i..self.elf_one_i+1).unwrap().parse::<usize>().unwrap();
        let elf_two = self.recipes.get(self.elf_two_i..self.elf_two_i+1).unwrap().parse::<usize>().unwrap();
        return (elf_one, elf_two);
    }

    /// Updates the current recipe locations of the two elves based on their current recipe scores.
    fn update_elf_current_recipes(&mut self) {
        let elf_current_recipes = self.get_elf_current_recipes();
        self.elf_one_i = (self.elf_one_i + elf_current_recipes.0 + 1) % self.num_recipes;
        self.elf_two_i = (self.elf_two_i + elf_current_recipes.1 + 1) % self.num_recipes;
    }

    pub fn conduct_turn(&mut self) {
        // Get current recipe for both elves
        let elf_current_recipes = self.get_elf_current_recipes();
        // Calculate the new recipes
        let new_recipes = elf_current_recipes.0 + elf_current_recipes.1;
        // Add new recipes to the board. (int div rounds down).
        self.recipes.extend(new_recipes.to_string().chars());
        self.num_recipes += (new_recipes / 10) + 1;
        // Update the current recipe for both elves
        self.update_elf_current_recipes();
    }

    /// Conducts turns until there is at least the given number of recipes + 10 in the recipe list.
    /// 
    /// Returns the sequence of 10 recipes immediately after the given number of recipes.
    pub fn conduct_turns_until(&mut self, to_left: usize) -> String {
        loop {
            // Check if we have generated enough new recipes
            if self.num_recipes >= (to_left + 10) {
                return String::from(self.recipes.get(to_left..(to_left + 10)).unwrap());
            }
            // Conduct a new turn to generate new recipes
            self.conduct_turn();
        }
    }

    /// Conducts turns until the given score sequence appears in the recipe list. The score sequence
    /// will either appear at the end of the recipe list or one offset from the end.
    /// 
    /// Returns the number of recipes to the left of the first appearance of the given score
    /// sequence.
    pub fn conduct_turns_until_appearance(&mut self, score_seq: &str) -> usize {
        let seq_len = score_seq.chars().count();
        loop {
            // Check if given sequence is either at end or end offset by 1
            if self.num_recipes > seq_len {
                // Get the string at end and set back by 1
                let end = self.recipes.get((self.num_recipes - seq_len)..self.num_recipes).unwrap();
                let end_1 = self.recipes.get((self.num_recipes - seq_len - 1)..(self.num_recipes-1)).unwrap();
                if score_seq == end {
                    return self.num_recipes - seq_len
                } else if score_seq == end_1 {
                    return self.num_recipes - seq_len - 1;
                }
            }
            // Conduct another turn
            self.conduct_turn();
        }
    }
}

#[aoc_generator(day14)]
fn generate_input(input: &str) -> String {
    return String::from(input);
}

#[aoc(day14, part1)]
fn solve_part_1(input: &String) -> String {
    // Create new recipe board
    let mut recipe_board = RecipeBoard::new("37");
    // Conduct turns until at least 10 new recipes have been made
    let recipes_to_left = input.parse::<usize>().unwrap();
    return recipe_board.conduct_turns_until(recipes_to_left);
}

#[aoc(day14, part2)]
fn solve_part_2(input: &String) -> usize {
    // Create new recipe board
    let mut recipe_board = RecipeBoard::new("37");
    return recipe_board.conduct_turns_until_appearance(input);
}
