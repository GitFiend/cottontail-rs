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
macro_rules! custom {
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
  fn select_state(&mut self);
}

pub struct NumbersState {
  numbers: Vec<i32>,
}

pub struct Numbers {
  state: NumbersState,
}

impl Numbers {
  pub fn new() -> Self {
    Self {
      state: NumbersState {
        numbers: Vec::new(),
      },
    }
  }
}

fn test_mut() {
  let mut n = 4;

  a(&mut n);
  b(&mut n);
}

fn a(n: &mut i32) {
  *n += 1;
}

fn b(n: &mut i32) {
  *n += 1;
}

impl CustomComponent for Numbers {
  fn render(&mut self) -> VNode {
    span([])
  }

  fn select_state(&mut self) {
    self.state = NumbersState {
      numbers: Vec::new(),
    }
  }
}

pub trait State {}

pub struct AppState {
  on: bool,
}

pub struct App {
  n: i32,
  state: AppState,
}
impl CustomComponent for App {
  fn render(&mut self) -> VNode {
    use_view!();

    span([
      style([Position(Absolute), Width(12.), Height(21.)]),
      children([
        span([]),
        div([]),
        div([]),
        div([]),
        div([]),
        custom!(Numbers::new()),
      ]),
      on_click! {self.n += 1},
    ])
  }

  fn select_state(&mut self) {
    self.state = AppState { on: false };
  }
}
