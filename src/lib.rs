use crate::c_t_store::CTStore;
use crate::element::div;
use crate::util::js_helpers::document;
use wasm_bindgen::prelude::*;

mod app_root;
mod c_t_store;
pub(crate) mod component;
pub mod components;
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

  let mut store = CTStore::new(root);

  store.render(Some(div([])), None, 1, 1, 0);
}
