use crate::style::StyleTrait;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Position {
  // Assume px units unless otherwise set?
  pub left: i32,
  pub top: i32,
  pub bottom: i32,
  pub right: i32,
  pub position_type: PositionType,
}

impl StyleTrait for Position {
  fn to_js_string(&self) -> js_sys::JsString {
    js_sys::JsString::from(format!(
      "position: {}; left: {}px; top: {}px; bottom: {}px; right: {}px;",
      self.position_type, self.left, self.top, self.bottom, self.right
    ))
  }
}

impl Position {
  pub fn new() -> Position {
    Position {
      left: 0,
      top: 0,
      bottom: 0,
      right: 0,
      position_type: PositionType::Static,
    }
  }

  pub fn absolute(mut self) -> Self {
    self.position_type = PositionType::Absolute;
    self
  }

  pub fn relative(mut self) -> Self {
    self.position_type = PositionType::Relative;
    self
  }

  pub fn fixed(mut self) -> Self {
    self.position_type = PositionType::Fixed;
    self
  }

  pub fn left(mut self, left: i32) -> Self {
    self.left = left;
    self
  }

  pub fn top(mut self, top: i32) -> Self {
    self.top = top;
    self
  }

  pub fn bottom(mut self, bottom: i32) -> Self {
    self.bottom = bottom;
    self
  }

  pub fn right(mut self, right: i32) -> Self {
    self.right = right;
    self
  }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PositionType {
  Static,
  Absolute,
  Relative,
  Fixed,
}

impl fmt::Display for PositionType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      PositionType::Static => write!(f, "static"),
      PositionType::Absolute => write!(f, "absolute"),
      PositionType::Relative => write!(f, "relative"),
      PositionType::Fixed => write!(f, "fixed"),
    }
  }
}

fn test() {
  let p = Position::new().absolute().left(10).top(10);
}
