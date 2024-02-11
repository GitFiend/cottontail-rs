use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::c_t_store::{CTStore, ComponentInfo, Id, NONE_ID};
use crate::console_log;
use crate::element::{Attribute, DomMeta};
use crate::util::js_helpers::document;

// Like DomMeta, with sub nodes and key removed.
#[derive(Debug)]
pub struct DomComponentInfo {
  pub element_kind: &'static str,
  pub attr: Vec<Attribute>,
}

// TODO: Return type.
pub fn render_dom(
  meta: DomMeta,
  prev: Option<Id>,
  parent: Id,
  dom_parent: Id,
  index: usize,
  store: &mut CTStore,
) -> Id {
  if prev.is_none() {
    return make_dom_component(meta, parent, dom_parent, index, store);
  }

  NONE_ID
}

pub fn make_dom_component(
  meta: DomMeta,
  parent_id: Id,
  dom_parent_id: Id,
  index: usize,
  store: &mut CTStore,
) -> Id {
  let el = document()
    .create_element(meta.name)
    .unwrap()
    .dyn_into::<HtmlElement>()
    .ok();

  let key = meta
    .key
    .unwrap_or_else(|| format!("{}-{}", store.key[parent_id], index));

  let id = store.add(
    ComponentInfo::Dom(DomComponentInfo {
      element_kind: meta.name,
      attr: meta.attr,
    }),
    el,
    key,
    index,
    parent_id,
    dom_parent_id,
    Some(Vec::new()),
    Some(HashMap::new()),
  );

  if let Some(el) = &store.element[id] {
    el.set_id(&id.to_string());
  }

  store.insert(parent_id, id);
  store.render_subcomponents(id, id, meta.sub_nodes);

  id
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
