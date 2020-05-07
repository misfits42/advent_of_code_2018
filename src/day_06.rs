use super::utils::map::Point2D;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(day6)]
fn generate_input(input: &str) -> HashMap<i64, Point2D> {
    // Create empty hashmap to store parsed results
    let mut results: HashMap<i64, Point2D> = HashMap::new();
    // Create regex to extract location x- and y-coordinates
    let loc_regex = Regex::new(r"(\d+), (\d+)").unwrap();
    // Parse each line of raw input
    let mut location_id: i64 = 1;
    for line in input.lines() {
        let line = line.trim();
        for capture in loc_regex.captures_iter(line) {
            let x_coord = capture[1].parse::<i64>().unwrap();
            let y_coord = capture[2].parse::<i64>().unwrap();
            let location = Point2D::new(x_coord, y_coord);
            results.insert(location_id, location);
            break;
        }
        location_id += 1;
    }
    return results;
}

#[aoc(day6, part1)]
fn solve_part_1(input: &HashMap<i64, Point2D>) -> u64 {
    // Extract all x- and y-coordinates
    let all_locations = input.values().map(|x| *x).collect::<Vec<Point2D>>();
    let all_ids = input.keys().map(|x| *x).collect::<Vec<i64>>();
    let x_coords = all_locations.iter().map(|x| x.pos_x).collect::<Vec<i64>>();
    let y_coords = all_locations.iter().map(|x| x.pos_y).collect::<Vec<i64>>();
    // Find min and max x-coordinates
    let x_min = *x_coords.iter().min().unwrap();
    let x_max = *x_coords.iter().max().unwrap();
    // Find min and max y-coordinates
    let y_min = *y_coords.iter().min().unwrap();
    let y_max = *y_coords.iter().max().unwrap();
    // Initialise map to track closest points to each location
    let mut manhattan_map: HashMap<Point2D, i64> = HashMap::new();
    // Initialise hashmap to track number of points closest to each location
    let mut area_count: HashMap<i64, u64> = HashMap::new();
    for id in all_ids {
        area_count.insert(id, 0);
    }
    // Initialise vector to record all points on boundary
    let mut boundary_points: Vec<Point2D> = vec![];
    // Process each possible point
    for pos_x in x_min..(x_max + 1) {
        for pos_y in y_min..(y_max + 1) {
            let mut closest_dist = u64::MAX;
            let mut contenders: Vec<i64> = vec![];
            // Check if current point is on boundary
            let current_point = Point2D::new(pos_x, pos_y);
            if pos_x == x_min || pos_x == x_max || pos_y == y_min || pos_y == y_max {
                boundary_points.push(current_point);
            }
            for (location_id, location) in input {
                // For each location, calculate Manhattan distance from current point
                let manhattan_dist = current_point.calculate_manhattan_dist(location);
                // if distance is less than current closest, clear contenders and add location id,
                // and update closest dist
                if manhattan_dist < closest_dist {
                    contenders = vec![*location_id];
                    closest_dist = manhattan_dist;
                // If distance is equal to current closest, push location id to contenders
                } else if manhattan_dist == closest_dist {
                    contenders.push(*location_id);
                }
                // if distance is greater than current closest, disregard and proceed to next location
            }
            // If contenders has only one location id, update the area count for the location id
            // and add location id into new entry in manhattan_map
            if contenders.len() == 1 {
                *area_count.get_mut(&contenders[0]).unwrap() += 1;
                manhattan_map.insert(current_point, contenders[0]);
            } else if contenders.len() >= 2 {
                // If contenders has two or more location ids, update manhattan_map with -1 in
                // current point to represent a tie for closest distance
                manhattan_map.insert(current_point, -1);
            }
        }
    }
    // Find each area with presence on outer boundary
    let mut excluded_ids: HashSet<i64> = HashSet::new();
    for point in boundary_points {
        excluded_ids.insert(*manhattan_map.get(&point).unwrap());
    }
    // Remove all area counts with present on outer boundary
    area_count.retain(|k, _| !excluded_ids.contains(&k));
    // Get the max area count after removing infinite regions
    let max_area_size = *area_count
        .values()
        .map(|x| *x)
        .collect::<Vec<u64>>()
        .iter()
        .max()
        .unwrap();
    return max_area_size;
}

#[aoc(day6, part2)]
fn solve_part_2(input: &HashMap<i64, Point2D>) -> usize {
    // Extract all x- and y-coordinates
    let all_locations = input.values().map(|x| *x).collect::<Vec<Point2D>>();
    let x_coords = all_locations.iter().map(|x| x.pos_x).collect::<Vec<i64>>();
    let y_coords = all_locations.iter().map(|x| x.pos_y).collect::<Vec<i64>>();
    // Find min and max x-coordinates
    let x_min = *x_coords.iter().min().unwrap();
    let x_max = *x_coords.iter().max().unwrap();
    // Find min and max y-coordinates
    let y_min = *y_coords.iter().min().unwrap();
    let y_max = *y_coords.iter().max().unwrap();
    // Initialise area count to track size of target region
    let mut area_count = 0;
    let max_dist_total = 10000;
    // Iterate over possible points with less than 10000 total distance from known locations
    for pos_x in (x_min - max_dist_total)..(x_max + max_dist_total + 1) {
        for pos_y in (y_min - max_dist_total)..(y_max + max_dist_total + 1) {
            // Create instance of the current point
            let current_point = Point2D::new(pos_x, pos_y);
            let mut point_dist_total = 0;
            // For each known location, calculate Manhattan distance and add to total
            for location in &all_locations {
                let manhattan_dist = current_point.calculate_manhattan_dist(&location);
                point_dist_total += manhattan_dist;
                // Break if running total dist is over limit already
                if point_dist_total >= (max_dist_total as u64) {
                    break;
                }
            }
            // Check if current point has total dist less than limit - is in target region
            if point_dist_total < (max_dist_total as u64) {
                area_count += 1;
            }
        }
    }
    return area_count;
}
