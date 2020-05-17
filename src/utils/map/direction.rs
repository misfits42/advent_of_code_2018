#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    /// Checks if the given direction is the opposite of current direction.
    pub fn is_opposite(&self, other: Direction) -> bool {
        match self {
            Direction::North => {
                if other == Direction::South {
                    return true;
                }
            },
            Direction::South => {
                if other == Direction::North {
                    return true;
                }
            },
            Direction::East => {
                if other == Direction::West {
                    return true;
                }
            },
            Direction::West => {
                if other == Direction::East {
                    return true;
                }
            }
        }
        return false;
    }

    /// Gets the direction resulting from rotating 90 degrees CLOCK-WISE.
    pub fn get_cw_rotate(&self) -> Direction {
        match self {
            Direction::North => {
                return Direction::East;
            },
            Direction::South => {
                return Direction::West;
            },
            Direction::East => {
                return Direction::South;
            },
            Direction::West => {
                return Direction::North;
            }
        }
    }

    /// Gets the direction resulting from rotating 90 degrees CLOCK-WISE.
    pub fn get_ccw_rotate(&self) -> Direction {
        match self {
            Direction::North => {
                return Direction::West;
            },
            Direction::South => {
                return Direction::East;
            },
            Direction::East => {
                return Direction::North;
            },
            Direction::West => {
                return Direction::South;
            }
        }
    }
}