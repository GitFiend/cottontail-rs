pub(crate) mod border;
pub(crate) mod position;

use crate::element::Attribute;
use crate::style::position::Position;
use js_sys::JsString;

pub trait StyleTrait {
  fn to_js_string(&self) -> JsString;
}

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
