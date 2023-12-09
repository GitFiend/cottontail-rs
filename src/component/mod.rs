mod order;

use crate::component::order::NodeOrder;
use crate::element::{div, VNode};
use std::collections::HashMap;
use std::rc::Rc;
use web_sys::{Element, HtmlElement, Text};

pub enum AnyComponent {
  Custom(Box<dyn CustomComponent>),
  Dom(DomComponent),
  Root(RootComponent),
  Element(ElementComponent),
}

pub enum SubComponent {
  Custom(Box<dyn CustomComponent>),
  Dom(DomComponent),
  Element(ElementComponent),
}

pub enum ParentDomComponent {
  Root(RootComponent),
  Dom(DomComponent),
}

pub enum ParentComponent {
  Root(RootComponent),
  Dom(DomComponent),
  Custom(Box<dyn CustomComponent>),
}

struct RootComponent {
  pub element: HtmlElement,
  pub order: NodeOrder,
  pub key: String,
  pub inserted: Vec<ElementComponent>,
  pub siblings: HashMap<Element, Element>,
}

struct DomComponent {
  pub element: HtmlElement,
  pub order: NodeOrder,
  pub key: String,
  pub index: u32,
  pub inserted: Vec<ElementComponent>,
  pub siblings: HashMap<Element, Element>,
  pub sub_components: HashMap<String, SubComponent>,
}

struct TextComponent {
  pub element: Text,
  pub index: u32,
}

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

// impl CustomMeta {
//   pub fn new() -> Self {
//     let n: u32 = 43;
//     let c = char::from_u32(n).unwrap();
//
//     Self {
//       component: Box::new(MyComponent { num: 3 }),
//       direct_parent: Rc::new(ParentComponent::Root(RootComponent {order})),
//       dom_parent: Rc::new(DomComponent {}),
//       order: "before".to_string(),
//       key: "key".to_string(),
//     }
//   }
// }

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
