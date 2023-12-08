use crate::element::{div, VNode};
use std::rc::Rc;

trait Props {}

pub enum AnyComponent {
  Custom(Box<dyn CustomComponent>),
  Dom(DomComponent),
  Root(RootComponent),
  Element(ElementComponent),
}

pub enum ParentComponent {
  Root(RootComponent),
  Dom(DomComponent),
  Custom(Box<dyn CustomComponent>),
}

struct RootComponent {
  //
}

struct DomComponent {}

struct TextComponent {}

enum ElementComponent {
  Text(TextComponent),
  Dom(DomComponent),
}

trait CustomComponent {
  fn render(&mut self) -> VNode;
}

struct MyComponent {
  pub num: i32,
}

struct CustomMeta {
  pub component: Box<dyn CustomComponent>,
  pub direct_parent: Rc<ParentComponent>,
  pub dom_parent: Rc<DomComponent>,
  pub order: String,
  pub key: String,
}

impl CustomMeta {}

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
