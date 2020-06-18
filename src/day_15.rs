use super::utils::map::Point2D;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

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
    pub fn new(variant: UnitVariant) -> Self {
        Self {
            variant: variant,
            attack_power: 3,
            hit_points: 200,
        }
    }

    pub fn deal_damage(&mut self, damage: u64) -> bool {
        //println!("[{:?}] attacked", self);
        // Work out damage to be dealt
        let damage_to_deal = std::cmp::min(damage, self.hit_points);
        // Deal the damage
        self.hit_points -= damage_to_deal;
        //println!("[{:?}] HP remaining", self.hit_points);
        return self.is_alive();
    }

    pub fn is_alive(&self) -> bool {
        return self.hit_points != 0;
    }

    pub fn get_attack_power(&self) -> u64 {
        return self.attack_power;
    }

    pub fn get_variant(&self) -> UnitVariant {
        return self.variant;
    }

    pub fn get_hit_points(&self) -> u64 {
        return self.hit_points;
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
    pub fn duplicate(&self) -> Self {
        Self {
            full_rounds_compl: self.full_rounds_compl,
            combat_finished: self.combat_finished,
            map: self.map.clone(),
            unit_locations: self.unit_locations.clone()
        }
    }

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

    pub fn get_surrounding_points_space(&self, current_point: Point2D) -> Vec<Point2D> {
        let surr_points = current_point.get_surrounding_points();
        let mut check_points = Vec::<Point2D>::new();
        for surr_point in surr_points {
            // Check if the point contains a wall or another unit
            if *self.map.get(&surr_point).unwrap() != MapTileType::Wall
                && !self.unit_locations.contains_key(&surr_point)
            {
                check_points.push(surr_point);
            }
        }
        return check_points;
    }

    fn check_reachability(&self, start_point: Point2D, end_point: Point2D) -> bool {
        let surr_points = start_point.get_surrounding_points();
        let mut check_points = Vec::<Point2D>::new();
        let mut checked = HashSet::<Point2D>::new();
        for surr_point in surr_points {
            // Check if the point contains a wall or another unit
            if *self.map.get(&surr_point).unwrap() != MapTileType::Wall
                && !self.unit_locations.contains_key(&surr_point)
            {
                check_points.push(surr_point);
            }
        }
        // Get all reachable points
        for check_point in check_points {
            self.check_reachability_recurse(check_point, &mut checked);
        }
        return checked.contains(&end_point);
    }

    fn check_reachability_recurse(&self, current_point: Point2D, checked: &mut HashSet<Point2D>) {
        // Get all surrounding points
        let surr_points = current_point.get_surrounding_points();
        checked.insert(current_point);
        // Check if any surrounding points do not contain a wall, another unit or already checked
        let mut check_points = Vec::<Point2D>::new();
        for surr_point in surr_points {
            if *self.map.get(&surr_point).unwrap() != MapTileType::Wall
                && !self.unit_locations.contains_key(&surr_point)
                && !checked.contains(&surr_point)
            {
                check_points.push(surr_point);
            }
        }
        for check_point in check_points {
            self.check_reachability_recurse(check_point, checked);
        }
    }

    pub fn gen_min_path_dists_bfs(&self, unit_loc: Point2D, reach_loc: Point2D) -> Vec<(Point2D, usize)> {
        // Initialise queue to hold nodes to be visited
        let mut node_queue = VecDeque::<(Point2D, usize)>::new();
        let mut visited = HashMap::<Point2D, usize>::new();
        // Add starting node to the queue
        node_queue.push_back((reach_loc, 0));
        loop {
            // Stop if there are no more nodes to visit
            if node_queue.is_empty() {
                break;
            }
            // Visit the next node in queue
            let (node, depth) = node_queue.pop_front().unwrap();
            visited.insert(node, depth);
            // Add all unvisited neighbours to queue
            let neighbours = self.get_surrounding_points_space(node);
            let nodes = node_queue.iter().map(|x| x.0).collect::<HashSet<Point2D>>();
            for neigh in neighbours {
                if !visited.contains_key(&neigh) && !nodes.contains(&neigh) {
                    node_queue.push_back((neigh, depth + 1));
                }
            }
        }
        // Get out only the points around the unit
        let mut output = Vec::<(Point2D, usize)>::new();
        let unit_surr_points = self.get_surrounding_points_space(unit_loc);
        for point in unit_surr_points {
            if visited.contains_key(&point) {
                let dist = *visited.get(&point).unwrap();
                output.push((point, dist));
            }
        }
        return output;
    }

    pub fn calculate_outcome(&self) -> u64 {
        let hp_sum: u64 = self.unit_locations.values().map(|x| x.hit_points).sum();
        return self.full_rounds_compl * hp_sum;
    }

    fn get_turn_order(&self) -> Vec<Point2D> {
        let mut turn_order = self
            .unit_locations
            .keys()
            .map(|x| *x)
            .collect::<Vec<Point2D>>();
        turn_order.sort_by(|a, b| a.cmp(b));
        return turn_order;
    }
    
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

    fn check_if_enemy_is_adjacent(&self, unit_loc: Point2D, friendly: UnitVariant) -> bool {
        // Get all points around current unit location
        let surr_points = unit_loc.get_surrounding_points();
        // Check if any surrounding points contain an enemy unit
        for point in surr_points {
            if let Some(unit) = self.unit_locations.get(&point) {
                if unit.get_variant() != friendly {
                    return true;
                }
            }
        }
        return false;
    }

    fn conduct_attack(&mut self, attacker_loc: Point2D, friendly: UnitVariant) {
        // Get all points around attacker loc
        let surr_points = attacker_loc.get_surrounding_points();
        let mut min_hp: Option<u64> = None;
        let mut min_hp_targets = Vec::<Point2D>::new();
        // Check surrounding points for enemies
        for point in surr_points {
            if let Some(target_unit) = self.unit_locations.get(&point) {
                if target_unit.get_variant() != friendly {
                    if min_hp.is_none() {
                        min_hp = Some(target_unit.get_hit_points());
                        min_hp_targets = vec![point];
                    } else if target_unit.get_hit_points() == min_hp.unwrap() {
                        min_hp_targets.push(point);
                    } else if target_unit.get_hit_points() < min_hp.unwrap() {
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

        let still_alive = target.deal_damage(attack_unit_pow);
        // If target is no longer alive, remove it from the combat map
        if !still_alive {
            println!("$$$$$$$$ Unit removed (1): {:?}", target_loc);
            self.unit_locations.remove(&target_loc);
        }
    }

    pub fn conduct_turn(&mut self) {
        println!("");
        println!("##################################################");
        println!("");

        println!("Starting turn {} ...", self.full_rounds_compl + 1);
        // Determine move order at start of turn
        let turn_order = self.get_turn_order();
        // println!(">>>> Turn-order: {:?}", turn_order);
        // For each combat unit stil alive:
        // std::thread::sleep(std::time::Duration::from_millis(2000));
        for unit_loc in turn_order {
            // Check if unit is still alive
            if self.unit_locations.get(&unit_loc).is_none() {
                continue;
            }
            // Get the current unit
            let curr_unit = self.unit_locations.get(&unit_loc).unwrap().clone();
            println!(">>>> [{}, {}] Current unit: {:?}", unit_loc.pos_x, unit_loc.pos_y, curr_unit);
            println!(">>>>>>>> Enemies left: {}", self.count_enemies(curr_unit.get_variant()));
            // Check how many enemies are left
            if self.count_enemies(curr_unit.get_variant()) == 0 {
                println!("COMBAT FINISHED!!!");
                for (loc, unit) in self.unit_locations.iter() {
                    println!(">>>> [{}, {}] Current unit: {:?}", loc.pos_x, loc.pos_y, unit);
                    println!(">>>>>>>> Enemies left: {}", self.count_enemies(unit.get_variant()));
                }

                self.combat_finished = true;
                return;
            }

            // // If already in-range of enemy unit, attack and finish unit's turn
            let mut already_attacked = false;
            if self.check_if_enemy_is_adjacent(unit_loc, curr_unit.get_variant()) {
                already_attacked = true;
                // conduct attack
                self.conduct_attack(unit_loc, curr_unit.get_variant());
                println!("START - conducting attack");
            }
            if already_attacked {
                println!("already attacked, continuing...");
                continue;
            }

            // // Determine what squares are in range of enemy units
            let mut in_range_tiles = Vec::<Point2D>::new();
            for (unit_loc, unit) in self.unit_locations.iter() {
                if unit.get_variant() != curr_unit.get_variant() {
                    let surr_points = unit_loc.get_surrounding_points();
                    for surr_point in surr_points {
                        if *self.map.get(&surr_point).unwrap() != MapTileType::Wall
                            && !self.unit_locations.contains_key(&surr_point)
                        {
                            in_range_tiles.push(surr_point);
                        }
                    }
                }
            }
            in_range_tiles.sort_by(|a, b| a.cmp(b));
            //println!(">>>> Tiles in-range: {:?}", in_range_tiles);

            // // Determine what square in range are reachable
            let mut reachable_tiles = HashSet::<Point2D>::new();
            for in_range_tile in in_range_tiles {
                if reachable_tiles.contains(&in_range_tile) {
                    continue;
                }
                if self.check_reachability(unit_loc, in_range_tile) {
                    reachable_tiles.insert(in_range_tile);
                }
            }
            ////println!(">>>> Reachable tiles: {:?}", reachable_tiles);

            // // Determine which reachable in-range squares are closest
            let mut surr_point_min_dists = HashMap::<Point2D, usize>::new();
            let surr_points = &self.get_surrounding_points_space(unit_loc);
            for surr_point in surr_points {
                surr_point_min_dists.insert(*surr_point, usize::MAX);
            }
            ////println!(">>>> Dists: {:?}", surr_point_min_dists);

            // // Determine shortest-path to selected in-range square
            for reachable_tile in reachable_tiles {
                // Get distance to tile from all spaces around current location
                let dists = self.gen_min_path_dists_bfs(unit_loc, reachable_tile);
                ////println!("{:?} dist: {:?}", reachable_tile, dists);
                // Update the shortest distance to target for each surrounding space
                for (loc, dist) in dists {
                    if surr_point_min_dists.contains_key(&loc) && dist < *surr_point_min_dists.get(&loc).unwrap() {
                        surr_point_min_dists.insert(loc, dist);
                    }
                }
            }
            ////println!(">>>> Dists: {:?}", surr_point_min_dists);

            let mut min_dist_points = Vec::<Point2D>::new();
            let mut min_dist = usize::MAX;
            for (loc, dist) in surr_point_min_dists.iter() {
                if *dist < min_dist {
                    min_dist = *dist;
                    min_dist_points = vec![*loc];
                } else if *dist < usize::MAX && *dist == min_dist {
                    min_dist_points.push(*loc);
                }
            }
            min_dist_points.sort_by(|a, b| a.cmp(b));
            
            // Only move if there are options
            if min_dist_points.len() >= 1 {
                let new_loc = min_dist_points[0];
                self.unit_locations.remove(&unit_loc);
                self.unit_locations.insert(new_loc, curr_unit);
                if self.check_if_enemy_is_adjacent(new_loc, curr_unit.get_variant()) {
                    println!("END - conducting attack");
                    self.conduct_attack(new_loc, curr_unit.get_variant());
                }
            }

            if self.check_if_enemy_is_adjacent(unit_loc, curr_unit.get_variant()) {
                println!("END - conducting attack");
                self.conduct_attack(unit_loc, curr_unit.get_variant());
            }
        }

        println!("");
        println!("END OF TURN");
        println!("");
        for (loc, unit) in self.unit_locations.iter() {
            println!(">>>> [{}, {}] Current unit: {:?}", loc.pos_x, loc.pos_y, unit);
            println!(">>>>>>>> Enemies left: {}", self.count_enemies(unit.get_variant()));
        }

        // Full round now completed, so increment count
        self.full_rounds_compl += 1;
    }

    pub fn is_combat_finished(&self) -> bool {
        return self.combat_finished;
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
    println!("Starting combat!");

    loop {
        combat_map.conduct_turn();
        if combat_map.is_combat_finished() {
            return combat_map.calculate_outcome();
        }
    }
}

#[aoc(day15, part2)]
fn solve_part_2(input: &CombatMap) -> u64 {
    unimplemented!();
}
