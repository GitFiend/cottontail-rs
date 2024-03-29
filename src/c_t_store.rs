use std::collections::VecDeque;

use crate::component::order::NodeOrder;
use crate::components::Component;
use web_sys::HtmlElement;

use crate::element::Meta;

// TODO: Consider having id specific to each component type for type safety.
pub type Id = usize;
pub const NONE_ID: Id = 0;

pub struct CtStore2 {
  components: Vec<Component>,

  inserts_stack: Vec<Id>,

  // TODO
  pub deleted: Vec<bool>,
  pub next_id: Id,
  pub recycled_ids: VecDeque<Id>,
}

impl CtStore2 {
  pub fn add(&mut self, component: Component) {}

  pub fn get(&self, id: Id) -> Option<&Component> {
    self.components.get(id)
  }

  pub fn get_inserted(&self, id: Id) -> Option<Vec<Id>> {
    match self.get(id) {
      Some(Component::Dom(c)) => Some(c.inserted.clone()),
      Some(Component::Text(_)) => None,
      None => None,
    }
  }

  pub fn insert(&mut self, parent_id: Id, child_id: Id) -> Option<()> {
    let child = self.get(child_id)?;
    let new_order = child.get_order();
    let new_key = child.get_key();

    if let Some(mut inserted) = self.get_inserted(parent_id) {
      for (i, id) in inserted.iter().rev().enumerate() {
        let c = &self.components[*id];
        let order = c.get_order();

        if new_order >= order {
          if new_key != c.get_key() {
            inserted.insert(i, child_id);
            self.add_to_inserts(parent_id);
          }
          return Some(());
        }
      }
    }

    Some(())
  }

  pub fn insert2(&mut self, parent_id: Id, child_id: Id) -> Option<()> {
    let child = self.get(child_id)?;
    let new_order = child.get_order();
    let new_key = child.get_key();

    if let Some(mut inserted) = self.get_inserted(parent_id) {
      for (i, id) in inserted.iter().rev().enumerate() {
        let c = &self.components[*id];
        let order = c.get_order();

        if new_order >= order {
          if new_key != c.get_key() {
            inserted.insert(i, child_id);
            self.add_to_inserts(parent_id);
          }
          return Some(());
        }
      }
    }

    Some(())
  }

  // TODO: Confirm working and tidy up.
  pub fn apply_inserts(&mut self, parent_id: Id) -> Option<()> {
    // If the last element isn't already inserted, insert it.
    // TODO: Can we do this without the contains check?
    if let Some(inserted) = self.get_inserted(parent_id) {
      if let Some(last_id) = inserted.last() {
        if let Some(last) = self.get(*last_id) {
          if let Some(parent) = self.get(parent_id) {
            if let Some(el) = last.get_element_node() {
              let parent_node = parent.get_element_node()?;
              if !parent_node.contains(Some(&el)) {
                parent_node.insert_before(&el, None).ok()?;
              }
            }
          }
        }
      }

      for ids in inserted.windows(2).rev() {
        let a_id = ids[0];
        let b_id = ids[1];

        if let Some(component) = self.components.get(b_id) {
          let prev_id = component.get_prev_sibling();

          if prev_id.is_none() || prev_id.unwrap() != a_id {
            self.insert_before(parent_id, a_id, b_id);
          }
        }
      }
    }

    Some(())
  }

  fn insert_before(&mut self, parent_id: Id, a_id: Id, b_id: Id) -> Option<()> {
    let parent = &self.components[parent_id].get_element_node()?;

    let a_node = &self.components[a_id].get_element_node()?;
    let b_component = &mut self.components[b_id];
    b_component.set_sibling(a_id);

    parent
      .insert_before(a_node, Some(&b_component.get_element_node()?))
      .ok()?;

    Some(())
  }

  // fn get_sibling(&self, id: Id) -> Option<Id> {
  //   match self.get(id) {
  //     Some(Component::Dom(c)) => c.sibling,
  //     Some(Component::Text(_)) => None,
  //     None => None,
  //   }
  // }

  pub fn add_to_inserts(&mut self, id: Id) {
    if !self.inserts_stack.contains(&id) {
      self.inserts_stack.push(id);
    }
  }
}

// TODO: Do we need to go this far into data-oriented(?) style?
//  Don't we just need to decouple references between nodes using IDs?
// Update yes, we probably do due to how wierd everything is.
pub struct CTStore {
  pub kind: Vec<Meta>,
  pub element: Vec<Option<HtmlElement>>,
  pub key: Vec<String>,
  pub order: Vec<NodeOrder>,

  // Index is the element id, value is the previous sibling element id.
  pub sibling: Vec<Id>,

  // Index is the parent, and inserted are the ordered child elements.
  pub inserted: Vec<Option<Vec<Id>>>,

  pub direct_parent: Vec<Id>,
  pub dom_parent: Vec<Id>,

  // TODO
  pub deleted: Vec<bool>,
  pub next_id: Id,
  pub recycled_ids: VecDeque<Id>,
}

impl CTStore {
  pub fn new(root_element: HtmlElement) -> Self {
    // Root is position 1. 0 is always empty.
    // Do we need anything for Root?
    CTStore {
      kind: vec![Meta::None, Meta::None],
      element: vec![None, Some(root_element)],
      key: vec![String::default(), String::from("r")],
      order: vec![NodeOrder::none() /* NodeOrder::new_root() */],
      sibling: vec![NONE_ID, NONE_ID],
      inserted: vec![None, None],
      direct_parent: vec![NONE_ID, NONE_ID],
      dom_parent: vec![NONE_ID, NONE_ID],
      deleted: vec![true, false],
      next_id: 2,
      recycled_ids: Default::default(),
    }
  }

  // Consider passing in a struct.
  #[allow(clippy::too_many_arguments)]
  pub fn add(
    &mut self,
    kind: Meta,
    element: Option<HtmlElement>,
    index: u32,
    sibling: Option<Id>,
    inserted: Option<Vec<Id>>,
    direct_parent: Id,
    dom_parent: Id,
  ) -> Id {
    // TODO: Check for reusable ids first.

    let key = kind
      .get_key()
      .unwrap_or_else(|| format!("{}{}", self.key[direct_parent], index));

    self.kind.push(kind);
    self.element.push(element);
    self.key.push(key);
    self.order.push(self.order[direct_parent].next(index));
    if let Some(sibling) = sibling {
      self.sibling.push(sibling);
    }
    self.inserted.push(inserted);
    self.direct_parent.push(direct_parent);
    self.dom_parent.push(dom_parent);

    let id = self.next_id;
    self.next_id += 1;

    id
  }

  // Need to ensure we don't go out of array bounds.
  fn next_id(&mut self) -> Id {
    let id = self.next_id;
    self.next_id += 1;
    id
  }

  pub fn insert(&mut self, parent: Id, child: Id) {
    let order = &self.order[child];
    let key = &self.key[child];

    if let Some(inserted) = &mut self.inserted[parent] {
      for (i, current) in inserted.iter().rev().cloned().enumerate() {
        let next = inserted.get(i + 1);

        /*
        If order is the same we expect the keys to be different. This
        is expected for a virtual list.
         */
        if order >= &self.order[current] {
          if key != &self.key[current] {
            if next.is_some() {
              inserted.insert(i, child);
              self.apply_inserts(parent);
            } else {
              inserted.push(child);
              self.apply_inserts(parent);
            }
          }

          return;
        }
      }
      inserted.insert(0, child);
      self.apply_inserts(parent);
    }
  }

  fn apply_inserts(&mut self, parent: Id) -> Option<()> {
    let parent_element = self.element[parent].as_ref()?;
    let mut next: Option<Id> = None;

    for current in self.inserted[parent].as_mut()?.iter().rev().cloned() {
      let current_element = self.element[current].as_ref().unwrap();

      match next {
        None => {
          if self.sibling[current] == NONE_ID {
            parent_element.insert_before(current_element, None).unwrap();
          }
        }
        Some(next) => {
          if self.sibling[next] != NONE_ID && self.sibling[next] != current {
            parent_element
              .insert_before(
                current_element,
                Some(&**self.element[next].as_ref().unwrap()),
              )
              .unwrap();
            self.sibling[next] = current;
          }
        }
      }

      next = Some(current);
    }

    Some(())
  }

  pub fn move_component(&mut self, parent: Id, child: Id) {
    self.remove_from_inserted(parent, child);

    self.insert(parent, child);
  }

  pub fn remove(&mut self, parent: Id, child: Id) {
    self.remove_from_inserted(parent, child);

    if let Some(child) = &self.element[child] {
      child.remove();
    }
    self.sibling[child] = NONE_ID;
  }

  fn remove_from_inserted(&mut self, parent: Id, child: Id) {
    if let Some(inserted) = &mut self.inserted[parent] {
      let key = &self.key[child];
      if let Some(i) = inserted.iter().position(|ins| self.key[*ins] == *key) {
        inserted.remove(i);
      }
    }
  }
}
