use crate::component::parent::ParentAttr;
use crate::component::{ElementComponent, ParentDomComponent};
use std::rc::Rc;

pub struct OrderAttr {
  pub order: String,
  pub key: String,
  pub index: u32,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct NodeOrder {
  location: Vec<u32>,
}

pub fn apply_inserts(parent: ParentDomComponent) {
  let ParentAttr {
    inserted,
    siblings,
    element,
  } = match parent {
    ParentDomComponent::Root(c) => &c.parent_attr,
    ParentDomComponent::Dom(c) => &c.parent_attr,
  };

  let last = inserted.len() - 1;
  let mut next: Option<Rc<ElementComponent>> = None;

  for i in last..=0 {
    let current = &inserted[i];

    if next.is_none() {
      // current.element;

      // let element = match *current {
      //   ElementComponent::Dom(c) => c.child_attr
      // };
      // if !siblings.iter().find(|(a, b)| {
      //
      // }) {
      //
      // }
      // if siblings.get(current.element) {}
    }

    next = Some(current.clone());
  }
}

pub fn insert(parent: &mut ParentDomComponent, child: &ElementComponent) {
  let ParentAttr { inserted, .. } = match parent {
    ParentDomComponent::Root(c) => &c.parent_attr,
    ParentDomComponent::Dom(c) => &c.parent_attr,
  };

  let OrderAttr { order, key, .. } = match child {
    ElementComponent::Text(c) => &c.order_attr,
    ElementComponent::Dom(c) => &c.order_attr,
  };

  let last = inserted.len() - 1;

  for i in last..=0 {
    //
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
