use super::utils::map::Point2D;

use std::collections::HashMap;
use std::collections::HashSet;

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
        println!("[{:?}] attacked", self);
        // Work out damage to be dealt
        let damage_to_deal = std::cmp::min(damage, self.hit_points);
        // Deal the damage
        self.hit_points -= damage_to_deal;
        println!("[{:?}] HP remaining", self.hit_points);
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

    pub fn determine_shortest_path(&self, start_point: Point2D, end_point: Point2D) -> Vec<Point2D> {
        // Get surrounding points
        let surr_points = self.get_surrounding_points_space(start_point);

        let mut paths = Vec::<Vec<Point2D>>::new();
        for check_point in surr_points {
            if let Some(path) = self.determine_shortest_path_recurse(check_point, end_point, vec![start_point]) {
                println!("##### Path: {:?}", path);
                paths.push(path);
            }
        }

        // Determine which of the paths is shortest
        let mut min_path = Vec::<Point2D>::new();
        let mut min_length = usize::MAX;
        for path in paths {
            if path.len() < min_length {
                min_length = path.len();
                min_path = path;
            }
        }
        return min_path;
    }

    fn determine_shortest_path_recurse(&self, start_point: Point2D, end_point: Point2D, path: Vec<Point2D>) -> Option<Vec<Point2D>> {
        // if path.len() >= 2 && path[0] == Point2D::new(10, 3) && path[1] == Point2D::new(10,4){
        //     println!("######## Current path -----> {:?}", path);
        // }
        if path.len() == 1 {
            println!("######## Current path -----> {:?}", path);
        }

        let mut new_path = path.clone();
        new_path.push(start_point);
        // println!("######## New path -----> {:?}", new_path);
        // std::thread::sleep(std::time::Duration::from_millis(500));

        if start_point == end_point {
            return Some(path);
        }

        // Get surrounding points for current point
        let surr_points = self.get_surrounding_points_space(start_point);
        let mut check_points = Vec::<Point2D>::new();
        // Check if surrounding points are not wall, another unit or last location of path
        for surr_point in surr_points {
            if *self.map.get(&surr_point).unwrap() != MapTileType::Wall
                && !self.unit_locations.contains_key(&surr_point)
                && !new_path.contains(&surr_point)
            {
                check_points.push(surr_point);
                // println!("######## >>> Check point: {:?}", surr_point);
            }
        }
        let mut paths = Vec::<Vec<Point2D>>::new();
        for check_point in check_points {
            if let Some(path) = self.determine_shortest_path_recurse(check_point, end_point, new_path.clone()) {
                paths.push(path);
            }
        }

        let mut min_path: Option<Vec<Point2D>> = None;
        let mut min_len = usize::MAX;
        for path in paths {
            if path.len() < min_len {
                min_len = path.len();
                min_path = Some(path);
            }
        }
        // if min_path.is_none() {
        //     println!("!!! DEAD-END PATH");
        // }
        return min_path;
        // We have reached a dead-end path
        // println!("!!! DEAD-END PATH");
    }

    pub fn get_distance_of_min_path(&self, start_point: Point2D, end_point: Point2D) -> Vec<(Point2D, usize)> {
        // Get surrounding points
        let surr_points = self.get_surrounding_points_space(start_point);

        let mut output = Vec::<(Point2D, usize)>::new();
        for surr_point in surr_points {
            let shortest_path = self.determine_shortest_path(surr_point, end_point);
            output.push((surr_point, shortest_path.len()));
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

    pub fn conduct_turn(&mut self) {
        println!("Starting turn {} ...", self.full_rounds_compl + 1);
        // Determine move order at start of turn
        let turn_order = self.get_turn_order();
        // println!(">>>> Turn-order: {:?}", turn_order);
        // For each combat unit stil alive:
        for curr_loc in turn_order {
            std::thread::sleep(std::time::Duration::from_millis(10));
            // Check if unit is still alive
            if self.unit_locations.get(&curr_loc).is_none() {
                continue;
            }
            // Get the current unit
            let curr_unit = self.unit_locations.get(&curr_loc).unwrap().clone();
            println!(">>>> [{}, {}] Current unit: {:?}", curr_loc.pos_x, curr_loc.pos_y, curr_unit);
            println!(">>>>>>>> Enemies left: {}", self.count_enemies(curr_unit.get_variant()));
            // Check how many enemies are left
            if self.count_enemies(curr_unit.get_variant()) == 0 {
                self.combat_finished = true;
                return;
            }

            // // If already in-range of enemy unit, attack and finish unit's turn
            let mut already_attacked = false;
            let surr_points = curr_loc.get_surrounding_points();
            for point in surr_points {
                if let Some(unit) = self.unit_locations.get_mut(&point) {
                    if unit.get_variant() != curr_unit.get_variant() {
                        let still_alive = unit.deal_damage(curr_unit.get_attack_power());
                        // If target is no longer alive, remove it from the combat map
                        if !still_alive {
                            println!("$$$$$$$$ Unit removed (1): {:?}", point);
                            self.unit_locations.remove(&point);
                        }
                        already_attacked = true;
                        break;
                    }
                }
            }
            if already_attacked {
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
                if self.check_reachability(curr_loc, in_range_tile) {
                    reachable_tiles.insert(in_range_tile);
                }
            }
            // println!(">>>> Reachable tiles: {:?}", reachable_tiles);

            // // Determine which reachable in-range squares are closest
            let mut surr_point_min_dists = HashMap::<Point2D, usize>::new();
            let surr_points = &self.get_surrounding_points_space(curr_loc);
            for surr_point in surr_points {
                surr_point_min_dists.insert(*surr_point, usize::MAX);
            }
            println!(">>>> Dists: {:?}", surr_point_min_dists);

            // // Determine shortest-path to selected in-range square
            for reachable_tile in reachable_tiles {
                // Get distance to tile from all spaces around current location
                let dists = self.get_distance_of_min_path(curr_loc, reachable_tile);
                // Update the shortest distance to target for each surrounding space
                for (loc, dist) in dists {
                    if dist < *surr_point_min_dists.get(&loc).unwrap() {
                        surr_point_min_dists.insert(loc, dist);
                    }
                }
            }
            println!(">>>> Dists: {:?}", surr_point_min_dists);

            let mut min_dist_points = Vec::<Point2D>::new();
            let mut min_dist = usize::MAX;
            for (loc, dist) in surr_point_min_dists.iter() {
                if *dist < min_dist {
                    min_dist = *dist;
                    min_dist_points = vec![*loc];
                } else if *dist == min_dist {
                    min_dist_points.push(*loc);
                }
            }
            min_dist_points.sort_by(|a, b| a.cmp(b));
            let new_loc = min_dist_points[0];

            // // Update location of unit
            self.unit_locations.remove(&curr_loc);
            self.unit_locations.insert(new_loc, curr_unit);

            // // If unit now in range of enemy, attack and finish unit's turn
            let surr_points = new_loc.get_surrounding_points();
            for point in surr_points {
                if let Some(unit) = self.unit_locations.get_mut(&point) {
                    // Only attack enemy units!
                    if unit.get_variant() != curr_unit.get_variant() {
                        let still_alive = unit.deal_damage(curr_unit.get_attack_power());
                        // If target is no longer alive, remove it from the combat map
                        if !still_alive {
                            println!("$$$$$$$$ Unit removed (2): {:?}", point);
                            self.unit_locations.remove(&point);
                        }
                    }
                    break;
                }
            }
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
