use crate::util::js_helpers::document;
use wasm_bindgen::prelude::*;
use web_sys::HtmlDivElement;

mod app_root;
mod c_t_store;
pub(crate) mod component;
mod dom_component;
mod element;
mod render;
mod style;
pub(crate) mod util;

#[wasm_bindgen(start)]
pub fn start() {
  console_log!("hello");

  let root = document()
    .get_element_by_id("root")
    .unwrap()
    .dyn_into::<web_sys::HtmlElement>()
    .unwrap();

  let div = document()
    .create_element("div")
    .unwrap()
    .dyn_into::<HtmlDivElement>()
    .unwrap();

  let div2 = div.clone();
  let equal = div2 == div;

  console_log!("div: {:?}, equal: {equal}", div);

  // render(
  //   div!(
  //     // style!(
  //     //   Width(100.),
  //     //   Height(100.),
  //     //   BackgroundColor("red".to_string())
  //     // ),
  //     // children![
  //     //   div!(style!(
  //     //     Left(0.),
  //     //     Top(10.),
  //     //     Bottom(10.),
  //     //     Width(50.),
  //     //     Height(50.),
  //     //     BackgroundColor("blue".to_string())
  //     //   )),
  //     //   div!(style!(
  //     //     Width(50.),
  //     //     Height(50.),
  //     //     BackgroundColor("green".to_string())
  //     //   ))
  //     // ]
  //   ),
  //   None,
  //   root,
  // );
}
