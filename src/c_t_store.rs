use std::collections::VecDeque;

use web_sys::HtmlElement;

use crate::component::order::NodeOrder;

pub type Id = usize;
pub const NONE_ID: Id = 0;

#[derive(Default)]
pub struct CTStore {
  pub element: Vec<Option<HtmlElement>>,
  pub key: Vec<Option<String>>,
  pub order: Vec<Option<NodeOrder>>,

  // Index is the element id, value is the previous sibling element id.
  pub sibling: Vec<Id>,

  // Index is the parent, and inserted are the ordered child elements.
  pub inserted: Vec<Option<Vec<Id>>>,

  pub direct_parent: Vec<Id>,
  pub dom_parent: Vec<Id>,

  pub deleted: Vec<Id>,
  pub next_id: Id,
  pub recycled_ids: VecDeque<Id>,
}

impl CTStore {
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
              // TODO: Is this line in the wrong order?
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
