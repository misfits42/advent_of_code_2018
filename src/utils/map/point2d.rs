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

    // Calculates the new point moved by the given amount in the x- and y-directions.
    pub fn move_point(&self, delta_x: i64, delta_y: i64) -> Point2D {
        return Point2D {
            pos_x: self.pos_x + delta_x,
            pos_y: self.pos_y + delta_y,
        }
    }
}
