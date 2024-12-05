use std::fmt;
use std::ops::{Index, IndexMut, Add, Sub, Mul};

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

impl<T> Add for Matrix<T>
where
  T: Add<Output = T> + Copy
{
  type Output = Result<Self, String>;

  fn add(self, rhs: Self) -> Self::Output {
    if self.rows != rhs.rows || self.cols != rhs.cols {
      return Err("Cannot add 2 matrices with incompatible dimensions".to_string());
    }

    let new_data = self.data
      .iter()
      .zip(rhs.data.iter())
      .map(|(a, b)| *a + *b)
      .collect();

    Ok(Self {
      rows: self.rows,
      cols: self.cols,
      data: new_data
    })
  }
}

impl<T> Sub for Matrix<T>
where
  T: Sub<Output = T> + Copy
{
  type Output = Result<Self, String>;

  fn sub(self, rhs: Self) -> Self::Output {
    if self.rows != rhs.rows || self.cols != rhs.cols {
      return Err("Cannot substract 2 matrices with incompatible matrices".to_string());
    }

    let new_data = self.data
      .iter()
      .zip(rhs.data.iter())
      .map(|(a, b)| *a - *b)
      .collect();

    Ok(Self {
      rows: self.rows,
      cols: self.cols,
      data: new_data
    })
  }
}

impl<T> Mul for Matrix<T>
where
  T: Mul<Output = T> + Add<Output = T> + Copy + Default
{
  type Output = Result<Self, String>;

  fn mul(self, rhs: Self) -> Self::Output {
    if self.cols != rhs.rows {
      return Err("Cannot multiply matrices".to_string());
    }

    let mut new_data = vec![T::default(); self.rows * rhs.cols];

    for i in 0..self.rows {
      for j in 0..rhs.cols {
        new_data[i * rhs.cols + j] = (0..self.cols)
          .map(|k| self[(i, k)] * rhs[(k, j)])
          .fold(T::default(), |acc, x| acc + x);
      }
    }

    Ok(Self {
      rows: self.rows,
      cols: rhs.cols,
      data: new_data
    })
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