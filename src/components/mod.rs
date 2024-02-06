use crate::c_t_store::Id;
use crate::component::order::NodeOrder;
use crate::components::dom_component::DomComponent;
use crate::components::text_component::TextComponent;

pub mod dom_component;
pub mod text_component;

pub enum Component {
  Dom(DomComponent),
  Text(TextComponent),
}

impl Component {
  pub fn get_id(&self) -> Id {
    match self {
      Component::Dom(dom) => dom.id,
      Component::Text(text) => text.id,
    }
  }

  pub fn get_key(&self) -> &str {
    match self {
      Component::Dom(dom) => &dom.key,
      Component::Text(text) => &text.key,
    }
  }

  pub fn get_order(&self) -> &NodeOrder {
    match self {
      Component::Dom(dom) => &dom.order,
      Component::Text(text) => &text.order,
    }
  }

  pub fn get_sibling(&self) -> Option<Id> {
    match self {
      Component::Dom(dom) => dom.sibling,
      Component::Text(text) => text.sibling,
    }
  }
}
