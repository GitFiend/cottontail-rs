use crate::style::Styles;

#[derive(Clone)]
pub enum Event {
  Click,
  MouseMove,
}

pub enum NKind {
  Dom,
  Text,
  Root,
  None,
}

pub enum Meta {
  Dom(DomMeta),
  Text(String),
  None, // Custom(Box<dyn CustomComponent>),
}

pub struct DomMeta {
  pub name: &'static str,
  pub attr: Vec<Attribute>,
}

impl Into<NKind> for Meta {
  fn into(self) -> NKind {
    match self {
      Meta::Dom(_) => NKind::Dom,
      Meta::Text(_) => NKind::Text,
      Meta::None => NKind::None,
    }
  }
}

fn find_key(attr: &[Attribute]) -> Option<String> {
  attr.iter().find_map(|a| match a {
    Attribute::Key(key) => Some(key.clone()),
    _ => None,
  })
}

impl Meta {
  pub fn get_key(&self) -> Option<String> {
    match self {
      Meta::Dom(DomMeta { attr, .. }) => find_key(attr),
      Meta::Text(_) => None,
      Meta::None => None,
    }
  }
}

pub enum Attribute {
  Children(Vec<Meta>),
  Styles2(Styles),
  Events(Event),
  Key(String),
}

#[macro_export]
macro_rules! use_view {
  () => {
    use $crate::element::Event::*;
    // use $crate::style::Position::*;
    use $crate::style::Style::*;
  };
}

pub fn span<const N: usize>(attr: [Attribute; N]) -> Meta {
  Meta::Dom(DomMeta {
    name: "span",
    attr: Vec::from(attr),
  })
}

pub fn children<const N: usize>(c: [Meta; N]) -> Attribute {
  Attribute::Children(Vec::from(c))
}

#[macro_export]
macro_rules! div {
  ( $($item:expr),* ) => {{
     $crate::element::Meta::Div(Vec::from([
       $($item,)*
     ]))
  }};
}

macro_rules! style {
  (position: $v: expr) => {
    format!("position: {}", $v)
  };
  (width: $n: expr) => {
    format!("width: {}", $n)
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

pub fn div<const N: usize>(attr: [Attribute; N]) -> Meta {
  Meta::Dom(DomMeta {
    name: "div",
    attr: Vec::from(attr),
  })
}

#[macro_export]
macro_rules! custom {
  ($c: expr) => {
    Meta::Custom(Box::new($c))
  };
}

#[macro_export]
macro_rules! on_click {
  ($c: expr) => {{
    let _ = $c;
    Attribute::Events(Click)
  }};
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

pub trait State {}

pub struct AppState {
  on: bool,
}

// pub struct App {
//   n: i32,
//   state: AppState,
// }
// impl CustomComponent for App {
//   fn render(&mut self) -> Meta {
//     style! {width: 5};
//
//     span([
//       styles().position(Position::new()).background("blue").into(),
//       children([
//         span([]),
//         div([]),
//         div([]),
//         div([]),
//         div([]),
//         // custom!(Numbers::new()),
//       ]),
//       // on_click! {self.n += 1},
//     ])
//   }
// }
