use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

use super::utils::map::Point2D;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
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
fn generate_input(input: &str) -> Vec<(u64, FabricClaim)> {
    // Initialise empty vector to store parsed results
    let mut results: Vec<(u64, FabricClaim)> = vec![];
    // Capture data fields using regex
    let claim_regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    for capture in claim_regex.captures_iter(input) {
        // Extract capture fields needed for the FabricClaim
        let claim_number = capture[1].parse::<u64>().unwrap();
        let top_left_x = capture[2].parse::<i64>().unwrap();
        let top_left_y = capture[3].parse::<i64>().unwrap();
        let width = capture[4].parse::<u64>().unwrap();
        let height = capture[5].parse::<u64>().unwrap();
        // Create FabricClaim and add to results
        let fabric_claim = FabricClaim::new(top_left_x, top_left_y, width, height);
        results.push((claim_number, fabric_claim))
    }
    return results;
}

#[aoc(day3, part1)]
fn solve_part_1(input: &Vec<(u64, FabricClaim)>) -> usize {
    // Initialise hashmap to track state of claims
    let mut claims_map: HashMap<Point2D, u64> = HashMap::new();
    let mut overlap_spots: HashSet<Point2D> = HashSet::new();
    // Process each claim
    for (_, fabric_claim) in input {
        let start = fabric_claim.top_left_loc;
        // Iterate over all points covered by current claim
        for delta_x in 0..fabric_claim.width {
            for delta_y in 0..fabric_claim.height {
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

#[aoc(day3, part2)]
fn solve_part_2(input: &Vec<(u64, FabricClaim)>) -> u64 {
    // Initialise claims map
    let mut claims_map: HashMap<Point2D, Vec<u64>> = HashMap::new();
    // Process each claim
    for (claim_number, fabric_claim) in input {
        let start = fabric_claim.top_left_loc;
        // Iterate over all points covered by fabric claim
        for delta_x in 0..fabric_claim.width {
            for delta_y in 0..fabric_claim.height {
                // Calculate current point
                let claim_point = start.move_point(delta_x as i64, delta_y as i64);
                if claims_map.contains_key(&claim_point) { // Add point to existing claim
                    claims_map.get_mut(&claim_point).unwrap().push(*claim_number);
                } else { // Add point to new claim
                    claims_map.insert(claim_point, vec![*claim_number]);
                }
            }
        }
    }
    // Find all claims that overlap
    let mut overlaps: HashSet<u64> = HashSet::new();
    let all_claim_numbers: Vec<u64> = input.into_iter().map(|x| x.0).collect::<Vec<u64>>();
    for val in claims_map.values() {
        if val.len() >= 2 {
            for v in val {
                overlaps.insert(*v);
            }
        }
    }
    for claim_number in all_claim_numbers {
        if !overlaps.contains(&claim_number) {
            return claim_number;
        }
    }
    panic!("D3_P2 - should not get here!");
}
