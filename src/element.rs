use crate::component::CustomComponent;
use crate::style::position::Position;
use crate::style::{styles, Styles};

#[derive(Clone)]
pub enum Event {
  Click,
  MouseMove,
}

pub enum VNode {
  Div(Vec<Attribute>),
  Span(Vec<Attribute>),
  None, // Custom(Box<dyn CustomComponent>),
  Root,
}

pub enum Attribute {
  Children(Vec<VNode>),
  Styles2(Styles),
  Events(Event),
}

#[macro_export]
macro_rules! use_view {
  () => {
    use $crate::element::Event::*;
    // use $crate::style::Position::*;
    use $crate::style::Style::*;
  };
}

pub fn span<const N: usize>(attr: [Attribute; N]) -> VNode {
  VNode::Span(Vec::from(attr))
}

pub fn children<const N: usize>(c: [VNode; N]) -> Attribute {
  Attribute::Children(Vec::from(c))
}

#[macro_export]
macro_rules! div {
  ( $($item:expr),* ) => {{
     $crate::element::VNode::Div(Vec::from([
       $($item,)*
     ]))
  }};
}

macro_rules! style {
  (position: $v: expr;) => {
    format!("position: {}", $v)
  };
}

fn test() {
  let s = style! {
    position: 35;
  };
}

#[macro_export]
macro_rules! children {
  ( $($item:expr),* ) => {{
     $crate::element::Attribute::Children(Vec::from([
       $($item,)*
     ]))
  }};
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

// pub trait CustomComponent {
//   fn render(&mut self) -> VNode;
//   fn select_state(&mut self);
// }

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

// impl CustomComponent for Numbers {
//   fn render(&mut self) -> VNode {
//     span([])
//   }
//
//   fn select_state(&mut self) {
//     self.state = NumbersState {
//       numbers: Vec::new(),
//     }
//   }
// }

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
    span([
      styles().position(Position::new()).background("blue").into(),
      children([
        span([]),
        div([]),
        div([]),
        div([]),
        div([]),
        // custom!(Numbers::new()),
      ]),
      // on_click! {self.n += 1},
    ])
  }

  // fn select_state(&mut self) {
  //   self.state = AppState { on: false };
  // }
}
