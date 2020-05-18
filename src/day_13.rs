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
    direction: Direction,
}

impl CropCart {
    pub fn new(init_direction: Direction) -> Self {
        Self {
            next_turn_dir: CartTurnDirection::Left,
            direction: init_direction,
        }
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
}

struct CartMap {
    crop_carts: HashMap<Point2D, Vec<CropCart>>,
    track_map: HashMap<Point2D, TrackElement>,
    crash_sites: Vec<Point2D>,
}

impl CartMap {
    /// Creates a new CartMap from the given initial crop carts and
    pub fn new(init_crop_carts: HashMap<Point2D, Vec<CropCart>>, init_track_map: HashMap<Point2D, TrackElement>) -> Self {
        Self {
            crop_carts: init_crop_carts,
            track_map: init_track_map,
            crash_sites: vec![],
        }
    }

    /// Creates a duplicate of the CartMap. All internal data structures and resident data is copied
    /// across.
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
            },
            crash_sites: vec![],
        }
    }

    /// Gets the current direction of the cart at the given location. None is returned if a cart is
    /// not present at the given location.
    fn get_cart_direction(&self, location: Point2D) -> Option<Direction> {
        if !self.crop_carts.contains_key(&location) {
            return None;
        }
        return Some(self.crop_carts.get(&location).unwrap()[0].direction);
    }

    /// Gets the next point the cart at the given location would be in if it moved one unit in its
    /// current direction.
    /// 
    /// This function calls panic! if the given location does not contain a crop cart.
    fn get_shifted_point(&self, location: Point2D) -> Point2D {
        // Get cart direction
        let cart_direction_opt = self.get_cart_direction(location);
        // Get if we have valid cart direction
        if cart_direction_opt == None {
            panic!("PANIC - Day 13 - tried to get direction of non-existent cart.");
        }
        let cart_direction = cart_direction_opt.unwrap();
        // Calculate next point based on current direction
        match cart_direction {
            Direction::North => {
                location.move_point(0, -1)
            },
            Direction::South => {
                location.move_point(0, 1)
            },
            Direction::East => {
                location.move_point(1, 0)
            },
            Direction::West => {
                location.move_point(-1, 0)
            }
        }
    }

    /// Adds the given location to the list of crash sites.
    fn add_crash_site(&mut self, crash_site: Point2D) {
        self.crash_sites.push(crash_site);
    }

    /// Returns the total number of crash sites so far.
    pub fn get_crash_site_count(&self) -> usize {
        return self.crash_sites.len();
    }

    /// Generates and returns a copy of the list of crash sites.
    pub fn get_crash_sites(&self) -> Vec<Point2D> {
        return self.crash_sites.to_vec();
    }

    /// Ticks along all carts, halting on first crash according to flag passed to method.
    pub fn tick_along_carts(&mut self, halt_on_crash: bool) {
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
                    self.get_shifted_point(start_point)
                },
                TrackElement::TrackIntersection => {
                    // Rotate the cart direction
                    self.crop_carts.get_mut(&start_point).unwrap()[0].rotate_next_turn_direction();
                    self.get_shifted_point(start_point)
                },
                TrackElement::TrackCorner(dir1, dir2) => {
                    // We know start point will contain a cart, so we can unwrap straight away
                    let cart_dir = self.get_cart_direction(start_point).unwrap();
                    // Check which direction the cart entered the corner from
                    if cart_dir.is_opposite(*dir1) {
                        self.crop_carts.get_mut(&start_point).unwrap()[0].direction = *dir2;
                    } else {
                        self.crop_carts.get_mut(&start_point).unwrap()[0].direction = *dir1;
                    }
                    self.get_shifted_point(start_point)
                }
            };
            // Check if new point already has a cart
            if self.crop_carts.contains_key(&new_point) {
                // Handle the crash
                let cart = self.crop_carts.get(&start_point).unwrap()[0];
                self.add_crash_site(new_point);
                // Add the cart to the crash site
                self.crop_carts.get_mut(&new_point).unwrap().push(cart);
                // Remove cart from old location
                self.crop_carts.remove(&start_point);
                // Check if we need to halt on crash
                if halt_on_crash {
                    return;
                }
            } else {
                // Add cart to new location
                let cart = self.crop_carts.get(&start_point).unwrap()[0];
                self.crop_carts.insert(new_point, vec![cart]);
                // Remove cart from old location
                self.crop_carts.remove(&start_point);
            }
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
                    crop_carts.insert(current_loc, vec![CropCart::new(Direction::South)]); 
                },
                '^' => { // North-bound cart
                    track_map.insert(current_loc, TrackElement::TrackStraight);
                    crop_carts.insert(current_loc, vec![CropCart::new(Direction::North)]);
                },
                '<' => { // West-bound cart
                    track_map.insert(current_loc, TrackElement::TrackStraight);
                    crop_carts.insert(current_loc, vec![CropCart::new(Direction::West)]);
                },
                '>' => { // East-bound cart
                    track_map.insert(current_loc, TrackElement::TrackStraight);
                    crop_carts.insert(current_loc, vec![CropCart::new(Direction::East)]);
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
    loop {
        // Tick along the carts - halting when first crash occurs
        cart_map.tick_along_carts(true);
        // Check if a crash has occurred
        if cart_map.get_crash_site_count() == 1 {
            let crash_site = cart_map.get_crash_sites()[0];
            return format!("{},{}", crash_site.pos_x, crash_site.pos_y);
        }
    }
}

#[aoc(day13, part2)]
fn solve_part_2(input: &CartMap) -> String {
    // Duplicate the cart map
    let mut cart_map = input.duplicate();
    
    unimplemented!();
}
