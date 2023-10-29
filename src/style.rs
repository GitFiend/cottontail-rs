use crate::style::Position::Absolute;
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

pub struct Styles {
  styles: Vec<StyleAttribute>,
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
}

enum StyleAttribute {
  BackgroundColor(String),
  Position(Position),
  Left(f64),
  Top(f64),
  Bottom(f64),
  Width(f64),
  Height(f64),
}

pub fn styles() -> Styles {
  Styles { styles: Vec::new() }
}

fn test() {
  let style = styles().position(Absolute).background("blue");
}
