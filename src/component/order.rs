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

// pub fn move_component(parent: Id, child: Id, store: &mut CTStore) {
//   if let Some(inserted) = &mut store.inserted[parent] {
//     let key = &store.key[child];
//     if let Some(i) = inserted.iter().position(|ins| store.key[*ins] == *key) {
//       inserted.remove(i);
//     }
//   }
//
//   insert_component(parent, child, store);
// }
//
// pub fn remove_component(parent: Id, child: Id, store: &mut CTStore) {
//   if let Some(inserted) = &mut store.inserted[parent] {
//     let key = &store.key[child];
//     if let Some(i) = inserted.iter().position(|ins| store.key[*ins] == *key) {
//       inserted.remove(i);
//     }
//   }
//
//   if let Some(child) = &store.element[child] {
//     child.remove();
//   }
//   store.sibling[child] = NONE_ID;
// }
//
// pub fn insert_component(parent: Id, child: Id, store: &mut CTStore) {
//   let order = &store.order[child];
//   let key = &store.key[child];
//
//   if let Some(inserted) = &mut store.inserted[parent] {
//     for (i, current) in inserted.iter().rev().cloned().enumerate() {
//       let next = inserted.get(i + 1);
//
//       /*
//       If order is the same we expect the keys to be different. This
//       is expected for a virtual list.
//        */
//       if order >= &store.order[current] {
//         if key != &store.key[current] {
//           if next.is_some() {
//             inserted.insert(i, child);
//             apply_inserts(parent, store);
//           } else {
//             // TODO: Is this line in the wrong order?
//             inserted.push(child);
//             apply_inserts(parent, store);
//           }
//         }
//
//         return;
//       }
//     }
//     inserted.insert(0, child);
//     apply_inserts(parent, store);
//   }
// }
//
// // TODO: Return error instead of option.
// fn apply_inserts(parent: Id, store: &mut CTStore) -> Option<()> {
//   let parent_element = store.element[parent].as_ref()?;
//   let mut next: Option<Id> = None;
//
//   for current in store.inserted[parent].as_mut()?.iter().rev().cloned() {
//     let current_element = store.element[current].as_ref().unwrap();
//
//     match next {
//       None => {
//         if store.sibling[current] == NONE_ID {
//           parent_element.insert_before(current_element, None).unwrap();
//         }
//       }
//       Some(next) => {
//         if store.sibling[next] != NONE_ID && store.sibling[next] != current {
//           parent_element
//             .insert_before(
//               current_element,
//               Some(&**store.element[next].as_ref().unwrap()),
//             )
//             .unwrap();
//           store.sibling[next] = current;
//         }
//       }
//     }
//
//     next = Some(current);
//   }
//
//   Some(())
// }

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
