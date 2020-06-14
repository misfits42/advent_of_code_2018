struct RecipeBoard {
    recipes: String,
    orig_recipe_len: usize,
    elf_one_i: usize,
    elf_two_i: usize,
}

impl RecipeBoard {
    pub fn new(orig_recipe: &str) -> Self {
        if orig_recipe.len() < 2 {
            panic!("Day14 - initial recipe length is insufficient.");
        }
        Self {
            recipes: String::from(orig_recipe),
            orig_recipe_len: orig_recipe.len(),
            elf_one_i: 0,
            elf_two_i: 1
        }
    }

    fn get_elf_current_recipes(&self) -> (usize, usize) {
        let elf_one = self.recipes.get(self.elf_one_i..self.elf_one_i+1).unwrap().parse::<usize>().unwrap();
        let elf_two = self.recipes.get(self.elf_two_i..self.elf_two_i+1).unwrap().parse::<usize>().unwrap();
        return (elf_one, elf_two);
    }

    fn update_elf_current_recipes(&mut self) {
        let elf_current_recipes = self.get_elf_current_recipes();
        self.elf_one_i = (self.elf_one_i + elf_current_recipes.0 + 1) % self.recipes.len();
        self.elf_two_i = (self.elf_two_i + elf_current_recipes.1 + 1) % self.recipes.len();
    }

    pub fn conduct_turn(&mut self) {
        // Get current recipe for both elves
        let elf_current_recipes = self.get_elf_current_recipes();
        // Calculate the new recipes
        let new_recipes = elf_current_recipes.0 + elf_current_recipes.1;
        // Add new recipes to the board
        self.recipes += &new_recipes.to_string();
        // Update the current recipe for both elves
        self.update_elf_current_recipes();
    }

    pub fn conduct_turns_until(&mut self, recipes_to_left: usize) -> String {
        loop {
            // Check if we have generated enough new recipes
            if self.recipes.len() >= (recipes_to_left + 10) {
                break;
            }
            // Conduct a new turn to generate new recipes
            self.conduct_turn();
        }
        let result = self.recipes.get(recipes_to_left..(recipes_to_left + 10)).unwrap();
        return String::from(result);
    }
}

#[aoc_generator(day14)]
fn generate_input(input: &str) -> usize {
    return input.parse::<usize>().unwrap();
}

#[aoc(day14, part1)]
fn solve_part_1(recipes_to_left: &usize) -> String {
    // Create new recipe board
    let mut recipe_board = RecipeBoard::new("37");
    // Conduct turns until at least 10 new recipes have been made
    let result = recipe_board.conduct_turns_until(*recipes_to_left);
    return result;
}

#[aoc(day14, part2)]
fn solve_part_2(num_new_recipes: &usize) -> String {
    unimplemented!();
}