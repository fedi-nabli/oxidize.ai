use std::fmt;
use std::ops::{Index, IndexMut, Add, Sub, Mul};

use super::vector::Vector;

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

  pub fn from_vec(rows: usize, cols: usize, data: Vec<T>) -> Result<Self, String> {
    if data.len() != rows * cols {
      return Err("Data length does not match specified dimensions.".to_string());
    }

    Ok(Self {
      rows,
      cols,
      data
    })
  }

  pub fn from_rows(rows: Vec<Vector<T>>) -> Result<Self, String> {
    if rows.is_empty() {
      return Err("Cannot create matrix from empty vector of rows".to_string());
    }

    let num_rows = rows.len();
    let num_cols = rows[0].len();

    if !rows.iter().all(|row| row.len() == num_cols) {
      return Err("All rows must be the same length".to_string());
    }

    let data: Vec<T> = rows.into_iter().flat_map(|row| row.data).collect();

    Ok(Self {
      rows: num_rows,
      cols: num_cols,
      data
    })
  }

  pub fn from_columns(cols: Vec<Vector<T>>) -> Result<Self, String>
  where
    T: Clone + Default
  {
    if cols.is_empty() {
      return Err("Cannot create matrix from empty vector of colmuns".to_string());
    }

    let num_rows = cols[0].len();
    let num_cols = cols.len();

    if !cols.iter().all(|col| col.len() == num_rows) {
      return Err("All columns must be the same length".to_string());
    }

    let mut data = vec![T::default(); num_rows * num_cols];
    for (col_idx, column) in cols.into_iter().enumerate() {
      for (row_idx, value) in column.data.into_iter().enumerate() {
        data[row_idx * num_cols + col_idx] = value;
      }
    }

    Ok(Self {
      rows: num_rows,
      cols: num_cols,
      data
    })
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

impl<T> Matrix<T>
where
  T: Clone
{
  pub fn row(&self, index: usize) -> Option<Vector<T>> {
    if index >= self.rows {
      None
    } else {
      let start = index * self.cols;
      let end = start + self.cols;
      Some(Vector::from(self.data[start..end].to_vec()))
    }
  }

  pub fn column(&self, index: usize) -> Option<Vector<T>> {
    if index >= self.cols {
      None
    } else {
      let column_data: Vec<T> = (0..self.rows)
        .map(|row| self[(row, index)].clone())
        .collect();

      Some(Vector::from(column_data))
    }
  }

  pub fn transpose(&self) -> Self {
    let mut transposed_data = Vec::with_capacity(self.rows * self.cols);
    for col in 0..self.cols {
      for row in 0..self.rows {
        transposed_data.push(self[(row, col)].clone());
      }
    }

    Matrix {
      rows: self.cols,
      cols: self.rows,
      data: transposed_data
    }
  }

  pub fn reshape(&self, new_rows: usize, new_cols: usize) -> Result<Self, String> {
    if self.rows * self.cols != new_rows * new_cols {
      return Err("Cannot reshape matrix".to_string());
    }

    Ok(Self {
      rows: new_rows,
      cols: new_cols,
      data: self.data.clone()
    })
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

impl<T> Matrix<T>
where
  T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy + Default
{
  pub fn scalar_multiply(&self, scalar: T) -> Self {
    Matrix {
      rows: self.rows,
      cols: self.cols,
      data: self.data.iter().map(|&x| x * scalar).collect()
    }
  }

  pub fn dot(&self, other: &Self) -> Result<T, String> {
    if self.rows != other.rows || self.cols != other.cols {
      return Err("Matrices must have the same dimensions for dot product".to_string());
    }

    Ok(self.data
      .iter()
      .zip(other.data.iter())
      .map(|(a, b)| *a * *b)
      .fold(T::default(), |acc, x| acc + x)
    )
  }

  pub fn hadamard_product(&self, other: &Self) -> Result<Self, String> {
    if self.rows != other.rows || self.cols != other.cols {
      return Err("Matrices must have the same dimensions for Hadamard product".to_string());
    }

    let new_data = self.data
      .iter()
      .zip(other.data.iter())
      .map(|(a, b)| *a * *b)
      .collect();

    Ok(Self {
      rows: self.rows,
      cols: self.cols,
      data: new_data
    })
  }

  pub fn trace(&self) -> Result<T, String> {
    if self.rows != self.cols {
      return Err("Matrix must be square to compute trace".to_string());
    }

    Ok(
      (0..self.rows)
        .map(|i| self[(i, i)])
        .fold(T::default(), |acc, x| acc + x)
    )
  }
}

impl<T> Matrix<T>
where
  T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy + Default + PartialEq + std::fmt::Debug
{
  pub fn determinant(&self) -> Result<T, String> {
    if self.rows != self.cols {
      return Err("Matrix must be square to computer determinant".to_string());
    }

    let n = self.rows;
    if n == 1 {
      return Ok(self[(0, 0)]);
    }

    if n == 2 {
      return Ok(self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)]);
    }

    let mut det = T::default();
    for j in 0..n {
      let mut submatrix = Vec::with_capacity((n - 1) * (n -1));
      for i in 1..n {
        for k in 0..n {
          if k != j {
            submatrix.push(self[(i, k)]);
          }
        }
      }

      let subdet = Matrix { rows: n - 1, cols: n - 1, data: submatrix }.determinant()?;
      if j % 2 == 0 {
        det = det + self[(0, j)] * subdet;
      } else {
        det = det - self[(0, j)] * subdet;
      }
    }

    Ok(det)
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