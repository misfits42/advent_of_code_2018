use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

use super::utils::map::Point2D;

struct FabricClaim {
    top_left_loc: Point2D,
    width: u64,
    height: u64,
}

impl FabricClaim {
    pub fn new(pos_x: i64, pos_y: i64, width: u64, height: u64) -> Self {
        Self {
            top_left_loc: Point2D::new(pos_x, pos_y),
            width: width,
            height: height,
        }
    }
}

#[aoc_generator(day3)]
fn generate_input(input: &str) -> Vec<FabricClaim> {
    // Initialise empty vector to store parsed results
    let mut results: Vec<FabricClaim> = vec![];
    // Capture data fields using regex
    let claim_regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    for capture in claim_regex.captures_iter(input) {
        // Extract capture fields needed for the FabricClaim
        let _claim_number = capture[1].parse::<u64>().unwrap();
        let top_left_x = capture[2].parse::<i64>().unwrap();
        let top_left_y = capture[3].parse::<i64>().unwrap();
        let width = capture[4].parse::<u64>().unwrap();
        let height = capture[5].parse::<u64>().unwrap();
        // Create FabricClaim and add to results
        let fabric_claim = FabricClaim::new(top_left_x, top_left_y, width, height);
        results.push(fabric_claim)
    }
    return results;
}

#[aoc(day3, part1)]
fn solve_part_1(input: &Vec<FabricClaim>) -> usize {
    // Initialise hashmap to track state of claims
    let mut claims_map: HashMap<Point2D, u64> = HashMap::new();
    let mut overlap_spots: HashSet<Point2D> = HashSet::new();
    // Process each claim
    for claim in input {
        let start = claim.top_left_loc;
        // Iterate over all points covered by current claim
        for delta_x in 0..claim.width {
            for delta_y in 0..claim.height {
                // Calculate current point
                let claim_point = start.move_point(delta_x as i64, delta_y as i64);
                // Check if point is already claimed
                if claims_map.contains_key(&claim_point) {
                    // Increment claim map count for point
                    *claims_map.get_mut(&claim_point).unwrap() += 1;
                    // Add claim point to overlaps
                    overlap_spots.insert(claim_point);
                } else {
                    claims_map.insert(claim_point, 1);
                }
            }
        }
    }
    return overlap_spots.len();
}
