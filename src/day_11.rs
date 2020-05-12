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
            let subgrid_power = find_subgrid_power(cell_grid, &top_left);
            if subgrid_power > max_power_seen {
                max_power_seen = subgrid_power;
                max_top_left = top_left;
            }      
        }
    }
    return max_top_left;
}

fn find_subgrid_power(cell_grid: &HashMap<Point2D, i64>, top_left: &Point2D) -> i64 {
    let mut subgrid_power = 0;
    // Add together the powers of each cell in 3x3 grid with given point as top left
    for delta_y in 0..3 {
        for delta_x in 0..3 {
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
