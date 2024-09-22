use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug)]
pub struct Matrix<T = f64> {
  pub rows: usize,
  pub cols: usize,
  pub data: Vec<T>
}

impl<T> Matrix<T> {
  pub fn new(rows: usize, cols: usize) -> Self {
    Self {
      rows,
      cols,
      data: Vec::with_capacity(rows * cols)
    }
  }

  pub fn zeros(rows: usize, cols: usize) -> Self
  where
    T: Clone + Default
  {
    Self {
      rows,
      cols,
      data: vec![T::default(); rows * cols]
    }
  }

  pub fn ones(rows: usize, cols: usize) -> Self
  where
    T: Clone + From<i32>
  {
    Self {
      rows,
      cols,
      data: vec![T::from(1); rows * cols]
    }
  }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
  type Output = T;

  fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
    &self.data[row * self.cols + col]
  }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
  fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
    &mut self.data[row * self.cols + col]
  }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for i in 0..self.rows {
      for j in 0..self.cols {
        write!(f, "{} ", self[(i, j)])?;
      }
      writeln!(f)?;
    }
    Ok(())
  }
}
