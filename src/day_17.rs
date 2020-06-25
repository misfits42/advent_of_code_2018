use std::collections::HashMap;
use regex::Regex;

use super::utils::map::Point2D;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum MapTile {
    Clay,
    WaterFlow,
    WaterRest
}

struct ReservoirMap {
    contents: HashMap<Point2D, MapTile>,
    min_y: i64,
    max_y: i64
}

impl ReservoirMap {
    pub fn new(raw_map: &str) -> Self {
        let mut contents = HashMap::<Point2D, MapTile>::new();
        let x_range_regex = Regex::new(r"y=(\d+), x=(\d+)..(\d+)").unwrap();
        let y_range_regex = Regex::new(r"x=(\d+), y=(\d+)..(\d+)").unwrap();
        let mut max_y: i64 = i64::MIN;
        let mut min_y: i64 = i64::MAX;
        for line in raw_map.lines() {
            if x_range_regex.is_match(line) {
                for capture in x_range_regex.captures_iter(line) {
                    let y_loc = capture[1].parse::<i64>().unwrap();
                    let x_low = capture[2].parse::<i64>().unwrap();
                    let x_high = capture[3].parse::<i64>().unwrap();
                    for x in x_low..=x_high {
                        let new_clay_loc = Point2D::new(x, y_loc);
                        contents.insert(new_clay_loc, MapTile::Clay);
                    }
                    if y_loc > max_y {
                        max_y = y_loc;
                    } else if y_loc < min_y {
                        min_y = y_loc;
                    }
                    break;
                }
            } else {
                for capture in y_range_regex.captures_iter(line) {
                    let x_loc = capture[1].parse::<i64>().unwrap();
                    let y_low = capture[2].parse::<i64>().unwrap();
                    let y_high = capture[3].parse::<i64>().unwrap();
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
                }
            }
        }
        
        Self {
            contents: contents,
            min_y: min_y,
            max_y: max_y
        }
    }

    pub fn duplicate(&self) -> Self {
        Self {
            contents: self.contents.clone(),
            min_y: self.min_y,
            max_y: self.max_y
        }
    }

    pub fn flow_water(&mut self) {
        // Start with spring at (x:500, y:0)
        let spring_loc = Point2D::new(500, 0);
        self.flow_water_recurse(spring_loc);
    }

    fn flow_water_recurse(&mut self, spring_start: Point2D,) {
        println!("- Water flow down from: {:?}", spring_start);
        // Check if water can flow down
        let mut curr_loc = spring_start;
        self.contents.insert(curr_loc, MapTile::WaterFlow);
        loop {
            let below_loc = Point2D::new(curr_loc.pos_x, curr_loc.pos_y + 1);
            println!("---- Below loc: {:?}", below_loc);
            // Water can flow down
            if !self.contents.contains_key(&below_loc) {
                // Stop if we will go below the max depth
                if below_loc.pos_y > self.max_y {
                    println!("---- Max depth reached. Stoping flow stream...");
                    break;
                }
                println!("---- Flowing water down...");
                curr_loc = curr_loc.move_point(0, 1);
                self.contents.insert(curr_loc, MapTile::WaterFlow);
            // Otherwise, we have clay below
            } else {
                println!("- Water not flow down from: {:?}", curr_loc);
                println!("---- Water cannot flow down.");
                // Check if the water can flow left without falling down
                println!("---- Checking left flow...");
                let mut left_below_loc = curr_loc.move_point(0, 1);
                let mut left_loc = curr_loc.move_point(-1, 0);
                let mut left_bounded = true;
                loop {
                    // Check if something below and to left
                    if self.contents.contains_key(&left_below_loc) && self.contents.contains_key(&left_loc) {
                        break;
                    // Check if something below but not to left
                    } else if self.contents.contains_key(&left_below_loc) && !self.contents.contains_key(&left_loc) {
                        left_below_loc = left_below_loc.move_point(-1, 0);
                        left_loc = left_loc.move_point(-1, 0);
                    // Otherwise something not below - water flows down again
                    } else {
                        left_bounded = false;
                        break;
                    }
                }
                println!("---- Checking right flow...");
                // Check if water can flow right without falling down
                let mut right_below_loc = curr_loc.move_point(0, 1);
                let mut right_loc = curr_loc.move_point(1, 0);
                let mut right_bounded = true;
                loop {
                    // Check if something below and to right
                    if self.contents.contains_key(&right_below_loc) && self.contents.contains_key(&right_loc) {
                        break;
                    // Check if something below but not to right
                    } else if self.contents.contains_key(&right_below_loc) && !self.contents.contains_key(&right_loc) {
                        right_below_loc = right_below_loc.move_point(1, 0);
                        right_loc = right_loc.move_point(1, 0);
                    // Otherwise something not below - water flows down again
                    } else {
                        right_bounded = false;
                        break;
                    }
                }
                // Check if we need to recurse flows
                let left_bound = left_below_loc.pos_x;
                let right_bound = right_below_loc.pos_x;
                if !left_bounded || !right_bounded {
                    // Insert water flow from current location to before fall point
                    for x in left_bound..curr_loc.pos_x {
                        let new_water_flow_loc = Point2D::new(x, curr_loc.pos_y);
                        self.contents.insert(new_water_flow_loc, MapTile::WaterFlow);
                    }
                    // Insert water flow from current location to fall point to right
                    for x in curr_loc.pos_x..=right_bound {
                        let new_water_flow_loc = Point2D::new(x, curr_loc.pos_y);
                        self.contents.insert(new_water_flow_loc, MapTile::WaterFlow);
                    }
                    if !left_bounded {
                        self.flow_water_recurse(left_below_loc.move_point(0, -1));
                    }
                    if !right_bounded {
                        self.flow_water_recurse(right_below_loc.move_point(0, -1));
                    }
                    break;
                }
                if left_bounded && right_bounded {
                    // Now we know water can flow to left and right - fill the row
                    for x in left_bound..=right_bound {
                        let new_water_loc = Point2D::new(x, curr_loc.pos_y);
                        self.contents.insert(new_water_loc, MapTile::WaterRest);
                    }
                    println!("---- ROW FILLED ---- x={}..{}, y={}", left_bound, right_bound, curr_loc.pos_y);
                    // Move back up water flow again and check flow
                    curr_loc = curr_loc.move_point(0, -1);
                }
            }
        }
        println!("---- Flow stream stopped.");
    }

    pub fn count_water(&self) -> u64 {
        let mut count: u64 = 0;
        for (loc, tile) in self.contents.iter() {
            if loc.pos_y < self.min_y || loc.pos_y > self.max_y {
                continue;
            }
            if *tile == MapTile::WaterFlow || *tile == MapTile::WaterRest {
                count += 1;
            }
        }
        return count;
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
    return reservoir_map.count_water();
}

#[aoc(day17, part2)]
fn solve_part_2(input: &ReservoirMap) -> u64 {
    let mut reservoir_map = input.duplicate();

    unimplemented!();
}
