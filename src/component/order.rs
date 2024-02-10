#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct NodeOrder {
  location: Vec<usize>,
}

impl NodeOrder {
  pub fn default() -> Self {
    Self { location: vec![1] }
  }

  pub fn none() -> Self {
    Self { location: vec![] }
  }

  pub fn next(&self, index: usize) -> Self {
    let mut location = self.location.clone();
    location.push(index);

    Self { location }
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
