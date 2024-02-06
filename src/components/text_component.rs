use crate::c_t_store::Id;
use crate::component::order::NodeOrder;
use web_sys::Text;

pub struct TextComponent {
  pub id: Id,
  pub text: String,
  pub element: Text,
  pub key: String,
  pub order: NodeOrder,
  pub index: usize,
  pub dom_parent: Id,
  // Id of the previous element. Rename?
  pub sibling: Option<Id>,
}
