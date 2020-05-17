use super::utils::map::Point2D;
use super::utils::map::Direction;

use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq)]
enum CartTurnDirection {
    Left,
    Straight,
    Right
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum TrackElement {
    TrackStraight,
    TrackCorner(Direction, Direction),
    TrackIntersection,
}

#[derive(Copy, Clone)]
struct CropCart {
    next_turn_dir: CartTurnDirection,
    location: Point2D,
    direction: Direction,
}

impl CropCart {
    pub fn new(init_loc: Point2D, init_direction: Direction) -> Self {
        Self {
            next_turn_dir: CartTurnDirection::Left,
            location: init_loc,
            direction: init_direction,
        }
    }

    pub fn update_location(&mut self, delta_x: i64, delta_y: i64) {
        self.location = self.location.move_point(delta_x, delta_y);
    }

    pub fn rotate_next_turn_direction(&mut self) {
        // Rotate the cart's direction based on its turning direction
        if self.next_turn_dir == CartTurnDirection::Left {
            self.direction = self.direction.get_ccw_rotate();
        } else if self.next_turn_dir == CartTurnDirection::Right {
            self.direction = self.direction.get_cw_rotate();
        }
        // Change the cart's turning direction to the next in the cycle
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
    crop_carts: HashMap<Point2D, Vec<CropCart>>,
    track_map: HashMap<Point2D, TrackElement>,
}

impl CartMap {
    /// Creates a new CartMap from the given initial crop carts and
    pub fn new(init_crop_carts: HashMap<Point2D, Vec<CropCart>>, init_track_map: HashMap<Point2D, TrackElement>) -> Self {
        Self {
            crop_carts: init_crop_carts,
            track_map: init_track_map
        }
    }

    pub fn duplicate(&self) -> Self {
        Self {
            crop_carts: {
                let mut map = HashMap::<Point2D, Vec<CropCart>>::new();
                for (k, v) in self.crop_carts.iter() {
                    map.insert(*k, v.to_vec());
                }
                map
            },
            track_map: {
                let mut map = HashMap::<Point2D, TrackElement>::new();
                for (k, v) in self.track_map.iter() {
                    map.insert(*k, *v);
                }
                map
            }
        }
    }

    /// Ticks along all carts, unless a crash occurs. If a crash occurs, the function stops ticking
    /// along carts and returns the location of the first crash. Otherwise, None is returned.
    pub fn tick_along_carts(&mut self) -> Option<Point2D> {
        // Get list of cart starting points in order
        let mut start_points = self.crop_carts.keys().map(|x| *x).collect::<Vec<Point2D>>();
        start_points.sort();
        // Try to move each cart
        for start_point in start_points {
            if !self.crop_carts.contains_key(&start_point) {
                continue;
            }
            // Get track element the cart is currently on
            let track_element = self.track_map.get(&start_point).unwrap();
            let new_point = match track_element {
                TrackElement::TrackStraight => {
                    match self.crop_carts.get(&start_point).unwrap()[0].direction {
                        Direction::North => {
                            start_point.move_point(0, -1)
                        },
                        Direction::South => {
                            start_point.move_point(0, 1)
                        },
                        Direction::East => {
                            start_point.move_point(1, 0)
                        },
                        Direction::West => {
                            start_point.move_point(-1, 0)
                        }
                    }
                },
                TrackElement::TrackIntersection => {
                    // Rotate the cart direction
                    self.crop_carts.get_mut(&start_point).unwrap()[0].rotate_next_turn_direction();
                    match self.crop_carts.get(&start_point).unwrap()[0].direction {
                        Direction::North => {
                            start_point.move_point(0, -1)
                        },
                        Direction::South => {
                            start_point.move_point(0, 1)
                        },
                        Direction::East => {
                            start_point.move_point(1, 0)
                        },
                        Direction::West => {
                            start_point.move_point(-1, 0)
                        }
                    }
                },
                TrackElement::TrackCorner(dir1, dir2) => {
                    let cart_dir = self.crop_carts.get(&start_point).unwrap()[0].direction;
                    if cart_dir.is_opposite(*dir1) {
                        self.crop_carts.get_mut(&start_point).unwrap()[0].direction = *dir2;
                    } else {
                        self.crop_carts.get_mut(&start_point).unwrap()[0].direction = *dir1;
                    }
                    match self.crop_carts.get(&start_point).unwrap()[0].direction {
                        Direction::North => {
                            start_point.move_point(0, -1)
                        },
                        Direction::South => {
                            start_point.move_point(0, 1)
                        },
                        Direction::East => {
                            start_point.move_point(1, 0)
                        },
                        Direction::West => {
                            start_point.move_point(-1, 0)
                        }
                    }
                }
            };
            // Check if new point already has a cart
            if self.crop_carts.contains_key(&new_point) {
                // Handle the crash
                let cart = self.crop_carts.get(&start_point).unwrap()[0];
                // Add the cart to the crash site
                self.crop_carts.get_mut(&new_point).unwrap().push(cart);
                // Remove cart from old location
                self.crop_carts.remove(&start_point);
                // Return the location of the crash site
                return Some(new_point);
            } else {
                // Add cart to new location
                let cart = self.crop_carts.get(&start_point).unwrap()[0];
                self.crop_carts.insert(new_point, vec![cart]);
                // Remove cart from old location
                self.crop_carts.remove(&start_point);
            }
        }
        // No crash occurred, so return None
        return None;
    }
}

/// Gets the resulting point if the given point is shifted one unit in the given direction.
fn get_shifted_point(direction: Direction, start_point: Point2D) -> Point2D {
    match direction {
        Direction::North => {
            return start_point.move_point(0, -1);
        },
        Direction::South => {
            return start_point.move_point(0, 1);
        },
        Direction::East => {
            return start_point.move_point(1, 0);
        },
        Direction::West => {
            return start_point.move_point(-1, 0);
        }
    }
}

#[aoc_generator(day13)]
fn generate_input(input: &str) -> CartMap {
    // First read file characters into a 2D array
    let mut map_chars: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        if line.len() == 0 {
            continue;
        }
        let mut new_line_chars: Vec<char> = vec![];
        for c in line.chars() {
            new_line_chars.push(c);
        }
        map_chars.push(new_line_chars);
    }
    // Process each map char into the map
    let mut track_map: HashMap<Point2D, TrackElement> = HashMap::new();
    let mut crop_carts: HashMap<Point2D, Vec<CropCart>> = HashMap::new();
    for pos_y in 0..map_chars.len() {
        for pos_x in 0..map_chars[pos_y].len() {
            let current_loc = Point2D::new(pos_x as i64, pos_y as i64);
            match map_chars[pos_y][pos_x] {
                ' ' => { // Blank space - ignore
                    continue;
                },
                '-' => { // Straight track element - horizontal
                    track_map.insert(current_loc, TrackElement::TrackStraight);
                },
                '|' => { // Straight track element - vertical
                    track_map.insert(current_loc, TrackElement::TrackStraight);
                },
                '+' => { // Track intersection
                    track_map.insert(current_loc, TrackElement::TrackIntersection);
                },
                'v' => { // South-bound cart
                    track_map.insert(current_loc, TrackElement::TrackStraight);
                    crop_carts.insert(current_loc, vec![CropCart::new(current_loc, Direction::South)]); 
                },
                '^' => { // North-bound cart
                    track_map.insert(current_loc, TrackElement::TrackStraight);
                    crop_carts.insert(current_loc, vec![CropCart::new(current_loc, Direction::North)]);
                },
                '<' => { // West-bound cart
                    track_map.insert(current_loc, TrackElement::TrackStraight);
                    crop_carts.insert(current_loc, vec![CropCart::new(current_loc, Direction::West)]);
                },
                '>' => { // East-bound cart
                    track_map.insert(current_loc, TrackElement::TrackStraight);
                    crop_carts.insert(current_loc, vec![CropCart::new(current_loc, Direction::East)]);
                },
                '\\' => { // Left-slant corner
                    // Check for north & east
                    if pos_y >= 1 && pos_x < (map_chars[pos_y].len() - 1) {
                        let c_north = map_chars[pos_y - 1][pos_x];
                        let c_east = map_chars[pos_y][pos_x + 1];
                        if "|+".contains(c_north) && "-+".contains(c_east) {
                            track_map.insert(current_loc, TrackElement::TrackCorner(Direction::North, Direction::East));
                        }
                    }
                    // Check for south & west
                    if pos_y < (map_chars.len() - 1) && pos_x >= 1 { 
                        let c_south = map_chars[pos_y + 1][pos_x];
                        let c_west = map_chars[pos_y][pos_x - 1];
                        if "|+".contains(c_south) && "-+".contains(c_west) {
                            track_map.insert(current_loc, TrackElement::TrackCorner(Direction::South, Direction::West));
                        }
                    }
                },
                '/' => { // Right-slant corner
                    // Check for north & west
                    if pos_y >= 1 && pos_x >= 1 {
                        let c_north = map_chars[pos_y - 1][pos_x];
                        let c_west = map_chars[pos_y][pos_x - 1];
                        if "|+".contains(c_north) && "-+".contains(c_west) {
                            track_map.insert(current_loc, TrackElement::TrackCorner(Direction::North, Direction::West));
                        }
                    }
                    // Check for south & east
                    if pos_y < (map_chars.len() - 1) && pos_x < (map_chars[pos_y].len() - 1) {
                        let c_south = map_chars[pos_y + 1][pos_x];
                        let c_east = map_chars[pos_y][pos_x + 1];
                        if "|+".contains(c_south) && "-+".contains(c_east) {
                            track_map.insert(current_loc, TrackElement::TrackCorner(Direction::South, Direction::East));
                        }
                    }
                },
                _ => {
                    panic!("Day 13 generator - should not get here!");
                }
            }
        }
    }
    return CartMap::new(crop_carts, track_map);
}

#[aoc(day13, part1)]
fn solve_part_1(input: &CartMap) -> String {
    // Duplicate the cart map
    let mut cart_map = input.duplicate();
    let mut total_ticks = 0;
    loop {
        total_ticks += 1;
        // Tick along the carts
        let crash_site = cart_map.tick_along_carts();
        match crash_site {
            Some(crash_loc) => {
                return format!("{},{}", crash_loc.pos_x, crash_loc.pos_y);
            },
            None => {
                continue;
            }
        }
    }
}