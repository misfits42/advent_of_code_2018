use std::cmp::Ordering;
use std::fmt;

/// A simple struct used to represent a 2-dimensional point in Euclidian space.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Point2D {
    pub pos_x: i64,
    pub pos_y: i64,
}

impl Point2D {
    pub fn new(pos_x: i64, pos_y: i64) -> Self {
        Self {
            pos_x: pos_x,
            pos_y: pos_y,
        }
    }

    /// Calculates the new point moved by the given amount in the x- and y-directions.
    pub fn move_point(&self, delta_x: i64, delta_y: i64) -> Point2D {
        return Point2D {
            pos_x: self.pos_x + delta_x,
            pos_y: self.pos_y + delta_y,
        }
    }

    /// Calculates the Manhattan distance between current point and given point.
    pub fn calculate_manhattan_dist(&self, other: &Point2D) -> u64 {
        // Calculate distance in x- and y-axes
        let x_diff = (self.pos_x - other.pos_x).abs();
        let y_diff = (self.pos_y - other.pos_y).abs();
        return (x_diff + y_diff) as u64;
    }
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "<Point2D>[x: {}, y: {}]", self.pos_x, self.pos_y);
    }
}

impl Ord for Point2D {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.pos_x < other.pos_x {
            return Ordering::Less;
        } else if self.pos_x == other.pos_x {
            if self.pos_y < other.pos_y {
                return Ordering::Less;
            } else if self.pos_y == other.pos_y {
                return Ordering::Equal;
            } else {
                return Ordering::Greater;
            }
        } else {
            return Ordering::Greater;
        }
    }
}

impl PartialOrd for Point2D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}
