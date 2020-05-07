use std::collections::HashMap;

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

    /// Calculates all 2D points that are within the specified Manhattan distance of the current
    /// point.
    pub fn calculate_points_within_manhattan_distance(&self, distance: i64) -> HashMap<u64, Vec<Point2D>> {
        let mut points_with_dist: HashMap<u64, Vec<Point2D>> = HashMap::new();
        for pos_x in (self.pos_x - distance)..(self.pos_x + distance + 1) {
            // Calculate how far above and below central x-axis to go
            let y_extra = (pos_x - self.pos_x + distance).abs();
            for pos_y in (self.pos_y - y_extra)..(self.pos_y + y_extra + 1) {
                // Calculate the current point and its distance from the self point
                let point = Point2D::new(pos_x, pos_y);
                let distance_from_loc: u64 = ((self.pos_x - pos_x).abs() + (self.pos_y - pos_y).abs()) as u64;
                // Check if point with same distance has been found already
                if points_with_dist.contains_key(&distance_from_loc) {
                    points_with_dist.get_mut(&distance_from_loc).unwrap().push(point);
                } else {
                    points_with_dist.insert(distance_from_loc, vec![point]);
                }

            }
        }
        return points_with_dist;
    }
}
