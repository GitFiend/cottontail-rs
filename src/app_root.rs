use crate::element::{children, div, VNode};
use crate::style::position::Position;
use crate::style::styles;

pub fn app_root() -> VNode {
  div([children([toolbar()])])
}

fn toolbar() -> VNode {
  div([styles().position(Position::new()).into()])
}
