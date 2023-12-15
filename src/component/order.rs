use crate::c_t_store::{CTStore, NONE};

pub struct OrderAttr {
  pub order: String,
  pub key: String,
  pub index: u32,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct NodeOrder {
  location: Vec<u32>,
}

impl NodeOrder {
  pub fn new_root() -> Self {
    Self { location: vec![1] }
  }

  pub fn next(&self, parent: &NodeOrder, index: u32) -> Self {
    let mut location = parent.location.clone();
    location.push(index);

    Self { location }
  }
}

// TODO: Figure out the rules for inserting and removing from CTStore!

pub fn move_component(parent: usize, child: usize, store: &mut CTStore) {
  let inserted = &mut store.inserted[parent];
  let key = &store.key[child];

  if let Some(inserted) = inserted {
    if let Some(i) = inserted.iter().position(|ins| store.key[*ins] == *key) {
      // TODO
      inserted.remove(i);
    }
  }

  insert_component(parent, child, store);
}

pub fn remove_component(parent: usize, child: usize, store: &mut CTStore) {
  let inserted = &mut store.inserted[parent];
  let key = &store.key[child];

  if let Some(inserted) = inserted {
    if let Some(i) = inserted.iter().position(|ins| store.key[*ins] == *key) {
      // TODO
      store.siblings.remove(i);
    }
  }
}

pub fn insert_component(parent: usize, child: usize, store: &mut CTStore) {
  let order = &store.order[child];
  let key = &store.key[child];

  if let Some(inserted) = &mut store.inserted[parent] {
    for (i, current) in inserted.iter().rev().cloned().enumerate() {
      let next = inserted.get(i + 1);

      /*
      If order is the same we expect the keys to be different. This
      is expected for a virtual list.
       */
      if order >= &store.order[current] {
        if key != &store.key[current] {
          if next.is_some() {
            inserted.insert(i, child);
            apply_inserts(parent, store);
          } else {
            // TODO: Is this in the wrong order?
            inserted.push(child);
            apply_inserts(parent, store);
          }
        }

        return;
      }
    }
    inserted.insert(0, child);
    apply_inserts(parent, store);
  }
}

fn apply_inserts(parent: usize, store: &mut CTStore) {
  let parent_element = &store.element[parent];
  let mut next: Option<usize> = None;

  if let Some(inserted) = &mut store.inserted[parent] {
    for current in inserted.iter().rev().cloned() {
      let current_element = &store.element[current];

      match next {
        None => {
          if store.siblings[current] == NONE {
            parent_element.insert_before(current_element, None).unwrap();
          }
        }
        Some(next) => {
          if store.siblings[next] != NONE && store.siblings[next] != current {
            parent_element
              .insert_before(&store.element[current], Some(&**store.element[next]))
              .unwrap();
            store.siblings[next] = current;
          }
        }
      }
      next = Some(current);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_node_order() {
    let a = NodeOrder {
      location: vec![1, 2, 3],
    };
    let b = NodeOrder {
      location: vec![1, 2, 3],
    };
    let c = NodeOrder {
      location: vec![1, 2, 4],
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
    assert!(a < c);
    assert!(c > a);
  }
}
