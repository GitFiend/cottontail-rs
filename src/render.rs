use crate::element::{Attribute, VNode};
use crate::style::Style;
use crate::util::js_helpers::document;
use wasm_bindgen::JsCast;
use web_sys::{HtmlDivElement, HtmlElement};

pub fn render(node: VNode, prev_node: Option<VNode>, root_element: HtmlElement) {
  if let Some(prev) = prev_node {
    //
  } else {
    match node {
      VNode::Div(attr) => {
        let div = document()
          .create_element("div")
          .unwrap()
          .dyn_into::<HtmlDivElement>()
          .unwrap();

        for a in attr {
          match a {
            Attribute::Styles(styles) => {
              for style in styles {
                match style {
                  Style::Position(pos) => {
                    div
                      .style()
                      .set_property("position", &pos.to_string())
                      .unwrap();
                  }
                  Style::Left(l) => {
                    div.style().set_property("left", &l.to_string()).unwrap();
                  }
                  Style::Top(t) => {
                    div.style().set_property("top", &t.to_string()).unwrap();
                  }
                  Style::Bottom(b) => {
                    div.style().set_property("bottom", &b.to_string()).unwrap();
                  }
                  Style::Width(w) => {
                    div.style().set_property("width", &w.to_string()).unwrap();
                  }
                  Style::Height(h) => {
                    div.style().set_property("height", &h.to_string()).unwrap();
                  }
                  Style::BackgroundColor(colour) => {
                    div
                      .style()
                      .set_property("background-color", &colour.to_string())
                      .unwrap();
                  }
                }
              }
            }
            Attribute::Children(children) => {
              for child in children {
                render(child, None, div.clone().dyn_into::<HtmlElement>().unwrap());
              }
            }
            Attribute::Events(_) => {}
          }
        }

        root_element.append_child(&div).unwrap();
      }
      VNode::Custom(_) => {}
      _ => {}
    }
    // root_element.append_child()
  }
}
