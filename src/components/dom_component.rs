use crate::c_t_store::{CTStore, ComponentInfo, CtStore2, Id, NONE_ID};
use crate::component::order::NodeOrder;
use crate::element::{Attribute, DomMeta};
use crate::util::js_helpers::document;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

// Like DomMeta, with sub nodes and key removed.
#[derive(Debug)]
pub struct DomComponentInfo {
  pub element_kind: &'static str,
  pub attr: Vec<Attribute>,
}

pub struct DomComponent {
  pub element_kind: &'static str,
  pub id: Id,
  // TODO: Attr
  //  We should be able to keep all attr except children/subnodes.
  //  They will be moved to the owning components.
  pub element: HtmlElement,
  pub key: String,
  pub order: NodeOrder,
  pub index: usize,
  pub sub_components: HashMap<String, Id>,
  pub sibling: Option<Id>,
  // Id of the previous element. Rename?
  pub inserted: Vec<Id>,
  pub direct_parent: Id,
  pub dom_parent: Id,
}

impl DomComponent {
  // Dom component must always have a parent.
  // If creation of this fails, we there is probably something wrong in our design. We should panic/log.
  // TODO: Convert this to error return type.
  fn new(
    meta: DomMeta,
    id: Id,
    parent_id: Id,
    dom_parent_id: Id,
    index: u32,
    store: CtStore2,
  ) -> Option<Self> {
    let el = document()
      .create_element(meta.name)
      .unwrap()
      .dyn_into::<HtmlElement>()
      .ok()?;

    let parent = store.get(parent_id)?;

    Some(Self {
      element_kind: meta.name,
      id,
      element: el,
      key: meta
        .key
        .unwrap_or_else(|| format!("{}-{}", parent.get_key(), index)),
      order: parent.get_order().next(index),
      index: index as usize,
      sub_components: HashMap::new(),
      sibling: None,
      inserted: Vec::new(),
      direct_parent: parent_id,
      dom_parent: dom_parent_id,
    })
  }

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

fn calc_key(index: u32, parent: Id, store: CtStore2) -> String {
  if let Some(parent) = store.get(parent) {

    // return format!("{}-{}", parent.key, index);
  }
  index.to_string()
}

// TODO: Return type.
pub fn render_dom(
  meta: DomMeta,
  prev: Option<Id>,
  parent: Id,
  dom_parent: Id,
  index: u32,
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
  index: u32,
  store: &mut CTStore,
) -> Id {
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

  let key = meta
    .key
    .unwrap_or_else(|| format!("{}-{}", store.key[parent_id], index));

  let id = store.add(
    ComponentInfo::Dom(DomComponentInfo {
      element_kind: meta.name,
      attr: meta.attr,
    }),
    key,
    el,
    index,
    parent_id,
    dom_parent_id,
  );

  store.insert(parent_id, id);

  id
  // TODO: Subcomponents
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
