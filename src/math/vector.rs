use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone)]
pub struct Vector<T = f64> {
  pub data: Vec<T>
}

impl<T> From<Vec<T>> for Vector<T> {
  fn from(data: Vec<T>) -> Self {
    Self {
      data
    }
  }
}

impl<T> Vector<T> {
  pub fn new() -> Self {
    Self {
      data: vec![]
    }
  }

  pub fn len(&self) -> usize {
    self.data.len()
  }

  pub fn is_empty(&self) -> bool {
    self.data.is_empty()
  }

  pub fn equals(&self, other: &Self) -> bool
  where
    T: PartialEq
  {
    self.data == other.data
  }

  pub fn get(&self, index: usize) -> Option<&T> {
    self.data.get(index)
  }

  pub fn iter(&self) -> std::slice::Iter<'_, T> {
    self.data.iter()
  }

  pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
    self.data.iter_mut()
  }

  pub fn to_string(&self) -> String
  where
    T: std::fmt::Display
  {
    format!("[{}]", self.iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join(", "))
  }

  pub fn element_wise_apply<F>(&self, f: F) -> Self
  where
    F: Fn(T) -> T,
    T: Copy
  {
    let data: Vec<_> = self.data.iter().map(|x| f(*x)).collect();
    Self {
      data
    }
  }

  pub fn sum(&self) -> T
  where
    T: Add<Output = T> + Default + Clone
  {
    self.data.iter().cloned().fold(T::default(), |acc, x| acc + x)
  }

  pub fn mean(&self) -> Option<f64>
  where
    T: Add<Output = T> + Copy + Default + Into<f64>
  {
    if self.is_empty() {
      None
    } else {
      Some(self.sum().into() / self.len() as f64)
    }
  }

  pub fn max(&self) -> Option<T>
  where
    T: PartialOrd + Copy
  {
    self.data.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).cloned()
  }

  pub fn min(&self) -> Option<T>
  where
    T: PartialOrd + Copy
  {
    self.data.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).cloned()
  }

  pub fn to_array<const N: usize>(&self) -> Option<[T; N]>
  where
    T: Default + Copy
  {
    if self.len() == N {
      let mut arr = [T::default(); N];
      for (i, &item) in self.iter().enumerate() {
        arr[i] = item;
      }
      Some(arr)
    } else {
      None
    }
  }
}

impl<T> Add for Vector<T>
where
  T: Add<Output = T> + Copy,
{
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    let data: Vec<_> = self
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
    let data: Vec<_> = self
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
    let data: Vec<_> = self
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
    let data: Vec<_> = self
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

impl<T> Vector<T>
where
  T: Mul<Output = T> + Add<Output = T> + Default + Copy
{
  pub fn dot(&self, rhs: &Self) -> T {
    self.data
      .iter()
      .zip(rhs.data.iter())
      .map(|(a, b)| *a * *b)
      .fold(T::default(), |acc, x| acc + x)
  }
}

impl<T> Vector<T>
where
  T: Mul<Output = T> + Copy
{
  pub fn scalar_mul(&self, scalar: T) -> Self {
    let data: Vec<_> = self
      .data
      .iter()
      .map(|&x| x * scalar)
      .collect();

    Self {
      data
    }
  }
}

impl<T> Vector<T>
where
  T: Div<Output = T> + Copy
{
  pub fn scalar_div(&self, scalar: T) -> Self {
    let data: Vec<_> = self
      .data
      .iter()
      .map(|&x| x / scalar)
      .collect();

    Self {
      data
    }
  }
}

impl<T> Vector<T>
where
  T: Mul<Output = T> + Add<Output = T> + Copy + Default + PartialEq + From<f64> + Into<f64> + Div<Output = T>
{
  pub fn normalize(&self) -> Self {
    let length: f64 = self.dot(self).into();
    let length = length.sqrt();
    if length > 0.0 {
      self.scalar_div(T::from(length))
    } else {
      Self {
        data: vec![T::default(); self.len()]
      }
    }
  }
}

impl<T> Index<usize> for Vector<T> {
  type Output = T;

  fn index(&self, index: usize) -> &Self::Output {
    &self.data[index]
  }
}

impl<T> IndexMut<usize> for Vector<T> {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.data[index]
  }
}
