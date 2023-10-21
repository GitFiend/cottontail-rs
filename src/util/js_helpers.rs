use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Window};

pub fn window() -> Window {
  web_sys::window().expect("no global `window` exists")
}

pub fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
  window()
    .request_animation_frame(f.as_ref().unchecked_ref())
    .expect("should register `requestAnimationFrame` OK");
}

pub fn document() -> web_sys::Document {
  window()
    .document()
    .expect("should have a document on window")
}

pub fn body() -> web_sys::HtmlElement {
  document().body().expect("document should have a body")
}

pub fn get_canvas() -> Option<HtmlCanvasElement> {
  document()
    .get_element_by_id("canvas")?
    .dyn_into::<HtmlCanvasElement>()
    .ok()
}

pub fn get_2d_context(canvas: &HtmlCanvasElement) -> Option<CanvasRenderingContext2d> {
  canvas
    .get_context("2d")
    .ok()??
    .dyn_into::<CanvasRenderingContext2d>()
    .ok()
}
