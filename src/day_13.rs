use super::utils::map::Point2D;
use super::utils::map::Direction;

use std::collections::HashMap;

enum CartTurnDirection {
    Left,
    Straight,
    Right
}

enum TrackElement {
    TrackStraight,
    TrackCorner(Direction, Direction),
    TrackIntersection,
}

struct CropCart {
    next_turn_dir: CartTurnDirection,
    location: Point2D,
    direction: Direction,
}

impl CropCart {
    pub fn new(init_x: i64, init_y: i64, init_direction: Direction) -> Self {
        Self {
            next_turn_dir: CartTurnDirection::Left,
            location: Point2D::new(init_x, init_y),
            direction: init_direction,
        }
    }

    pub fn update_location(&mut self, delta_x: i64, delta_y: i64) {
        self.location = self.location.move_point(delta_x, delta_y);
    }

    pub fn rotate_next_turn_direction(&mut self) {
        self.next_turn_dir = match self.next_turn_dir {
            CartTurnDirection::Left => {
                CartTurnDirection::Straight
            },
            CartTurnDirection::Straight => {
                CartTurnDirection::Right
            },
            CartTurnDirection::Right => {
                CartTurnDirection::Left
            }
        }
    }

    pub fn update_direction(&mut self, new_direction: Direction) {
        self.direction = new_direction;
    }
}

struct CartMap {
    crop_carts: Vec<CropCart>,
    track_map: HashMap<Point2D, TrackElement>,
}

impl CartMap {
    /// Creates a new CartMap from the given initial crop carts and
    pub fn new(init_crop_carts: Vec<CropCart>, init_track_map: HashMap<Point2D, TrackElement>) -> Self {
        unimplemented!();
    }

    /// Ticks along all carts, unless a crash occurs. If a crash occurs, the function stops ticking
    /// along carts and returns the location of the first crash. Otherwise, None is returned.
    pub fn tick_along_carts(&mut self) -> Option<Point2D> {
        unimplemented!();
    }
}

#[aoc_generator(day13)]
fn generate_input(input: &str) -> CartMap {
    unimplemented!();
}

#[aoc(day13, part1)]
fn solve_part_1(input: &CartMap) -> String {
    unimplemented!();
}