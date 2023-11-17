use crate::render::render;
use wasm_bindgen::prelude::*;

mod app_root;
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

  render(
    div!(
      style!(
        Width(100.),
        Height(100.),
        BackgroundColor("red".to_string())
      ),
      children![
        div!(style!(
          Left(0.),
          Top(10.),
          Bottom(10.),
          Width(50.),
          Height(50.),
          BackgroundColor("blue".to_string())
        )),
        div!(style!(
          Width(50.),
          Height(50.),
          BackgroundColor("green".to_string())
        ))
      ]
    ),
    None,
    root,
  );
}
