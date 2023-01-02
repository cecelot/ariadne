#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

impl Direction {
    pub fn value(&self) -> usize {
        match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }
}
