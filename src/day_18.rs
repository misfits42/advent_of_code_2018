use std::collections::HashMap;

use super::utils::map::Point2D;

use enum_iterator::IntoEnumIterator;

#[derive(Copy, Clone, Hash, PartialEq, Eq, IntoEnumIterator)]
enum LumberAreaTile {
    OpenGround,
    Trees,
    Lumberyard,
}

/// Creates a new HashMap that can be used to count how many of each tile type are in a LumberArea.
fn get_blank_tile_count() -> HashMap<LumberAreaTile, u64> {
    let mut tile_counts = HashMap::<LumberAreaTile, u64>::new();
    for variant in LumberAreaTile::into_enum_iter() {
        tile_counts.insert(variant, 0);
    }
    return tile_counts;
}

/// This struct is used to represent the Lumber Collection Area described in AOC 2018 Day 18. Each
/// tile in the lumber area represents a one-acre-square area.
struct LumberArea {
    minutes_elapsed: u64,
    map: HashMap<Point2D, LumberAreaTile>,
    tile_counts: HashMap<LumberAreaTile, u64>,
}

impl LumberArea {
    pub fn new(raw_input: &str) -> Self {
        // Initialise the variables to hold data parsed from raw input
        let mut map = HashMap::<Point2D, LumberAreaTile>::new();
        let mut tile_counts = get_blank_tile_count();
        // Process each line
        let mut lines = raw_input.lines();
        let mut y = 0;
        let mut x;
        while let Some(line) = lines.next() {
            // Process each character from current line, adding tile to lumber area map
            x = 0;
            for c in line.chars() {
                let loc = Point2D::new(x, y);
                match c {
                    '.' => {
                        map.insert(loc, LumberAreaTile::OpenGround);
                        *tile_counts.get_mut(&LumberAreaTile::OpenGround).unwrap() += 1;
                    }
                    '|' => {
                        map.insert(loc, LumberAreaTile::Trees);
                        *tile_counts.get_mut(&LumberAreaTile::Trees).unwrap() += 1;
                    }
                    '#' => {
                        map.insert(loc, LumberAreaTile::Lumberyard);
                        *tile_counts.get_mut(&LumberAreaTile::Lumberyard).unwrap() += 1;
                    }
                    _ => panic!("Day 18 - invalid map character."),
                }
                x += 1;
            }
            y += 1;
        }
        Self {
            minutes_elapsed: 0,
            map: map,
            tile_counts: tile_counts,
        }
    }

    pub fn duplicate(&self) -> Self {
        Self {
            minutes_elapsed: self.minutes_elapsed,
            map: self.map.clone(),
            tile_counts: self.tile_counts.clone(),
        }
    }

    pub fn calculate_resource_value(&self) -> u64 {
        let trees = self.tile_counts.get(&LumberAreaTile::Trees).unwrap();
        let lumberyards = self.tile_counts.get(&LumberAreaTile::Lumberyard).unwrap();
        return trees * lumberyards;
    }

    fn get_surrounding_tile_count(&self, loc: Point2D) -> HashMap<LumberAreaTile, u64> {
        let mut tile_counts = get_blank_tile_count();
        let surr_points = loc.get_surrounding_points();
        for point in surr_points {
            if let Some(tile) = self.map.get(&point) {
                *tile_counts.get_mut(&tile).unwrap() += 1;
            }
        }
        return tile_counts;
    }

    pub fn simulate_next_minute(&mut self) {
        // Initialise variables to hold updated state of lumber collection area
        let mut new_map = HashMap::<Point2D, LumberAreaTile>::new();
        let mut new_tile_counts = get_blank_tile_count();
        // Visit each acre tile, determining what it will change to for the next minute
        for (loc, tile) in self.map.iter() {
            let surr_tile_counts = self.get_surrounding_tile_count(*loc);
            match tile {
                LumberAreaTile::OpenGround => {
                    if *surr_tile_counts.get(&LumberAreaTile::Trees).unwrap() >= 3 {
                        new_map.insert(*loc, LumberAreaTile::Trees);
                        *new_tile_counts.get_mut(&LumberAreaTile::Trees).unwrap() += 1;
                    } else {
                        new_map.insert(*loc, LumberAreaTile::OpenGround);
                        *new_tile_counts.get_mut(&LumberAreaTile::OpenGround).unwrap() += 1;
                    }
                }
                LumberAreaTile::Trees => {
                    if *surr_tile_counts.get(&LumberAreaTile::Lumberyard).unwrap() >= 3 {
                        new_map.insert(*loc, LumberAreaTile::Lumberyard);
                        *new_tile_counts.get_mut(&LumberAreaTile::Lumberyard).unwrap() += 1;
                    } else {
                        new_map.insert(*loc, LumberAreaTile::Trees);
                        *new_tile_counts.get_mut(&LumberAreaTile::Trees).unwrap() += 1;
                    }
                }
                LumberAreaTile::Lumberyard => {
                    if *surr_tile_counts.get(&LumberAreaTile::Lumberyard).unwrap() >= 1
                        && *surr_tile_counts.get(&LumberAreaTile::Trees).unwrap() >= 1
                    {
                        new_map.insert(*loc, LumberAreaTile::Lumberyard);
                        *new_tile_counts.get_mut(&LumberAreaTile::Lumberyard).unwrap() += 1;
                    } else {
                        new_map.insert(*loc, LumberAreaTile::OpenGround);
                        *new_tile_counts.get_mut(&LumberAreaTile::OpenGround).unwrap() += 1;
                    }
                }
            }
        }
        // Update lumber area with new state
        self.minutes_elapsed += 1;
        self.map = new_map;
        self.tile_counts = new_tile_counts;
    }
}

#[aoc_generator(day18)]
fn generate_input(input: &str) -> LumberArea {
    return LumberArea::new(input);
}

#[aoc(day18, part1)]
fn solve_part_1(input: &LumberArea) -> u64 {
    let mut lumber_area = input.duplicate();
    for _ in 0..10 {
        lumber_area.simulate_next_minute();
    }
    return lumber_area.calculate_resource_value();
}

#[aoc(day18, part2)]
fn solve_part_2(input: &LumberArea) -> u64 {
    let mut lumber_area = input.duplicate();
    // Keep track of how many times each resource value has been seen
    let mut res_val_counts = HashMap::<u64, u64>::new();
    // Keep track of all resource values, in order observed
    let mut res_vals: Vec<u64> = vec![0];
    // Parameters to help us find the cycle
    let cycle_count = 10;
    let min_cycle_length = 20;
    // Simulate minutes going by, looking for a cycle in the resource values
    for minutes_elapsed in 1..=1000000000 {
        // Simulate the next minute and calculate the resource value
        lumber_area.simulate_next_minute();
        let res_val = lumber_area.calculate_resource_value();
        // Update the count for how many times the resource_value has been seen
        if let Some(count) = res_val_counts.get_mut(&res_val) {
            *count += 1;
        } else {
            res_val_counts.insert(res_val, 1);
        }
        // Add the resource value to the list of all values generated
        res_vals.push(res_val);
        if let Some(count) = res_val_counts.get(&res_val) {
            // Check if we have a node with the the limit - indicating cycle
            let mut i = minutes_elapsed;
            if *count >= cycle_count {
                let mut cycle = Vec::<(u64, u64)>::new();
                // Until we reach same values as start with cycle being over min length, keep adding
                // values to it
                loop {
                    let next = res_vals[i];
                    if next == res_val && cycle.len() >= min_cycle_length {
                        // Check which of the minutes in the period aligns with the end-goal
                        let period = cycle.len() as u64;
                        for (mins, val) in cycle.iter() {
                            if (1000000000 - mins) % period == 0 {
                                return *val;
                            }
                        }
                    }
                    // Add the next resource value in the cycle, in backwards order
                    cycle.push((i as u64, next));
                    i -= 1;
                }
            }
        }
    }
    // Return resource value - HOWEVER, algo should have found the cycle by now for puzzle input
    return lumber_area.calculate_resource_value();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d18_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2018/day18.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(604884, result);
    }
}
