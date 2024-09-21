use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub struct Vector<T = f64> {
  pub data: Vec<T>
}

impl<T> Vector<T> {
  pub fn len(&self) -> usize {
    self.data.len()
  }

  pub fn is_empty(&self) -> bool {
    self.data.is_empty()
  }
}

impl<T> Add for Vector<T>
where
  T: Add<Output = T> + Copy,
{
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    let data: Vec<T> = self
      .data
      .iter()
      .zip(rhs.data.iter())
      .map(|(a, b)| *a + *b)
      .collect();
    
    Self {
      data
    }
  }
}

impl<T> AddAssign for Vector<T>
where
  T: AddAssign + Copy
{
  fn add_assign(&mut self, rhs: Self) {
    for (a, b) in self.data.iter_mut().zip(rhs.data.iter()) {
      *a += *b;
    }
  }
}

impl<T> Sub for Vector<T>
where
  T: Sub<Output = T> + Copy,
{
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    let data: Vec<T> = self
      .data
      .iter()
      .zip(rhs.data.iter())
      .map(|(a, b)| *a - *b)
      .collect();

    Self {
      data
    }
  }
}

impl<T> SubAssign for Vector<T>
where
  T: SubAssign + Copy
{
  fn sub_assign(&mut self, rhs: Self) {
    for (a, b) in self.data.iter_mut().zip(rhs.data.iter()) {
      *a -= *b;
    }
  }
}

impl<T> Mul for Vector<T>
where
  T: Mul<Output = T> + Copy
{
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    let data: Vec<T> = self
      .data
      .iter()
      .zip(rhs.data.iter())
      .map(|(a, b)| *a * *b)
      .collect();

    Self {
      data
    }
  }
}

impl<T> MulAssign for Vector<T>
where
  T: MulAssign + Copy
{
  fn mul_assign(&mut self, rhs: Self) {
    for (a, b) in self.data.iter_mut().zip(rhs.data.iter()) {
      *a *= *b;
    }
  }
}

impl<T> Div for Vector<T>
where
  T: Div<Output = T> + Copy
{
  type Output = Self;

  fn div(self, rhs: Self) -> Self::Output {
    let data: Vec<T> = self
      .data
      .iter()
      .zip(rhs.data.iter())
      .map(|(a, b)| *a / *b)
      .collect();

    Self {
      data
    }
  }
}

impl<T> DivAssign for Vector<T>
where
  T: DivAssign + Copy
{
  fn div_assign(&mut self, rhs: Self) {
    for (a, b) in self.data.iter_mut().zip(rhs.data.iter()) {
      *a /= *b;
    }
  }
}