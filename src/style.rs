use std::fmt;

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
