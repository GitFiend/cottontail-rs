use std::cell::UnsafeCell;
use std::fmt;
use std::ops::{Deref, DerefMut, Shl, ShlAssign};

pub enum Style {
  BackgroundColor(String),
  Position(Position),
  Left(f64),
  Top(f64),
  Bottom(f64),
  Width(f64),
  Height(f64),
}

pub enum Position {
  Absolute,
  Relative,
  Fixed,
}

impl fmt::Display for Position {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Position::Absolute => write!(f, "absolute"),
      Position::Relative => write!(f, "relative"),
      Position::Fixed => write!(f, "fixed"),
    }
  }
}

struct MyStore {
  pub num: Rune<u32>,
  pub styles: Rune<Style>,
}

impl MyStore {
  pub fn new() -> Self {
    Self {
      num: Rune::new(0),
      styles: Rune::new(Style::BackgroundColor("red".to_string())),
    }
  }

  fn do_stuff() {
    let MyStore { num, styles } = MyStore::new();
    let value = num.get();

    num.set(value + 1);

    styles.set(Style::BackgroundColor("blue".to_string()));
  }
}

struct Rune<T> {
  value: UnsafeCell<T>,
}

impl<T> Rune<T> {
  pub fn new(value: T) -> Rune<T> {
    Rune {
      value: UnsafeCell::new(value),
    }
  }

  pub fn set(&self, value: T) {
    unsafe {
      *self.value.get() = value;
    }
  }

  pub fn get(&self) -> &T {
    unsafe { &*self.value.get() }
  }
}

impl<T> ShlAssign<T> for Rune<T> {
  fn shl_assign(&mut self, rhs: T) {
    self.set(rhs);
  }
}

impl<T> Shl<T> for Rune<T> {
  type Output = ();

  fn shl(self, rhs: T) -> Self::Output {
    self.set(rhs);
  }
}

impl<T> Deref for Rune<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    self.get()
  }
}

impl<T> DerefMut for Rune<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { &mut *self.value.get() }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[allow(clippy::no_effect)]
  #[test]
  fn test_get_and_set() {
    let rune = Rune::new(0);
    rune.set(1);
    let value = rune.get();

    assert_eq!(*value, 1);
    assert_eq!(*rune, 1);

    rune.set(4);

    assert_eq!(*rune, 4);
  }

  #[test]
  fn test_deref_assign() {
    let mut rune = Rune::new(0);
    *rune = 5;

    assert_eq!(*rune, 5);
  }

  #[test]
  fn test_assign() {
    let mut rune = Rune::new(0);
    rune <<= 5;

    assert_eq!(*rune, 5);
  }
}
