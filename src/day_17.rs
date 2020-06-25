use std::fmt;
use std::collections::HashMap;
use regex::Regex;

use enum_iterator::IntoEnumIterator;

use super::utils::map::Point2D;

#[derive(Copy, Clone, Hash, PartialEq, Eq, IntoEnumIterator)]
enum MapTile {
    Clay,
    WaterFlow,
    WaterRest
}

struct ReservoirMap {
    contents: HashMap<Point2D, MapTile>,
    min_y: i64,
    max_y: i64,
    min_x: i64,
    max_x: i64
}

impl ReservoirMap {
    /// Creates a new ReservourMap from the given raw map input.
    pub fn new(raw_map: &str) -> Self {
        let mut contents = HashMap::<Point2D, MapTile>::new();
        // Create regex to match two variants of line from raw input
        let x_range_regex = Regex::new(r"y=(\d+), x=(\d+)..(\d+)").unwrap();
        let y_range_regex = Regex::new(r"x=(\d+), y=(\d+)..(\d+)").unwrap();
        // Keep track of the min and max X and Y co-ords seen
        let mut max_y: i64 = i64::MIN;
        let mut min_y: i64 = i64::MAX;
        let mut max_x: i64 = i64::MIN;
        let mut min_x: i64 = i64::MAX;
        // Process each line in raw input
        for line in raw_map.lines() {
            // Check if line matches regex with X-coord having range
            if x_range_regex.is_match(line) {
                for capture in x_range_regex.captures_iter(line) {
                    // Extract co-ordinate values from the line
                    let y_loc = capture[1].parse::<i64>().unwrap();
                    let x_low = capture[2].parse::<i64>().unwrap();
                    let x_high = capture[3].parse::<i64>().unwrap();
                    // Record new horizontal clay vein
                    for x in x_low..=x_high {
                        let new_clay_loc = Point2D::new(x, y_loc);
                        contents.insert(new_clay_loc, MapTile::Clay);
                    }
                    // Check in observed co-ord limits need to be updated
                    if y_loc > max_y {
                        max_y = y_loc;
                    } else if y_loc < min_y {
                        min_y = y_loc;
                    }
                    if x_low < min_x {
                        min_x = x_low;
                    }
                    if x_high > max_x {
                        max_x = x_high;
                    }
                    // Line should only match regex once
                    break;
                }
            // Otherwise, the line will match the regex with the Y-coord having a range
            } else {
                for capture in y_range_regex.captures_iter(line) {
                    // Extract co-ordinate values from regex capture groups
                    let x_loc = capture[1].parse::<i64>().unwrap();
                    let y_low = capture[2].parse::<i64>().unwrap();
                    let y_high = capture[3].parse::<i64>().unwrap();
                    // Record new vertical clay vein
                    for y in y_low..=y_high {
                        let new_clay_loc = Point2D::new(x_loc, y);
                        contents.insert(new_clay_loc, MapTile::Clay);
                    }
                    if y_high > max_y {
                        max_y = y_high;
                    }
                    if y_low < min_y {
                        min_y = y_low;
                    }
                    if x_loc < min_x {
                        min_x = x_loc;
                    } else if x_loc > max_x {
                        max_x = x_loc;
                    }
                    // Line should only match regex once
                    break;
                }
            }
        }
        // Create the new ReservoirMap from the data extracted from the raw input
        Self {
            contents: contents,
            min_y: min_y,
            max_y: max_y,
            min_x: min_x - 1,
            max_x: max_x + 1
        }
    }

    /// Creates a duplicate instance of the ReservoirMap by cloning all fields.
    pub fn duplicate(&self) -> Self {
        Self {
            contents: self.contents.clone(),
            min_y: self.min_y,
            max_y: self.max_y,
            min_x: self.min_x,
            max_x: self.max_x
        }
    }

    /// Flows water from spring at (x:500, y:0) until all applicable spaces are filled with water
    /// at rest or flowing water.
    pub fn flow_water(&mut self) {
        // Start with spring at (x:500, y:0)
        let spring_loc = Point2D::new(500, 0);
        self.flow_water_dfs(spring_loc);
    }

    /// Checks if the given location contains a tile that additional water can come to rest upon - 
    /// i.e. clay or other water at rest.
    fn check_if_full(&self, loc: Point2D) -> bool {
        if let Some(tile) = self.contents.get(&loc) {
            if *tile == MapTile::Clay || *tile == MapTile::WaterRest {
                return true;
            }
        }
        return false;
    }

    /// Flows water down from the starting node, using a modified depth-first search algorithm
    /// (adapted to meet needs to puzzle).
    fn flow_water_dfs(&mut self, node: Point2D) {
        // base case - exceeded max depth
        if node.pos_y > self.max_y {
            return;
        }
        // Initially visit current node by inserting a water flow tile
        self.contents.insert(node, MapTile::WaterFlow);
        // Check if water can flow down - visit neighbour below
        let node_below = node.move_point(0, 1);
        if !self.check_if_full(node_below) {
            self.flow_water_dfs(node_below);
        }
        // base case - no rest water under current node even after flow underneath
        if !self.check_if_full(node_below) {
            return;
        // Water cannot flow down
        } else {
            // Check for left bound
            let mut left_bel_node = node.move_point(0, 1);
            let mut left_node = node.move_point(-1, 0);
            let mut is_left_bounded = true;
            loop {
                // Check if we have reached left bound
                if self.check_if_full(left_node) && self.check_if_full(left_bel_node) {
                    break;
                // Not at left bound yet, but still something underneath
                } else if !self.check_if_full(left_node) && self.check_if_full(left_bel_node) {
                    left_bel_node = left_bel_node.move_point(-1, 0);
                    left_node = left_node.move_point(-1, 0);
                // Water will fall
                } else {
                    is_left_bounded = false;
                    break;
                }
            }
            // Check for right bound
            let mut right_bel_node = node.move_point(0, 1);
            let mut right_node = node.move_point(1, 0);
            let mut is_right_bounded = true;
            loop {
                // Check if we are at right bound
                if self.check_if_full(right_node) && self.check_if_full(right_bel_node) {
                    break;
                // Check if we are still going towards right bound, but water not falling
                } else if !self.check_if_full(right_node) && self.check_if_full(right_bel_node) {
                    right_bel_node = right_bel_node.move_point(1, 0);
                    right_node = right_node.move_point(1, 0);
                // Water will fall
                } else {
                    is_right_bounded = false;
                    break;
                }
            }
            // If water is bounded on left and right, fill row and back-track
            let x_left_bound = left_bel_node.pos_x;
            let x_right_bound = right_bel_node.pos_x;
            // If left and right bounds valid, insert water at rest between bounds then backtrack
            if is_left_bounded && is_right_bounded {
                for x in x_left_bound..=x_right_bound {
                    self.contents.insert(Point2D::new(x, node.pos_y), MapTile::WaterRest);
                }
                return;
            // Not bounded, so insert flowing water then flow down from sides where needed
            } else {
                for x in x_left_bound..=x_right_bound {
                    self.contents.insert(Point2D::new(x, node.pos_y), MapTile::WaterFlow);
                }
                if !is_left_bounded {
                    self.flow_water_dfs(left_bel_node.move_point(0, -1));
                }
                if !is_right_bounded {
                    self.flow_water_dfs(right_bel_node.move_point(0, -1));
                }
            }
        }
    }

    /// Counts number of given tile type in the ReserviourMap.
    pub fn get_tile_count(&self, tile_type: MapTile) -> u64 {
        let mut count: u64 = 0;
        for (loc, tile) in self.contents.iter() {
            // Ignore tiles with Y co-ord outside of min and max values from scan results
            if loc.pos_y < self.min_y || loc.pos_y > self.max_y {
                continue;
            }
            if *tile == tile_type {
                count += 1;
            }
        }
        return count;
    }
}

impl std::fmt::Display for ReservoirMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        for y in 0..=self.max_y+1 {
            for x in self.min_x..=self.max_x {
                if x == 500 && y == 0 {
                    output += "+";
                    continue;
                }
                let loc = Point2D::new(x, y);
                if let Some(tile) = self.contents.get(&loc) {
                    match tile {
                        MapTile::Clay => output += "#",
                        MapTile::WaterRest => output += "~",
                        MapTile::WaterFlow => output += "|",
                    }
                } else {
                    output += ".";
                }
            }
            output += "\n";
        }
        write!(f, "{}", output)
    }
}

#[aoc_generator(day17)]
fn generate_input(input: &str) -> ReservoirMap {
    return ReservoirMap::new(input);
}

#[aoc(day17, part1)]
fn solve_part_1(input: &ReservoirMap) -> u64 {
    let mut reservoir_map = input.duplicate();
    reservoir_map.flow_water();
    // Get total number of water tiles after completing flow simulation
    let water_rest = reservoir_map.get_tile_count(MapTile::WaterRest);
    let water_flow = reservoir_map.get_tile_count(MapTile::WaterFlow);
    return water_rest + water_flow;
}

#[aoc(day17, part2)]
fn solve_part_2(input: &ReservoirMap) -> u64 {
    let mut reservoir_map = input.duplicate();
    reservoir_map.flow_water();
    // Get total number of rest water tiles after completing flow simulation
    return reservoir_map.get_tile_count(MapTile::WaterRest);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_d17_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2018/day17.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(31861, result);
    }

    #[ignore]
    #[test]
    fn test_d17_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2018/day17.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(26030, result);
    }
}
