use regex::Regex;

use super::utils::map::Point2D;

/// This struct is used to represent a rescue Light Point, as described in AoC 2018 Day 10.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct LightPoint {
    position: Point2D,
    vel_x: i64,
    vel_y: i64,
}

impl LightPoint {
    /// Creates a new LightPoint with the given initial position and velocity values.
    pub fn new(init_pos_x: i64, init_pos_y: i64, vel_x: i64, vel_y: i64) -> Self {
        Self {
            position: Point2D::new(init_pos_x, init_pos_y),
            vel_x: vel_x,
            vel_y: vel_y,
        }
    }

    /// Moves the position of the LightPoint by one tick according to its velocity.
    pub fn move_point_tick(&mut self) {
        self.position = self.position.move_point(self.vel_x, self.vel_y);
    }

    /// Gets the current position of the LightPoint.
    pub fn get_position(&self) -> Point2D {
        return self.position;
    }
}

/// This struct is used to contain all Light Points observed and manage the simulation of future
/// positions.
struct LightPointSim {
    points: Vec<LightPoint>
}

impl LightPointSim {
    pub fn new() -> Self {
        Self {
            points: vec![],
        }
    }

    pub fn add_light_point(&mut self, init_pos_x: i64, init_pos_y: i64, vel_x: i64, vel_y: i64) {
        self.points.push(LightPoint::new(init_pos_x, init_pos_y, vel_x, vel_y));
    }

    pub fn step_sim(&mut self) {
        // Tick along each light point
        for light_point in self.points.iter_mut() {
            light_point.move_point_tick();
        }
    }

    pub fn get_light_points(&self) -> Vec<LightPoint> {
        return self.points.to_vec();
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
            // println!("{},{},{},{}", init_pos_x, init_pos_y, vel_x, vel_y);
            break;
        }
    }
    return light_point_sim;
}

#[aoc(day10, part1)]
fn solve_part_1(light_point_sim: &LightPointSim) -> String {
    // println!("No. light points: {}", light_point_sim.get_light_points().len());
    return String::new();
}
