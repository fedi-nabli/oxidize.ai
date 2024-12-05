use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Matrix<T = f64> {
  pub rows: usize,
  pub cols: usize,
  pub data: Vec<T>
}

impl<T> Matrix<T> {
  pub fn new(rows: usize, cols: usize) -> Self {
    Matrix {
      rows,
      cols,
      data: Vec::with_capacity(rows * cols)
    }
  }

  pub fn zeroes(rows: usize, cols: usize) -> Self
  where
    T: Clone + Default
  {
    Matrix {
      rows,
      cols,
      data: vec![T::default(); rows * cols]
    }
  }

  pub fn ones(rows: usize, cols: usize) -> Self
  where
    T: Clone + From<i32>
  {
    Matrix {
      rows,
      cols,
      data: vec![T::from(1); rows * cols]
    }
  }

  pub fn identity(size: usize) -> Self
  where
    T: Clone + From<i32> + Default
  {
    let mut matrix = Self::zeroes(size, size);
    for i in 0..size {
      matrix[(i, i)] = T::from(1);
    }

    matrix
  }

  pub fn get(&self, row: usize, col: usize) -> Option<&T> {
    if row >= self.rows || col >= self.cols {
      return None;
    }

    Some(&self[(row, col)])
  }

  pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
    if row >= self.rows || col >= self.cols {
      return None;
    }

    Some(&mut self[(row, col)])
  }

  pub fn set(&mut self, row: usize, col: usize, value: T) -> Result<(), String> {
    if row >= self.rows || col >= self.cols {
      return Err("Index out of bounds".to_string());
    }

    self[(row, col)] = value;
    Ok(())
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
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for i in 0..self.rows {
      for j in 0..self.cols {
        write!(f, "{}", self[(i, j)])?;
      }
      writeln!(f)?;
    }
    Ok(())
  }
}