use self::direction::Direction;
use std::collections::HashMap;

pub mod cell;
pub mod coordinate;
mod direction;
pub mod maze;
pub mod prelude;

pub fn history() -> String {
    use std::fs;
    let home = std::env::var("HOME").expect("$HOME is not set");
    let config = std::env::var("XDG_STATE_HOME").unwrap_or(format!("{}/.local/state", home));
    let dir = format!("{}/ariadne", config);
    let _ = fs::create_dir_all(&dir);
    let path = format!("{}/history", dir);
    path
}

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
