use crate::component::{ElementComponent, ParentDomComponent};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct NodeOrder {
  location: Vec<u32>,
}

pub fn insert(parent: &ParentDomComponent, child: &ElementComponent) {}

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
