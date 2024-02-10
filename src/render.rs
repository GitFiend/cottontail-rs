use crate::c_t_store::{CTStore, Id, NONE_ID};
use crate::components::dom_component::render_dom;
use crate::element::Meta;

impl CTStore {
  pub fn render(
    &mut self,
    meta: Option<Meta>,
    prev: Option<Id>,
    parent: Id,
    dom_parent: Id,
    index: u32,
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
        Meta::Dom(meta) => render_dom(meta, prev, parent, dom_parent, index, self),
        Meta::None => NONE_ID,
      },
    };

    if let Some(prev) = prev {
      // keep Id
    } else {
    }
  }

  pub fn render_subcomponents(
    &mut self,
    parent_id: Id,
    dom_parent: Id,
    sub_nodes: Vec<Meta>,
  ) {
    if sub_nodes.is_empty() {
      if let Some(inserted) = self.inserted[parent_id].clone() {
        for id in inserted {
          self.remove(parent_id, id);
        }
        self.inserted[parent_id] = None;
      }
      return;
    }

    for (i, meta) in sub_nodes.into_iter().rev().enumerate() {
      self.render_subcomponent(meta, parent_id, dom_parent, i);
    }
  }

  pub fn render_subcomponent(
    &mut self,
    meta: Meta,
    parent_id: Id,
    dom_parent: Id,
    index: usize,
  ) {
  }
}
