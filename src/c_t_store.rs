use std::collections::VecDeque;

use web_sys::HtmlElement;

use crate::component::order::NodeOrder;

pub const NONE: usize = 0;

#[derive(Default)]
pub struct CTStore {
  pub element: Vec<HtmlElement>,
  pub key: Vec<String>,
  pub order: Vec<NodeOrder>,

  // Index is the element, value is the previous element.
  pub siblings: Vec<usize>,
  pub inserted: Vec<Option<Vec<usize>>>,

  pub direct_parent: Vec<usize>,
  pub dom_parent: Vec<usize>,

  pub deleted: Vec<usize>,
  pub next_id: usize,
  pub recycled_ids: VecDeque<usize>,
}
