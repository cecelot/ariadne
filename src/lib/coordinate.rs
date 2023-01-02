use serde::{Deserialize, Serialize};
use std::ops::Add;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct Coordinates {
  pub row: usize,
  pub col: usize,
}

impl Coordinates {
  pub fn new(row: usize, col: usize) -> Self {
    Self { row, col }
  }
}

impl From<Coordinates> for (usize, usize) {
  fn from(coordinates: Coordinates) -> (usize, usize) {
    (coordinates.row, coordinates.col)
  }
}

impl From<(usize, usize)> for Coordinates {
  fn from(t: (usize, usize)) -> Coordinates {
    Coordinates { row: t.0, col: t.1 }
  }
}

impl Add<(isize, isize)> for Coordinates {
  type Output = Coordinates;

  fn add(self, rhs: (isize, isize)) -> Self::Output {
    let (row_adjust, col_adjust) = rhs;
    let row = ((self.row as isize) + row_adjust) as usize;
    let col = ((self.col as isize) + col_adjust) as usize;

    Self::Output { row, col }
  }
}
