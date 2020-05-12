use std::collections::HashMap;

use super::utils::map::Point2D;

#[aoc_generator(day11)]
fn generate_input(input: &str) -> HashMap<Point2D, i64> {
    let serial_num = input.trim().parse::<i64>().unwrap();
    let mut cell_grid = HashMap::<Point2D, i64>::new();
    for pos_y in 1..301 {
        for pos_x in 1..301 {
            let cell_loc = Point2D::new(pos_x, pos_y);
            let power_level = calculate_power_level(serial_num, &cell_loc);
            cell_grid.insert(cell_loc, power_level);
        }
    }
    return cell_grid;
}

#[aoc(day11, part1)]
fn solve_part_1(cell_grid: &HashMap<Point2D, i64>) -> Point2D {
    let mut max_power_seen = 0;
    let mut max_top_left = Point2D::new(-1, -1);
    for pos_y in 1..299 {
        for pos_x in 1..299 {
            let top_left = Point2D::new(pos_x, pos_y);
            let subgrid_power = find_subgrid_power(cell_grid, &top_left, 3);
            if subgrid_power > max_power_seen {
                max_power_seen = subgrid_power;
                max_top_left = top_left;
            }
        }
    }
    return max_top_left;
}

#[aoc(day11, part2)]
fn solve_part_2(cell_grid: &HashMap<Point2D, i64>) -> String {
    let mut max_power_seen = 0;
    let mut max_top_left = Point2D::new(-1, -1);
    let mut size_of_max = 0; // Width of square providing largest power sum
    for width in 1..301 {
        for pos_y in 1..301 {
            // Break if test square would not fit within cell grid
            if (pos_y + width - 1) > 300 {
                break;
            }
            let mut last_sub_power = 0;
            for pos_x in 1..301 {
                // Break if test square would not fit within cell grid
                if (pos_x + width - 1) > 300 {
                    break;
                }
                // Calculate the current subgrid power by shifting last subgrid across one column
                let top_left = Point2D::new(pos_x, pos_y);
                let subgrid_power = find_subgrid_power_shift(
                    cell_grid,
                    width,
                    last_sub_power,
                    &top_left,
                );
                if subgrid_power > max_power_seen {
                    max_power_seen = subgrid_power;
                    max_top_left = top_left;
                    size_of_max = width;
                }
                last_sub_power = subgrid_power;
            }
        }
        // println!("Completed width {}. Current max power: {}", width, max_power_seen);
    }
    return format!(
        "{},{},{}",
        max_top_left.pos_x, max_top_left.pos_y, size_of_max
    );
}

fn find_subgrid_power_shift(
    cell_grid: &HashMap<Point2D, i64>,
    width: i64,
    last_sub_power: i64,
    top_left: &Point2D,
) -> i64 {
    // Check if we have overlap - subtract the difference
    if top_left.pos_x == 1 {
        let mut subgrid_power = 0;
        for delta_y in 0..width {
            for delta_x in 0..width {
                let test_point = top_left.move_point(delta_x, delta_y);
                subgrid_power += cell_grid.get(&test_point).unwrap();
            }
        }
        return subgrid_power;
    } else {
        let last_x = top_left.pos_x + width - 1;
        let mut subgrid_power = last_sub_power;
        for pos_y in top_left.pos_y..(top_left.pos_y + width) {
            let subtract_point = Point2D::new(top_left.pos_x - 1, pos_y);
            let add_point = Point2D::new(last_x, pos_y);
            subgrid_power -= cell_grid.get(&subtract_point).unwrap();
            subgrid_power += cell_grid.get(&add_point).unwrap();
        }
        return subgrid_power;
    }
}

/// Adds up the power of all cells within the square of specified width starting from the given
/// location.
fn find_subgrid_power(cell_grid: &HashMap<Point2D, i64>, top_left: &Point2D, width: i64) -> i64 {
    let mut subgrid_power = 0;
    // Add together the powers of each cell in 3x3 grid with given point as top left
    for delta_y in 0..width {
        for delta_x in 0..width {
            let test_point = top_left.move_point(delta_x, delta_y);
            subgrid_power += cell_grid.get(&test_point).unwrap();
        }
    }
    return subgrid_power;
}

fn calculate_power_level(serial_no: i64, cell_loc: &Point2D) -> i64 {
    // Find rack ID
    let rack_id = cell_loc.pos_x + 10;
    // Calculate power level
    let mut power_level: i64 = rack_id * cell_loc.pos_y;
    power_level += serial_no;
    power_level *= rack_id;
    power_level = (power_level % 1000) / 100;
    power_level -= 5;
    return power_level;
}
