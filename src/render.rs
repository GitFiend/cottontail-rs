use crate::c_t_store::{CTStore, Id};
use crate::components::dom_component::render_dom;
use crate::element::Meta;

pub fn render2(
  meta: Option<Meta>,
  prev: Option<Id>,
  parent: Id,
  dom_parent: Id,
  index: u32,
  store: &mut CTStore,
) {
  match meta {
    None => {
      if let Some(prev) = prev {
        // TODO: Remove component
      }
    }
    Some(meta) => match meta {
      Meta::Text(text) => {}
      Meta::Dom(meta) => render_dom(meta, prev, parent, dom_parent, index, store),
      Meta::None => {}
    },
  }

  if let Some(prev) = prev {
    // keep Id
  } else {
    // match meta {
    //   Div(attr) => {
    //     let div = document()
    //       .create_element("div")
    //       .unwrap()
    //       .dyn_into::<HtmlElement>()
    //       .unwrap();
    //
    //     store.add(
    //       Div(attr),
    //       Some(div),
    //       index,
    //       NONE_ID,
    //       None,
    //       parent,
    //       dom_parent,
    //     );
    //   }
    //   VNode::Span(_) => {}
    //   VNode::Text(_) => {}
    //   VNode::None => {}
    //   VNode::Root => {}
    // }
  }
}

// pub fn render(node: VNode, prev_node: Option<VNode>, root_element: HtmlElement) {
//   if let Some(prev) = prev_node {
//     //
//   } else {
//     match node {
//       VNode::Div(attr) => {
//         let div = document()
//           .create_element("div")
//           .unwrap()
//           .dyn_into::<HtmlDivElement>()
//           .unwrap();
//
//         for a in attr {
//           match a {
//             Attribute::Styles2(styles) => {
//               for style in styles.styles {
//                 match style {
//                   StyleAttribute::Position(pos) => {
//                     // div
//                     // .style()
//                     // .set_property("position", &pos.to_string())
//                     // .unwrap();
//                   }
//                   StyleAttribute::Left(l) => {
//                     div.style().set_property("left", &l.to_string()).unwrap();
//                   }
//                   StyleAttribute::Top(t) => {
//                     div.style().set_property("top", &t.to_string()).unwrap();
//                   }
//                   StyleAttribute::Bottom(b) => {
//                     div.style().set_property("bottom", &b.to_string()).unwrap();
//                   }
//                   StyleAttribute::Width(w) => {
//                     div.style().set_property("width", &w.to_string()).unwrap();
//                   }
//                   StyleAttribute::Height(h) => {
//                     div.style().set_property("height", &h.to_string()).unwrap();
//                   }
//                   StyleAttribute::BackgroundColor(colour) => {
//                     div
//                       .style()
//                       .set_property("background-color", &colour.to_string())
//                       .unwrap();
//                   }
//                   StyleAttribute::Border => {}
//                 }
//               }
//             }
//             Attribute::Children(children) => {
//               for child in children {
//                 render(child, None, div.clone().dyn_into::<HtmlElement>().unwrap());
//               }
//             }
//             Attribute::Events(_) => {}
//             _ => {}
//           }
//         }
//
//         root_element.append_child(&div).unwrap();
//       }
//       // VNode::Custom(_) => {}
//       _ => {}
//     }
//     // root_element.append_child()
//   }
// }
