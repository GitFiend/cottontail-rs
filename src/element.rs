use crate::style::Style;

pub enum Event {
  Click,
  MouseMove,
}

pub enum VNode {
  Div(Vec<Attribute>),
  Span(Vec<Attribute>),
  Custom(Box<dyn CustomComponent>),
}

pub enum Attribute {
  Children(Vec<VNode>),
  Styles(Vec<Style>),
  Events(Event),
}

#[macro_export]
macro_rules! use_view {
  () => {
    use $crate::element::Event::*;
    use $crate::style::Position::*;
    use $crate::style::Style::*;
  };
}

pub fn span<const N: usize>(attr: [Attribute; N]) -> VNode {
  VNode::Span(Vec::from(attr))
}

pub fn children<const N: usize>(c: [VNode; N]) -> Attribute {
  Attribute::Children(Vec::from(c))
}

pub fn style<const N: usize>(s: [Style; N]) -> Attribute {
  Attribute::Styles(Vec::from(s))
}

pub fn div<const N: usize>(attr: [Attribute; N]) -> VNode {
  VNode::Div(Vec::from(attr))
}

#[macro_export]
macro_rules! comp {
  ($c: expr) => {
    VNode::Custom(Box::new($c))
  };
}

#[macro_export]
macro_rules! on_click {
  ($c: expr) => {{
    let _ = $c;
    Attribute::Events(Click)
  }};
}

pub trait CustomComponent {
  fn render(&mut self) -> VNode;
}

pub struct Numbers {}
impl Numbers {
  fn get(self) -> VNode {
    VNode::Custom(Box::new(self))
  }
}
impl CustomComponent for Numbers {
  fn render(&mut self) -> VNode {
    span([])
  }
}

pub struct App {
  n: i32,
}
impl CustomComponent for App {
  fn render(&mut self) -> VNode {
    use_view!();

    span([
      style([Position(Absolute), Width(12.), Height(21.)]),
      children([span([]), div([]), Numbers {}.get()]),
      on_click! {self.n += 1},
    ])
  }
}
