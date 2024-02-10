use crate::c_t_store::{CTStore, Id, NONE_ID};
use crate::components::dom_component::render_dom;
use crate::element::Meta;
use std::collections::HashMap;

impl CTStore {
  pub fn render(
    &mut self,
    meta: Option<Meta>,
    prev: Option<Id>,
    parent: Id,
    dom_parent: Id,
    index: usize,
  ) -> Id {
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

    id
  }

  pub fn render_subcomponents(
    &mut self,
    parent_id: Id,
    dom_parent: Id,
    sub_nodes: Vec<Meta>,
  ) {
    if sub_nodes.is_empty() {
      if let Some(inserted) = self.inserted[dom_parent].clone() {
        for id in inserted {
          self.remove(parent_id, id);
        }
        self.inserted[parent_id] = None;
      }
      return;
    }

    let mut new_components = HashMap::new();

    for (i, meta) in sub_nodes.into_iter().rev().enumerate() {
      if meta != Meta::None {
        self.render_subcomponent(meta, parent_id, dom_parent, i, &mut new_components);
      }
    }

    if let Some(s) = self.sub_components[parent_id].clone() {
      for id in s.values() {
        self.remove(parent_id, *id);
      }
    }

    self.sub_components[parent_id] = Some(new_components);
  }

  pub fn render_subcomponent(
    &mut self,
    meta: Meta,
    parent_id: Id,
    dom_parent: Id,
    index: usize,
    new_components: &mut HashMap<String, Id>,
  ) -> Id {
    let key = meta.get_key(index);
    let prev = self.sub_components[parent_id]
      .as_ref()
      .and_then(|sc| sc.get(&key).cloned());

    let id = self.render(Some(meta), prev, parent_id, dom_parent, index);

    if let Some(sub_components) = &mut self.sub_components[parent_id] {
      sub_components.remove(&key);
    }

    new_components.insert(key, id);

    id
  }
}
