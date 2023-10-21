use wasm_bindgen::prelude::*;

mod element;
mod render;
mod style;
pub(crate) mod util;

#[wasm_bindgen(start)]
pub fn start() {
  console_log!("hello")
}
