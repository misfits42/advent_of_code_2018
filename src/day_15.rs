use super::utils::map::Point2D;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::cmp;

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
enum UnitVariant {
    Goblin,
    Elf,
}

#[derive(Copy, Clone, Hash, Debug)]
struct CombatUnit {
    variant: UnitVariant,
    attack_power: u64,
    hit_points: u64,
}

impl CombatUnit {
    /// Creates a new CombatUnit with given variant. Attack power and hit points are set to default
    /// values of 3 and 200, respectively.
    pub fn new(variant: UnitVariant) -> Self {
        Self {
            variant: variant,
            attack_power: 3,
            hit_points: 200,
        }
    }

    /// Deals damage to the CombatUnit. If more damage would be dealt than the CombatUnit has in
    /// remaining HP, its HP is reduced to 0.
    pub fn deal_damage(&mut self, damage: u64) -> bool {
        // Work out damage to be dealt
        let damage_to_deal = cmp::min(damage, self.hit_points);
        // Deal the damage
        self.hit_points -= damage_to_deal;
        return self.is_alive();
    }

    /// Checks if the CombatUnit has HP remaining.
    pub fn is_alive(&self) -> bool {
        return self.hit_points != 0;
    }

    /// Gets the attack power of the CombatUnit.
    pub fn get_attack_power(&self) -> u64 {
        return self.attack_power;
    }

    /// Gets the variant of the CombatUnit.
    pub fn get_variant(&self) -> UnitVariant {
        return self.variant;
    }

    /// Gets the remaining hit points of the CombatUnit.
    pub fn get_hit_points(&self) -> u64 {
        return self.hit_points;
    }

    /// Updates the attack power of the CombatUnit.
    pub fn update_attack_power(&mut self, power: u64) {
        self.attack_power = power;
    }
}

//

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum MapTileType {
    Wall,
    Space,
}

struct CombatMap {
    map: HashMap<Point2D, MapTileType>,
    unit_locations: HashMap<Point2D, CombatUnit>,
    full_rounds_compl: u64,
    combat_finished: bool,
}

impl CombatMap {
    /// Creates a duplicate instance of the CombatMap by cloning all fields within the original
    /// instance.
    pub fn duplicate(&self) -> Self {
        Self {
            full_rounds_compl: self.full_rounds_compl,
            combat_finished: self.combat_finished,
            map: self.map.clone(),
            unit_locations: self.unit_locations.clone()
        }
    }

    /// Creates a new CombatMap from the given raw map.
    pub fn new(raw_map: &str) -> Self {
        let mut map = HashMap::<Point2D, MapTileType>::new();
        let mut units = HashMap::<Point2D, CombatUnit>::new();
        let lines = raw_map.lines();
        let mut y = 0;
        let mut x = 0;
        // Process each line in raw map, by character
        for line in lines {
            for tile in line.chars() {
                let loc = Point2D::new(x, y);
                match tile {
                    '#' => {
                        map.insert(loc, MapTileType::Wall);
                    }
                    '.' => {
                        map.insert(loc, MapTileType::Space);
                    }
                    'G' => {
                        map.insert(loc, MapTileType::Space);
                        units.insert(loc, CombatUnit::new(UnitVariant::Goblin));
                    }
                    'E' => {
                        map.insert(loc, MapTileType::Space);
                        units.insert(loc, CombatUnit::new(UnitVariant::Elf));
                    }
                    _ => panic!("Day15 - invalid map tile character."),
                }
                x += 1;
            }
            // Go to the next line
            y += 1;
            x = 0;
        }
        Self {
            unit_locations: units,
            map: map,
            full_rounds_compl: 0,
            combat_finished: false,
        }
    }

    pub fn count_num_units(&self, variant: UnitVariant) -> usize {
        let mut count = 0;
        for (_, unit) in self.unit_locations.iter() {
            if unit.get_variant() == variant {
                count += 1;
            }
        }
        return count;
    }

    pub fn update_unit_powers(&mut self, variant: UnitVariant, new_power: u64) {
        for (_, unit) in self.unit_locations.iter_mut() {
            if unit.get_variant() == variant {
                unit.update_attack_power(new_power);
            }
        }
    }

    /// Gets all the locations around the given point that are not walls or occupied by another
    /// unit (friendly or enemy).
    pub fn get_adjacent_points_space(&self, current_point: Point2D) -> Vec<Point2D> {
        let adj_points = current_point.get_adjacent_points();
        let mut check_points = Vec::<Point2D>::new();
        for adj_point in adj_points {
            // Check if the point contains a wall or another unit
            if *self.map.get(&adj_point).unwrap() != MapTileType::Wall
                && !self.unit_locations.contains_key(&adj_point)
            {
                check_points.push(adj_point);
            }
        }
        check_points.sort_by(|a, b| a.cmp(b));
        return check_points;
    }

    /// Checks if the end point can be reached from the start point.
    fn check_reachability(&self, start_point: Point2D, end_point: Point2D) -> bool {
        let result = self.find_min_path_length_bfs(start_point, end_point);
        return result.is_some();
    }

    /// Finds the length of the minimum path between the two given points using a breadth-first
    /// search.
    fn find_min_path_length_bfs(&self, start_loc: Point2D, end_loc: Point2D) -> Option<usize> {
        // Initialise queue to hold nodes to be visited
        let mut node_queue = VecDeque::<(Point2D, usize)>::new();
        let mut visited = HashMap::<Point2D, usize>::new();
        // Add start node to the queue with depth 0
        node_queue.push_back((start_loc, 0));
        // Keep processing nodes until queue is empty
        loop {
            if node_queue.is_empty() {
                break;
            }
            // Visit next node in queue
            let (node, depth) = node_queue.pop_front().unwrap();
            visited.insert(node, depth);
            // Break early if we have reached the end node
            if node == end_loc {
                break;
            }
            // Add all unvisited neighbours to queue
            let neighbours = self.get_adjacent_points_space(node);
            let enqueued_nodes = node_queue.iter().map(|x| x.0).collect::<HashSet<Point2D>>();
            for neigh in neighbours {
                if !visited.contains_key(&neigh) && !enqueued_nodes.contains(&neigh) {
                    node_queue.push_back((neigh, depth + 1));
                }
            }
        }
        // Check if the end location was reached
        if visited.contains_key(&end_loc) {
            return Some(*visited.get(&end_loc).unwrap());
        } else {
            return None;
        }
    }

    /// Finds the length of the minimum path to the end location from all open spaces around the
    /// starting location.
    pub fn get_min_paths_around_unit_loc(&self, unit_loc: Point2D, end_loc: Point2D) -> Vec<(Point2D, usize)> {
        let mut output = Vec::<(Point2D, usize)>::new();
        let check_points = self.get_adjacent_points_space(unit_loc);
        for point in check_points {
            if let Some(dist) = self.find_min_path_length_bfs(point, end_loc) {
                output.push((point, dist));
            }
        }
        return output;
    }

    /// Calculates the output of the combat as the product of no. full rounds completed and total
    /// remaining HP of units.
    pub fn calculate_outcome(&self) -> u64 {
        let hp_sum: u64 = self.unit_locations.values().map(|x| x.hit_points).sum();
        return self.full_rounds_compl * hp_sum;
    }

    /// Determines the turn order for remaining units by sorting all unit locations by the reading
    /// order of their starting positions.
    fn get_turn_order(&self) -> Vec<Point2D> {
        let mut turn_order = self
            .unit_locations
            .keys()
            .map(|x| *x)
            .collect::<Vec<Point2D>>();
        turn_order.sort_by(|a, b| a.cmp(b));
        return turn_order;
    }
    
    /// Counts how many enemy units remain on the combat map.
    fn count_enemies(&self, friendly: UnitVariant) -> u64 {
        let units = self.unit_locations.values().map(|x| *x).collect::<Vec<CombatUnit>>();
        let mut count = 0;
        for unit in units {
            if unit.get_variant() != friendly {
                count += 1;
            }
        }
        return count;
    }

    /// Checks if there is an enemy unit adjacent to the given unit location.
    fn check_if_enemy_is_adjacent(&self, unit_loc: Point2D, friendly: UnitVariant) -> bool {
        // Get all points around current unit location
        let adj_points = unit_loc.get_adjacent_points();
        // Check if any adjacent points contain an enemy unit
        for point in adj_points {
            if let Some(unit) = self.unit_locations.get(&point) {
                if unit.get_variant() != friendly {
                    return true;
                }
            }
        }
        return false;
    }

    /// Conducts an attack from the given attacker location, as per the combat rules.
    fn conduct_attack(&mut self, attacker_loc: Point2D, friendly: UnitVariant) {
        // Get all points around attacker loc
        let adj_points = attacker_loc.get_adjacent_points();
        let mut min_hp: Option<u64> = None;
        let mut min_hp_targets = Vec::<Point2D>::new();
        // Check adjacent points for enemies
        for point in adj_points {
            // Check if point adjacent to attacking unit contains a unit
            if let Some(target_unit) = self.unit_locations.get(&point) {
                // Check if the unit is an enemy
                if target_unit.get_variant() != friendly {
                    if min_hp.is_none() {
                        // First enemy seen, so set the min HP seen
                        min_hp = Some(target_unit.get_hit_points());
                        min_hp_targets = vec![point];
                    } else if target_unit.get_hit_points() == min_hp.unwrap() {
                        // Min HP matches current level, so add target to list
                        min_hp_targets.push(point);
                    } else if target_unit.get_hit_points() < min_hp.unwrap() {
                        // New min HP found, so reinitialise possible targets with current point
                        min_hp = Some(target_unit.get_hit_points());
                        min_hp_targets = vec![point];
                    }
                }
            }
        }
        // If more than one unit with same min HP, break tie with reading order
        min_hp_targets.sort_by(|a, b| a.cmp(b));
        let target_loc = min_hp_targets[0];
        let attack_unit_pow = self.unit_locations.get(&attacker_loc).unwrap().get_attack_power();
        let target = self.unit_locations.get_mut(&target_loc).unwrap();
        // Deal damage to target unit, then check if it is still alive
        let still_alive = target.deal_damage(attack_unit_pow);
        if !still_alive {
            self.unit_locations.remove(&target_loc);
        }
    }

    /// Conducts a single turn of combat.
    pub fn conduct_turn(&mut self) {
        // Determine move order at start of turn
        let turn_order = self.get_turn_order();
        for unit_loc in turn_order {
            // Move to next unit if current unit is dead
            if self.unit_locations.get(&unit_loc).is_none() {
                continue;
            }
            // Get the current unit
            let curr_unit = self.unit_locations.get(&unit_loc).unwrap().clone();
            // Check if all enemies have been eliminated
            if self.count_enemies(curr_unit.get_variant()) == 0 {
                self.combat_finished = true;
                return;
            }

            // // If already in-range of enemy unit, attack and finish unit's turn
            if self.check_if_enemy_is_adjacent(unit_loc, curr_unit.get_variant()) {
                self.conduct_attack(unit_loc, curr_unit.get_variant());
                continue;
            }

            // // Determine what squares are in range of enemy units
            let mut in_range_tiles = HashSet::<Point2D>::new();
            for (unit_loc, unit) in self.unit_locations.iter() {
                if unit.get_variant() != curr_unit.get_variant() {
                    in_range_tiles.extend(self.get_adjacent_points_space(*unit_loc).iter());
                }
            }

            // // Determine what square in range are reachable
            let mut reachable_locs = HashSet::<Point2D>::new();
            for in_range_tile in in_range_tiles {
                if self.check_reachability(unit_loc, in_range_tile) {
                    reachable_locs.insert(in_range_tile);
                }
            }
            // // End turn if no in-range locations are reachable
            if reachable_locs.is_empty() {
                continue;
            }

            // Determine length of min path to each reachable square
            let mut min_dist: Option<usize> = None;
            let mut nearest_squares = Vec::<Point2D>::new();
            for end_loc in reachable_locs {
                // Find length of min path from current location to reachable location
                if let Some(dist) = self.find_min_path_length_bfs(unit_loc, end_loc) {
                    if min_dist.is_none() {
                        min_dist = Some(dist);
                        nearest_squares = vec![end_loc];
                    } else if dist == min_dist.unwrap() {
                        nearest_squares.push(end_loc);
                    } else if dist < min_dist.unwrap() {
                        min_dist = Some(dist);
                        nearest_squares = vec![end_loc];
                    }
                }
            }
            // Sort nearest squares into reading order
            nearest_squares.sort_by(|a, b| a.cmp(b));
            let target_square = nearest_squares[0];
            // Find the min path lengths to selected square from spaces around unit location
            let adj_min_dists = self.get_min_paths_around_unit_loc(unit_loc, target_square);
            // Find which step options have the shortest distance to target square
            let mut min_dist: Option<usize> = None;
            let mut step_options = Vec::<Point2D>::new();
            for (step_option, dist) in adj_min_dists {
                if min_dist.is_none() {
                    min_dist = Some(dist);
                    step_options = vec![step_option];
                } else if dist == min_dist.unwrap() {
                    step_options.push(step_option);
                } else if dist < min_dist.unwrap() {
                    min_dist = Some(dist);
                    step_options = vec![step_option];
                }
            }
            // Sort step options into reading order and select first option
            step_options.sort_by(|a, b| a.cmp(b));
            let step_square = step_options[0];
            
            // Remove unit from old location and move to the new location
            self.unit_locations.remove(&unit_loc);
            self.unit_locations.insert(step_square, curr_unit);
            // Check if an enemy unit is now adjacent to unit after moving
            if self.check_if_enemy_is_adjacent(step_square, curr_unit.get_variant()) {
                self.conduct_attack(step_square, curr_unit.get_variant());
            }
        }
        // Full round now completed, so increment count
        self.full_rounds_compl += 1;
    }

    /// Indicates whether or not the combat has finished - i.e. if all of one variant of unit has
    /// been eliminated.
    pub fn is_combat_finished(&self) -> bool {
        return self.combat_finished;
    }

    pub fn conduct_combat_until_finished(&mut self) {
        loop {
            self.conduct_turn();
            if self.is_combat_finished() {
                return;
            }
        }
    }
}

//

#[aoc_generator(day15)]
fn generate_input(input: &str) -> CombatMap {
    return CombatMap::new(input);
}

#[aoc(day15, part1)]
fn solve_part_1(input: &CombatMap) -> u64 {
    let mut combat_map = input.duplicate();

    loop {
        combat_map.conduct_turn();
        if combat_map.is_combat_finished() {
            return combat_map.calculate_outcome();
        }
    }
}

#[aoc(day15, part2)]
fn solve_part_2(input: &CombatMap) -> u64 {
    let mut elf_power: u64 = 3;
    let num_start_elves = input.count_num_units(UnitVariant::Elf);
    // Keep track of last powers providing elf loss and no loss
    let mut lower_power: u64 = 3;
    let mut upper_power: u64 = u64::MAX;
    let mut last_no_loss_outcome: u64 = 0;
    loop {
        // Make new copy of combat map
        let mut combat_map = input.duplicate();
        // Update elf power
        combat_map.update_unit_powers(UnitVariant::Elf, elf_power);
        // Conduct combat until finished
        combat_map.conduct_combat_until_finished();
        // Check if any elves were lost - using binary-search to update elf power
        if combat_map.count_num_units(UnitVariant::Elf) == num_start_elves {
            // Update upper power and record last outcome of no-loss combat
            upper_power = elf_power;
            last_no_loss_outcome = combat_map.calculate_outcome();
            // Check if we have found the lowest elf power causing no elf losses
            if upper_power - 1 == lower_power {
                return last_no_loss_outcome;
            }
            // Move elf power to halfway between lower and upper limits
            elf_power = lower_power + (upper_power - lower_power) / 2;
        } else {
            // Move the lower limit up
            lower_power = elf_power;
            // Check if we have found the lowest elf power causing no elf losses
            if upper_power - 1 == lower_power {
                return last_no_loss_outcome;
            }
            // Double elf power if no upper limit yet, otherwise increase half towards max power
            if upper_power < u64::MAX {
                elf_power = lower_power + (upper_power - lower_power) / 2;
            } else {
                elf_power *= 2;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_d15_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2018/day15.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(result, 346574);
    }

    #[ignore]
    #[test]
    fn test_d15_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2018/day15.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(result, 60864);
    }

    #[test]
    fn test_d15_p1_example_01() {
        let input = generate_input(&std::fs::read_to_string("./input/2018/test/day_15_t_01.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(result, 27730);
    }

    #[test]
    fn test_d15_p1_example_02() {
        let input = generate_input(&std::fs::read_to_string("./input/2018/test/day_15_t_02.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(result, 36334);
    }

    #[test]
    fn test_d15_p1_example_03() {
        let input = generate_input(&std::fs::read_to_string("./input/2018/test/day_15_t_03.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(result, 39514);
    }

    #[test]
    fn test_d15_p1_example_04() {
        let input = generate_input(&std::fs::read_to_string("./input/2018/test/day_15_t_04.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(result, 27755);
    }

    #[test]
    fn test_d15_p1_example_05() {
        let input = generate_input(&std::fs::read_to_string("./input/2018/test/day_15_t_05.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(result, 28944);
    }

    #[test]
    fn test_d15_p1_example_06() {
        let input = generate_input(&std::fs::read_to_string("./input/2018/test/day_15_t_06.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(result, 18740);
    }
}
