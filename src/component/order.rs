use crate::c_t_store::{CTStore, NONE};
use crate::component::ParentDomComponent;

pub struct OrderAttr {
  pub order: String,
  pub key: String,
  pub index: u32,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct NodeOrder {
  location: Vec<u32>,
}

fn apply_inserts2(parent: usize, store: &mut CTStore) {
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

pub fn apply_inserts(parent: &ParentDomComponent) {
  // let ParentAttr {
  //   inserted,
  //   mut siblings,
  //   element: parent_element,
  // } = match parent {
  //   ParentDomComponent::Root(c) => c.parent_attr.as_ref(),
  //   ParentDomComponent::Dom(c) => c.parent_attr.as_ref(),
  // };

  // let last = inserted.len() - 1;
  // let mut next: Option<Rc<ElementComponent>> = None;
  //
  // for i in last..=0 {
  //   let current: Rc<ElementComponent> = inserted[i].clone();
  //
  //   let current_element = match current.as_ref() {
  //     ElementComponent::Text(c) => c.element.dyn_into().unwrap(),
  //     ElementComponent::Dom(c) => c.element,
  //   };
  //
  //   if let Some(next) = next {
  //     let next_element = match next.as_ref() {
  //       ElementComponent::Text(c) => &c.element.dyn_into().unwrap(),
  //       ElementComponent::Dom(c) => &c.element,
  //     };
  //
  //     if let Some(mut pair) = siblings.iter_mut().find(|(e, _)| e == next_element) {
  //       if pair.1.is_none() || pair.1.unwrap() != current_element {
  //         parent_element.insert_before(&current_element, Some(next_element));
  //         pair.1 = Some(current_element.clone());
  //       }
  //     }
  //   } else {
  //     if !siblings.iter().any(|(e, _)| *e == current_element) {
  //       parent_element.insert_before(&current_element, None);
  //       siblings.push((current_element.clone(), None));
  //     }
  //   }
  //
  //   next = Some(current.clone());
  // }
}

pub fn insert(parent: usize, child: usize, store: &mut CTStore) {
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
            apply_inserts2(parent, store);
          } else {
            // TODO: Is this in the wrong order?
            inserted.push(child);
            apply_inserts2(parent, store);
          }
        }

        return;
      }
    }
    inserted.insert(0, child);
    apply_inserts2(parent, store);
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
