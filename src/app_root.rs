use crate::element::{children, div, Meta};
use crate::style::position::Position;
use crate::style::styles;

pub fn app_root() -> Meta {
  div([children([toolbar()])])
}

fn toolbar() -> Meta {
  div([styles().position(Position::new()).into()])
}
