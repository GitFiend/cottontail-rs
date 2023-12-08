use crate::element::{Attribute, VNode};
use crate::style::StyleAttribute;
use crate::util::js_helpers::document;
use wasm_bindgen::JsCast;
use web_sys::{HtmlDivElement, HtmlElement};

struct CottonTail {
  // root:
}

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
            Attribute::Styles2(styles) => {
              for style in styles.styles {
                match style {
                  StyleAttribute::Position(pos) => {
                    // div
                    // .style()
                    // .set_property("position", &pos.to_string())
                    // .unwrap();
                  }
                  StyleAttribute::Left(l) => {
                    div.style().set_property("left", &l.to_string()).unwrap();
                  }
                  StyleAttribute::Top(t) => {
                    div.style().set_property("top", &t.to_string()).unwrap();
                  }
                  StyleAttribute::Bottom(b) => {
                    div.style().set_property("bottom", &b.to_string()).unwrap();
                  }
                  StyleAttribute::Width(w) => {
                    div.style().set_property("width", &w.to_string()).unwrap();
                  }
                  StyleAttribute::Height(h) => {
                    div.style().set_property("height", &h.to_string()).unwrap();
                  }
                  StyleAttribute::BackgroundColor(colour) => {
                    div
                      .style()
                      .set_property("background-color", &colour.to_string())
                      .unwrap();
                  }
                  StyleAttribute::Border => {}
                }
              }
            }
            Attribute::Children(children) => {
              for child in children {
                render(child, None, div.clone().dyn_into::<HtmlElement>().unwrap());
              }
            }
            Attribute::Events(_) => {}
            _ => {}
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
