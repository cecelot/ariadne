use super::{
  adjustments,
  cell::Cell,
  coordinate::Coordinates,
  direction::{Direction, DIRECTIONS},
  opposites,
};
use serde::{Deserialize, Serialize};
use std::{
  collections::{HashSet, VecDeque},
  fs::File,
  io::BufReader,
  path::Path,
  sync::{mpsc::Sender, Arc, RwLock},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Maze {
  pub width: usize,
  pub height: usize,
  cells: Vec<Cell>,
}

impl Maze {
  pub fn new(width: usize, height: usize) -> Self {
    let mut m = Self {
      width,
      height,
      cells: vec![],
    };

    m.init();

    m
  }

  pub fn init(&mut self) {
    let (width, height) = self.dimensions();

    for row in 0..height {
      for col in 0..width {
        let coords: Coordinates = (row, col).into();
        self.set(coords, Cell::new(row, col));
      }
    }
  }

  fn remove_wall(&mut self, coordinates: Coordinates, direction: Direction) {
    let adjustments = adjustments()[&direction];
    let adjusted = coordinates + adjustments;
    let opposite = opposites()[&direction];
    {
      let origin = self
        .get_mut(coordinates)
        .expect("Expected a cell at coordinates");
      origin.set_wall(direction, false);
    }
    let dest = self
      .get_mut(adjusted)
      .expect("Expected a cell at coordinates");
    dest.set_wall(opposite, false);
  }

  fn dimensions(&self) -> (usize, usize) {
    (self.width, self.height)
  }

  fn set(&mut self, coordinates: Coordinates, cell: Cell) {
    self.cells.insert(self.calculate_index(coordinates), cell);
  }

  fn get_mut(&mut self, coordinates: Coordinates) -> Option<&mut Cell> {
    let index = self.calculate_index(coordinates);
    self.cells.get_mut(index)
  }

  pub fn get(&self, coordinates: Coordinates) -> Option<&Cell> {
    if self.within_bounds(coordinates) {
      let index = self.calculate_index(coordinates);
      self.cells.get(index)
    } else {
      None
    }
  }

  fn calculate_index(&self, coordinates: Coordinates) -> usize {
    let (width, _) = self.dimensions();
    width * coordinates.row + coordinates.col
  }

  fn within_bounds(&self, coordinates: Coordinates) -> bool {
    let (row, col) = coordinates.into();
    let (width, height) = self.dimensions();
    row < height && col < width
  }

  pub fn spread(&self, visited: HashSet<Coordinates>) -> String {
    let mut ret = String::new();
    for row in 0..self.height {
      for node_row in 0..2 {
        for col in 0..self.width {
          let cell = self.get((row, col).into());
          let mark = if visited.contains(&(row, col).into()) {
            true
          } else {
            false
          };
          ret += &cell.unwrap().row_string(node_row, Some(mark));
          if col == self.width - 1 {
            ret += "\n";
          }
        }
      }
    }
    for _ in 0..self.width {
      ret += "███";
    }
    ret
  }

  pub fn string(&self) -> String {
    let mut ret = String::new();
    for row in 0..self.height {
      for node_row in 0..2 {
        for col in 0..self.width {
          let cell = self.get((row, col).into());
          ret += &cell.unwrap().row_string(node_row, None);
          if col == self.width - 1 {
            ret += "\n";
          }
        }
      }
    }
    for _ in 0..self.width {
      ret += "███";
    }
    ret
  }
}

impl<P: AsRef<Path>> From<P> for Maze {
  fn from(path: P) -> Maze {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let u = serde_json::from_reader(reader).expect("Failed to parse JSON");

    u
  }
}

pub struct DFSOptions {
  width: usize,
  height: usize,
  visualize: bool,
  tx: Sender<Maze>,
}

impl DFSOptions {
  pub fn new(width: usize, height: usize, visualize: bool, tx: Sender<Maze>) -> Self {
    Self {
      width,
      height,
      visualize,
      tx,
    }
  }

  pub fn create(&mut self) -> Maze {
    let mut maze = Maze {
      width: self.width,
      height: self.height,
      cells: Vec::new(),
    };

    maze.init();

    let mut stack: Vec<Coordinates> = Vec::new();
    let mut visited: HashSet<Coordinates> = HashSet::new();
    stack.push((0, 0).into());
    while !stack.is_empty() {
      let top = *stack.last().expect("Expected stack to be non-empty");
      stack.pop();

      let has_unvisited_neighbor = maze
        .get(top)
        .expect("Expected maze to have a cell at coordinates")
        .has_unvisited_neighbor(&maze, &visited);

      if has_unvisited_neighbor {
        stack.push(top.into());
        let adjust: (isize, isize);
        let direction: Direction;
        {
          direction = maze
            .get(top)
            .expect("Expected maze to have a cell at coordinates")
            .get_unvisited_neighbor(&maze, &visited);
          adjust = adjustments()[&direction];
        }
        let adjusted = top + adjust;
        maze.remove_wall(top, direction);
        if self.visualize {
          self.tx.send(maze.clone()).unwrap();
        }
        visited.insert(adjusted);
        stack.push(adjusted);
      }
    }

    maze
  }
}

pub trait Solver {
  fn solve(&mut self, visualize: bool, tx: Sender<HashSet<Coordinates>>) -> bool;
}

pub struct BFS<'a> {
  maze: &'a Arc<RwLock<Maze>>,
  relevant: HashSet<Coordinates>,
}

impl<'a> BFS<'a> {
  pub fn new(maze: &'a Arc<RwLock<Maze>>) -> Self {
    Self {
      maze,
      relevant: HashSet::new(),
    }
  }
}

impl<'a> Solver for BFS<'a> {
  fn solve(&mut self, visualize: bool, tx: Sender<HashSet<Coordinates>>) -> bool {
    let root: Coordinates = (0, 0).into();
    let mut end: Coordinates = self.maze.read().unwrap().dimensions().into();
    end.row -= 1;
    end.col -= 1;

    let mut queue: VecDeque<Coordinates> = VecDeque::new();
    let mut explored: HashSet<Coordinates> = HashSet::new();
    queue.push_back(root);

    while !queue.is_empty() {
      let m = self.maze.read().unwrap();
      let top = *queue.front().expect("Expected queue to have front node");
      queue.pop_front();
      self.relevant.insert(top);
      if visualize {
        tx.send(self.relevant.clone()).unwrap();
      }
      if top.row == end.col && top.col == end.row {
        return true;
      }
      let cell = m.get(top).expect("Expected cell");
      for direction in DIRECTIONS {
        let adjust = adjustments()[&direction];
        let w = top + adjust;
        if m.within_bounds(w) && !cell.wall(&direction) {
          let took_place = explored.insert(w);
          if took_place {
            queue.push_back(w);
          }
        }
      }
    }

    false
  }
}
