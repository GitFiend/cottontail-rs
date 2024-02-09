use crate::c_t_store::{CTStore, Id, NONE_ID};
use crate::components::dom_component::render_dom;
use crate::element::Meta;

pub fn render(
  meta: Option<Meta>,
  prev: Option<Id>,
  parent: Id,
  dom_parent: Id,
  index: u32,
  store: &mut CTStore,
) {
  let id = match meta {
    None => {
      if let Some(prev) = prev {
        // TODO: Remove component
      }
      NONE_ID
    }
    Some(meta) => match meta {
      Meta::Text(text) => NONE_ID,
      Meta::Dom(meta) => render_dom(meta, prev, parent, dom_parent, index, store),
      Meta::None => NONE_ID,
    },
  };

  if let Some(prev) = prev {
    // keep Id
  } else {
  }
}

pub fn render_sub_components(
  store: &mut CTStore,
  parent_id: Id,
  dom_parent: Id,
  sub_nodes: Vec<Meta>,
) {
  if sub_nodes.is_empty() {
    if let Some(sub_components) = store.sub_components[parent_id].clone() {
      for id in sub_components {
        store.remove(parent_id, id);
      }
      store.sub_components[parent_id] = None;
    }
    return;
  }

  //
}
