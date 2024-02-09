pub(crate) mod border;
pub(crate) mod position;

use crate::element::Attribute;
use crate::style::position::Position;
use js_sys::JsString;
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut, Shl, ShlAssign};

pub trait StyleTrait {
  fn to_js_string(&self) -> JsString;
}

struct MyStore {
  pub num: Rune<u32>,
}

impl MyStore {
  pub fn new() -> Self {
    Self { num: Rune::new(0) }
  }

  fn do_stuff() {
    let MyStore { num } = MyStore::new();
    let value = num.get();

    num.set(value + 1);
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

#[derive(Clone, Debug)]
pub struct Styles {
  pub styles: Vec<StyleAttribute>,
}
impl Styles {
  pub fn new() -> Styles {
    Styles { styles: Vec::new() }
  }

  pub fn background(mut self, background: &str) -> Self {
    self
      .styles
      .push(StyleAttribute::BackgroundColor(background.to_string()));
    self
  }
  pub fn position(mut self, position: Position) -> Self {
    self.styles.push(StyleAttribute::Position(position));
    self
  }
  pub fn border(mut self) -> Self {
    self
  }
}

impl From<Styles> for Attribute {
  fn from(styles: Styles) -> Attribute {
    Attribute::Styles2(styles)
  }
}

#[derive(Clone, Debug)]
pub enum StyleAttribute {
  BackgroundColor(String),
  Position(Position),
  Left(f64),
  Top(f64),
  Bottom(f64),
  Width(f64),
  Height(f64),
  Border,
}

pub fn styles() -> Styles {
  Styles { styles: Vec::new() }
}

fn test() {
  let style = styles().position(Position::new()).background("blue");
}
