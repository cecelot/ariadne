use super::{adjustments, coordinate::Coordinates, direction::Direction, maze::*};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cell {
  walls: Vec<bool>,
  coordinates: Coordinates,
}

impl Cell {
  pub fn new(row: usize, col: usize) -> Self {
    Self {
      walls: vec![true, true, true, true],
      coordinates: Coordinates::new(row, col),
    }
  }

  fn neighbor<'a>(&self, maze: &'a Maze, direction: Direction) -> Option<&'a Cell> {
    let adjustments = adjustments()[&direction];
    let adjusted = self.coordinates + adjustments;

    maze.get(adjusted)
  }

  pub fn has_unvisited_neighbor(&self, maze: &Maze, visited: &HashSet<Coordinates>) -> bool {
    let unvisited: Vec<_> = adjustments()
      .into_iter()
      .filter(|(_, adjustment)| {
        let adjusted = self.coordinates + *adjustment;
        maze.get(adjusted).is_some() && !visited.contains(&adjusted)
      })
      .collect();

    !unvisited.is_empty()
  }

  pub fn get_unvisited_neighbor(&self, maze: &Maze, visited: &HashSet<Coordinates>) -> Direction {
    let mut rng = rand::thread_rng();

    let directions: Vec<Direction> = vec![
      Direction::North,
      Direction::East,
      Direction::South,
      Direction::West,
    ];

    let mut index = rng.gen_range(0usize..=3);
    let mut direction = directions[index];
    let mut neighbor = self.neighbor(maze, direction);
    while neighbor.is_none() || visited.contains(&neighbor.unwrap().coordinates) {
      index = rng.gen_range(0usize..=3);
      direction = directions[index];
      neighbor = self.neighbor(maze, direction);
    }

    directions[index]
  }

  pub fn wall(&self, direction: &Direction) -> bool {
    self.walls[direction.value()]
  }

  pub fn set_wall(&mut self, direction: Direction, value: bool) {
    self.walls[direction.value()] = value;
  }

  pub fn row_string(&self, row: usize, mark: Option<bool>) -> String {
    let mut ret = String::new();
    let mark = mark.unwrap_or(false);

    let write = &mut |ret: &mut String, direction: &Direction| {
      let s = if self.wall(direction) { "█" } else { " " };
      *ret += s;
    };

    if row == 0 {
      ret += "█";

      write(&mut ret, &Direction::North);

      ret += "█";
    } else {
      write(&mut ret, &Direction::West);

      if mark {
        ret += "X";
      } else {
        ret += " ";
      }
      write(&mut ret, &Direction::East);
    }

    ret
  }
}
