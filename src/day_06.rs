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
    // Initialise empty hashmap to total manhattan distance to each known location from points
    let mut distance_count: HashMap<Point2D, u64> = HashMap::new();
    let distance = 10000;
    // Process each point in input
    for (_, point) in input.iter() {
        // Find the points within set Manhattan distance of current point
        // let close_points = point.calculate_points_within_manhattan_distance(10000);

        for pos_x in (point.pos_x - distance)..(point.pos_x + distance + 1) {
            // Calculate how far above and below central x-axis to go
            let y_extra = (pos_x - point.pos_x + distance).abs();
            for pos_y in (point.pos_y - y_extra)..(point.pos_y + y_extra + 1) {
                // Calculate the current point and its distance from the self point
                let point = Point2D::new(pos_x, pos_y);
                let distance_from_loc: u64 = ((point.pos_x - pos_x).abs() + (point.pos_y - pos_y).abs()) as u64;
                // Check if point with same distance has been found already
                if distance_count.contains_key(&point) {
                    *distance_count.get_mut(&point).unwrap() += distance_from_loc;
                } else {
                    distance_count.insert(point, distance_from_loc);
                }
            }
        }

        // for (distance, near_points) in close_points.iter() {
        //     // For each point at same distance from current point, add its distance total into
        //     // overall distance count
        //     for near_point in near_points {
        //         if distance_count.contains_key(&near_point) {
        //             *distance_count.get_mut(&near_point).unwrap() += distance;
        //         } else {
        //             distance_count.insert(*near_point, *distance);
        //         }
        //     }
        // }
    }
    distance_count.retain(|_k, v| v < &mut 10000);
    let area_size = distance_count.keys().len();
    return area_size;
}
