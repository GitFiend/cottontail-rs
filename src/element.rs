use crate::style::Styles;

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Meta {
  Dom(DomMeta),
  Text(String),
  None, // Custom(Box<dyn CustomComponent>),
}

impl Meta {
  pub fn get_key(&self, index: usize) -> String {
    match self {
      Meta::Dom(dom) => {
        if let Some(key) = &dom.key {
          return key.clone();
        }
        index.to_string()
      }
      Meta::Text(_) => index.to_string(),
      Meta::None => index.to_string(),
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct DomMeta {
  pub name: &'static str,
  pub attr: Vec<Attribute>,
  pub sub_nodes: Vec<Meta>,
  pub key: Option<String>,
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

#[derive(Debug, PartialEq)]
pub enum Attribute {
  SubNodes(Vec<Meta>),
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

fn make_dom_meta<const N: usize>(kind: &'static str, attr: [Attribute; N]) -> DomMeta {
  let mut sub_nodes: Option<Vec<Meta>> = None;
  let mut key: Option<String> = None;

  let mut attributes: Vec<Attribute> = Vec::new();

  for a in attr {
    match a {
      Attribute::SubNodes(s) => sub_nodes = Some(s),
      Attribute::Key(k) => key = Some(k),
      a => attributes.push(a),
    }
  }

  DomMeta {
    name: kind,
    attr: attributes,
    sub_nodes: sub_nodes.unwrap_or_default(),
    key,
  }
}

pub fn div<const N: usize>(attr: [Attribute; N]) -> Meta {
  Meta::Dom(make_dom_meta("div", attr))
}
pub fn span<const N: usize>(attr: [Attribute; N]) -> Meta {
  Meta::Dom(make_dom_meta("span", attr))
}

pub fn children<const N: usize>(c: [Meta; N]) -> Attribute {
  Attribute::SubNodes(Vec::from(c))
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
     $crate::element::Attribute::SubNodes(Vec::from([
       $($item,)*
     ]))
  }};
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

// fn test_mut() {
//   let mut n = 4;
//
//   a(&mut n);
//   b(&mut n);
// }
//
// fn a(n: &mut i32) {
//   *n += 1;
// }
//
// fn b(n: &mut i32) {
//   *n += 1;
// }
//
// pub trait State {}
//
// pub struct AppState {
//   on: bool,
// }

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
