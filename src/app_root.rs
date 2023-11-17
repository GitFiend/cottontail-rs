use crate::element::{children, div, VNode};
use crate::style::{styles, Position};

pub fn app_root() -> VNode {
  div([children([toolbar()])])
}

fn toolbar() -> VNode {
  div([styles().position(Position::Absolute).into()])
}
