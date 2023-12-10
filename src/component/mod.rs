use std::rc::Rc;

use web_sys::Text;

use crate::component::order::OrderAttr;
use crate::component::parent::{DomComponent, ParentDomComponent, RootComponent};
use crate::element::{div, VNode};

mod order;
mod parent;
mod trait_style;

pub enum AnyComponent {
  Custom(Box<dyn CustomComponent>),
  Dom(DomComponent),
  Root(RootComponent),
  Text(TextComponent),
}

pub struct ChildAttr {
  pub direct_parent: Rc<ParentDomComponent>,
  pub dom_parent: Rc<DomComponent>,
}

pub enum SubComponent {
  // Custom(Box<dyn CustomComponent>),
  Dom(DomComponent),
  Element(ElementComponent),
}

struct CustomMeta {
  // TODO: Props, __ref
  pub child_attr: ChildAttr,
  pub order_attr: OrderAttr,
  pub component: Box<dyn CustomComponent>,
  pub sub_component: Option<SubComponent>,
}

struct TextComponent {
  pub order_attr: OrderAttr,
  pub child_attr: ChildAttr,
  pub element: Text,
}

enum ElementComponent {
  Text(TextComponent),
  Dom(DomComponent),
}

pub trait CustomComponent {
  fn render(&mut self) -> VNode;
}

struct MyComponent {
  pub num: i32,
}

fn try_thing() {
  let a: AnyComponent = MyComponent { num: 3 }.into();
}

impl CustomComponent for MyComponent {
  fn render(&mut self) -> VNode {
    let Self { num } = self;

    println!("{num}");

    div([])
  }
}

impl<T> From<T> for AnyComponent
where
  T: CustomComponent + 'static,
{
  fn from(c: T) -> AnyComponent {
    AnyComponent::Custom(Box::new(c))
  }
}
