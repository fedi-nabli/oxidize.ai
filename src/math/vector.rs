use std::ops::Add;

#[derive(Clone, PartialEq)]
pub struct Vector<T = f64> {
  pub data: Vec<T>
}

impl<T> Vector<T> {
  pub fn new() -> Self {
    Vector {
      data: Vec::new()
    }
  }

  pub fn with_capacity(capacity: usize) -> Self {
    Vector {
      data: Vec::with_capacity(capacity)
    }
  }

  pub fn from_elem(elem: T, len: usize) -> Self
  where
    T: Clone
  {
    Vector {
      data: vec![elem; len]
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

  pub fn set(&mut self, index: usize, value: T) -> Result<(), String> {
    if index >= self.len() {
      return Err("Index is out of bounds".to_string());
    }

    self.data[index] = value;
    Ok(())
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

    Vector {
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
      return None;
    }

    Some(self.sum().into() / self.len() as f64)
  }

  pub fn min(&self) -> Option<T>
  where
    T: PartialOrd + Copy
  {
    self.data.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).cloned()
  }

  pub fn max(&self) -> Option<T>
  where
    T: PartialOrd + Copy
  {
    self.data.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).cloned()
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

  pub fn map<F, U>(&self, f: F) -> Vector<U>
  where
    F: Fn(&T) -> U
  {
    Vector {
      data: self.data.iter().map(f).collect()
    }
  }

  pub fn zip_map<F, U>(&self, other: &Self, f: F) -> Vector<U>
  where
    F: Fn(&T, &T) -> U
  {
    Vector {
      data: self.data.iter().zip(other.data.iter()).map(|(a, b)| f(a, b)).collect()
    }
  }
}