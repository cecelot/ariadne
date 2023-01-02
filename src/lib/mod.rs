use self::direction::Direction;
use std::collections::HashMap;

pub mod cell;
pub mod coordinate;
mod direction;
pub mod maze;
pub mod prelude;

pub fn adjustments() -> HashMap<Direction, (isize, isize)> {
    HashMap::from([
        (Direction::North, (-1, 0)),
        (Direction::East, (0, 1)),
        (Direction::South, (1, 0)),
        (Direction::West, (0, -1)),
    ])
}

pub fn opposites() -> HashMap<Direction, Direction> {
    HashMap::from([
        (Direction::North, Direction::South),
        (Direction::East, Direction::West),
        (Direction::South, Direction::North),
        (Direction::West, Direction::East),
    ])
}
