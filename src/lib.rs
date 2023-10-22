use crate::element::{div, style};
use crate::render::render;
use crate::style::Style::{BackgroundColor, Height, Width};
use wasm_bindgen::prelude::*;

mod dom_component;
mod element;
mod render;
mod style;
pub(crate) mod util;

#[wasm_bindgen(start)]
pub fn start() {
  console_log!("hello");

  let root = util::js_helpers::document()
    .get_element_by_id("root")
    .unwrap()
    .dyn_into::<web_sys::HtmlElement>()
    .unwrap();

  /*

  div().style([
    Width(100.),
    Height(100.),
    BackgroundColor("red")
  ]).children([
    div(),
    span()
  ])

   */

  render(
    div([style([
      Width(100.),
      Height(100.),
      BackgroundColor("red".to_string()),
    ])]),
    None,
    root,
  );
}

struct Something {
  things: Vec<u32>,
}

impl Something {
  pub fn new() -> Self {
    Self { things: Vec::new() }
  }
  pub fn attr<const N: usize>(&mut self, a: [u32; N]) {
    self.things.extend(a);
  }
}

fn test() {
  let mut s = Something::new();
  s.attr([32, 42]);

  let mut s = Something::new();
  s.attr([32, 42, 54]);
}
