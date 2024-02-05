use crate::c_t_store::{CTStore, CtStore2, Id};
use crate::component::order::NodeOrder;
use crate::element::{Attribute, DomMeta, Meta};
use crate::util::js_helpers::document;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub struct DomComponent {
  pub element_kind: &'static str,
  // TODO: Attr
  //  We should be able to keep all attr except children/subnodes.
  //  They will be moved to the owning components.
  pub element: HtmlElement,
  pub key: String,
  pub order: NodeOrder,
  pub index: usize,
  pub sibling: Option<Id>,
  pub inserted: Vec<Id>,
  pub direct_parent: Id,
  pub dom_parent: Id,
}

impl DomComponent {
  // TODO
  // fn new(meta: DomMeta, parent: Id, dom_parent: Id, index: u32) -> Option<Self> {
  //   let el = document()
  //     .create_element(meta.name)
  //     .unwrap()
  //     .dyn_into::<HtmlElement>()
  //     .ok()?;
  //
  //   // meta.
  //
  //   Self {
  //     element_kind: meta.name,
  //     element: el,
  //     key, meta.key
  //   }
  // }

  pub fn render(
    meta: DomMeta,
    prev: Option<Id>,
    parent: Id,
    dom_parent: Id,
    index: usize,
    store: &mut CtStore2,
  ) {
    if let Some(prev) = prev {
      //
    } else {
    }
  }
}

// TODO: Return type.
pub fn render_dom(
  meta: DomMeta,
  prev: Option<Id>,
  parent: Id,
  dom_parent: Id,
  index: u32,
  store: &mut CTStore,
) {
  if prev.is_none() {
    make_dom_component(meta, parent, dom_parent, index, store);
  }
}

pub fn make_dom_component(
  meta: DomMeta,
  parent: Id,
  dom_parent: Id,
  index: u32,
  store: &mut CTStore,
) {
  let el = document()
    .create_element(meta.name)
    .unwrap()
    .dyn_into::<HtmlElement>()
    .ok();

  for a in &meta.attr {
    match a {
      Attribute::SubNodes(nodes) => {
        for node in nodes {
          // TODO: Tricky problem where sub-nodes are owned by parent
          //  and need to be passed to render?
          // render2(Some(*node))
        }
      }
      Attribute::Styles2(_) => {}
      Attribute::Events(_) => {}
      Attribute::Key(_) => {}
    }
  }

  store.add(Meta::Dom(meta), el, index, None, None, parent, dom_parent);
}

// IDEAS:
// pub enum Node {
//   Div(Div),
//   Span(Span),
//   Custom(Box<dyn CustomComponent>),
// }
//
// fn test() {
//   // let s = span().style([Position(Absolute)]);
// }
//
// #[macro_export]
// macro_rules! basic_dom_elements {
//   ($struct_name: ident, $func_name: ident) => {
//     pub struct $struct_name {
//       pub styles: Vec<StyleAttribute>,
//       pub children: Vec<VNode>,
//     }
//
//     impl $struct_name {
//       pub fn new() -> Self {
//         Self {
//           styles: Vec::new(),
//           children: Vec::new(),
//         }
//       }
//
//       pub fn render(&self) -> VNode {
//         todo!()
//       }
//
//       pub fn style<const N: usize>(mut self, style: [StyleAttribute; N]) -> Self {
//         self.styles.extend(style);
//         self
//       }
//
//       pub fn children<const N: usize>(self, children: [Node; N]) -> Self {
//         todo!()
//       }
//     }
//
//     pub fn $func_name<const N: usize>() -> Node {
//       Node::$struct_name($struct_name::new())
//     }
//   };
// }
//
// basic_dom_elements!(Div, div);
// basic_dom_elements!(Span, span);
