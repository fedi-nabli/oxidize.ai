use std::{fmt, ops::Index};

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
