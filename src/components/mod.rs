use crate::components::dom_component::DomComponent;
use crate::components::text_component::TextComponent;

pub mod dom_component;
pub mod text_component;

pub enum Component {
  Dom(DomComponent),
  Text(TextComponent),
}
