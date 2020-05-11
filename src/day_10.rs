use regex::Regex;

use std::collections::HashSet;

use super::utils::map::Point2D;

/// This struct is used to represent a rescue Light Point, as described in AoC 2018 Day 10.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct LightPoint {
    pos: Point2D,
    vel_x: i64,
    vel_y: i64,
}

impl LightPoint {
    /// Creates a new LightPoint with the given initial position and velocity values.
    pub fn new(init_pos_x: i64, init_pos_y: i64, vel_x: i64, vel_y: i64) -> Self {
        Self {
            pos: Point2D::new(init_pos_x, init_pos_y),
            vel_x: vel_x,
            vel_y: vel_y,
        }
    }

    /// Moves the position of the LightPoint by one tick according to its velocity.
    pub fn move_point_tick(&mut self) {
        self.pos = self.pos.move_point(self.vel_x, self.vel_y);
    }
}

/// This struct is used to contain all Light Points observed and manage the simulation of future
/// positions.
struct LightPointSim {
    points: Vec<LightPoint>,
    total_steps: u64,
}

impl LightPointSim {
    pub fn new() -> Self {
        Self {
            points: vec![],
            total_steps: 0,
        }
    }

    pub fn get_copy(&self) -> Self {
        Self {
            points: self.points.to_vec(),
            total_steps: self.total_steps,
        }
    }

    pub fn add_light_point(&mut self, init_pos_x: i64, init_pos_y: i64, vel_x: i64, vel_y: i64) {
        self.points
            .push(LightPoint::new(init_pos_x, init_pos_y, vel_x, vel_y));
    }

    pub fn step_sim(&mut self) {
        // Tick along each light point
        for light_point in self.points.iter_mut() {
            light_point.move_point_tick();
        }
        self.total_steps += 1;
    }

    pub fn get_total_steps(&self) -> u64 {
        return self.total_steps;
    }

    pub fn get_uniq_pos(&self) -> HashSet<Point2D> {
        return self
            .points
            .iter()
            .map(|point| point.pos)
            .collect::<HashSet<Point2D>>();
    }

    pub fn display_points(&self) {
        // Get all unique points, and the min and max in x- and y-directions
        let uniq_pos = self.get_uniq_pos();
        let min_x = uniq_pos.iter().map(|pos| pos.pos_x).min().unwrap();
        let max_x = uniq_pos.iter().map(|pos| pos.pos_x).max().unwrap();
        let min_y = uniq_pos.iter().map(|pos| pos.pos_y).min().unwrap();
        let max_y = uniq_pos.iter().map(|pos| pos.pos_y).max().unwrap();
        // Iterate over all points within bounds
        for pos_y in min_y..(max_y + 1) {
            for pos_x in min_x..(max_x + 1) {
                let c_point = Point2D::new(pos_x, pos_y);
                if uniq_pos.contains(&c_point) {
                    print!("*");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }

    pub fn calculate_box_size(&self) -> i64 {
        // Get all unique points, and the min and max in x- and y-directions
        let uniq_pos = self.get_uniq_pos();
        let min_x = uniq_pos.iter().map(|pos| pos.pos_x).min().unwrap();
        let max_x = uniq_pos.iter().map(|pos| pos.pos_x).max().unwrap();
        let min_y = uniq_pos.iter().map(|pos| pos.pos_y).min().unwrap();
        let max_y = uniq_pos.iter().map(|pos| pos.pos_y).max().unwrap();
        return (max_x - min_x) * (max_y - min_y);
    }
}

#[aoc_generator(day10)]
fn generate_input(input: &str) -> LightPointSim {
    let mut light_point_sim = LightPointSim::new();
    let line_regex = Regex::new(r"position=<(-?\d+),(-?\d+)>velocity=<(-?\d+),(-?\d+)>").unwrap();
    for line in input.lines() {
        // Prepare line for parsing
        let line = line.trim().replace(" ", "");
        // Parse line with regex
        for capture in line_regex.captures_iter(&line) {
            let init_pos_x = capture[1].parse::<i64>().unwrap();
            let init_pos_y = capture[2].parse::<i64>().unwrap();
            let vel_x = capture[3].parse::<i64>().unwrap();
            let vel_y = capture[4].parse::<i64>().unwrap();
            light_point_sim.add_light_point(init_pos_x, init_pos_y, vel_x, vel_y);
            break;
        }
    }
    return light_point_sim;
}

/// Day 10, Part 1 solver.
/// 
/// ASSUMPTION - all light points will be within bounding box spelling out message.
#[aoc(day10, part1)]
fn solve_part_1(input: &LightPointSim) -> i64 {
    let mut light_point_sim = input.get_copy();
    loop {
        light_point_sim.step_sim();
        let box_size = light_point_sim.calculate_box_size();
        if box_size < 1000 {
            println!("DAY 10 PART 1 - SOLUTION");
            light_point_sim.display_points();
            return -1;
        }
    }
}
