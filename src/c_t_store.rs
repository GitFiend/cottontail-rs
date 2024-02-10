use std::collections::{HashMap, VecDeque};

use crate::component::order::NodeOrder;
use crate::components::dom_component::DomComponentInfo;
use crate::console_log;
use web_sys::HtmlElement;

// TODO: Consider having id specific to each component type for type safety.
pub type Id = usize;
pub const NONE_ID: Id = 0;

#[derive(Debug)]
pub enum ComponentInfo {
  Dom(DomComponentInfo),
  Text,
  None,
}

// TODO: Do we need to go this far into data-oriented(?) style?
//  Don't we just need to decouple references between nodes using IDs?
// Update yes, we probably do due to how wierd everything is.
#[derive(Debug)]
pub struct CTStore {
  pub kind: Vec<ComponentInfo>,
  pub element: Vec<Option<HtmlElement>>,
  pub key: Vec<String>,
  pub order: Vec<NodeOrder>,

  // Index is the element id, value is the previous sibling element id.
  pub sibling: Vec<Id>,

  // Index is the parent, and inserted are the ordered child elements.
  // Optional as not all components can have subcomponents.
  pub inserted: Vec<Option<Vec<Id>>>,
  pub sub_components: Vec<Option<HashMap<String, Id>>>,

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
      kind: vec![ComponentInfo::None, ComponentInfo::None],
      element: vec![None, Some(root_element)],
      key: vec![String::default(), String::from("root")],
      order: vec![NodeOrder::none(), NodeOrder::default()],
      sibling: vec![NONE_ID, NONE_ID],
      direct_parent: vec![NONE_ID, NONE_ID],
      dom_parent: vec![NONE_ID, NONE_ID],
      inserted: vec![None, Some(Vec::new())],
      sub_components: vec![None, Some(HashMap::new())],
      deleted: vec![true, false],
      next_id: 2,
      recycled_ids: Default::default(),
    }
  }

  pub fn print(&self) {
    console_log!("{:?}", self);
  }

  #[allow(clippy::too_many_arguments)]
  pub fn add(
    &mut self,
    kind: ComponentInfo,
    element: Option<HtmlElement>,
    key: String,
    index: usize,
    direct_parent: Id,
    dom_parent: Id,
    inserted: Option<Vec<Id>>,
    sub_components: Option<HashMap<String, Id>>,
  ) -> Id {
    // TODO: Check for reusable ids first.

    self.kind.push(kind);
    self.element.push(element);
    self.key.push(key);
    self.order.push(self.order[direct_parent].next(index));
    self.sibling.push(NONE_ID);
    self.direct_parent.push(direct_parent);
    self.dom_parent.push(dom_parent);
    self.inserted.push(inserted);
    self.sub_components.push(sub_components);
    self.deleted.push(false);

    let id = self.next_id;
    self.next_id += 1;

    id
  }

  // TODO: Push to stack of inserts to do later, rather than immediately.
  pub fn insert(&mut self, parent: Id, child: Id) {
    let order = &self.order[child];
    let key = &self.key[child];

    self.print();

    if let Some(inserted) = &mut self.inserted[parent] {
      for (i, current) in inserted.iter().rev().cloned().enumerate() {
        let next = inserted.get(i + 1);

        // If order is the same, we expect the keys to be different.
        // This is expected for a virtual list.
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

      inserted.push(child);
      self.apply_inserts(parent);
    }
  }

  fn apply_inserts(&mut self, parent: Id) -> Option<()> {
    let parent_element = self.element[parent].as_ref()?;
    let mut next: Option<Id> = None;

    for current in self.inserted[parent].as_mut()?.iter().rev().cloned() {
      let current_element = self.element[current].as_ref().unwrap();

      match next {
        // This means current is the last element.
        None => {
          // If current has no sibling, it may not have been inserted.
          if self.sibling[current] == NONE_ID {
            // Insert at the end
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

    debug_assert_eq!(
      parent_element.child_element_count() as usize,
      self.inserted[parent].as_ref().unwrap().len()
    );

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
