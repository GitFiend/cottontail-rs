use crate::c_t_store::CTStore;
use crate::element::{children, div, span};
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
  let root = document()
    .get_element_by_id("root")
    .unwrap()
    .dyn_into::<web_sys::HtmlElement>()
    .unwrap();

  let mut store = CTStore::new(root);

  let view = div([children([
    div([children([
      div([]),
      span([]),
      div([]),
      span([]),
      div([]),
      span([]),
      div([]),
      span([]),
      div([]),
    ])]),
    span([]),
  ])]);

  let id = store.render(Some(view), None, 1, 1, 0);

  store.apply_inserts();

  // let view = div([children([
  //   div([children([div([]), span([]), div([])])]),
  //   span([]),
  // ])]);
  // store.render(Some(view), Some(id), 1, 1, 0);

  store.render(None, Some(id), 1, 1, 0);

  store.apply_inserts();
}
