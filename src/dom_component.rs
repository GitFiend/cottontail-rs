use crate::element::{CustomComponent, VNode};
use crate::style::Position::Absolute;
use crate::style::Style;
use crate::style::Style::Position;

pub enum Node {
  Div(Div),
  Span(Span),
  Custom(Box<dyn CustomComponent>),
}

impl Node {
  pub fn render(&mut self) -> VNode {
    match self {
      Node::Div(d) => d.render(),
      Node::Span(s) => s.render(),
      Node::Custom(c) => c.render(),
    }
  }
}

fn test() {
  // let s = span().style([Position(Absolute)]);
}

#[macro_export]
macro_rules! basic_dom_elements {
  ($struct_name: ident, $func_name: ident) => {
    pub struct $struct_name {
      pub styles: Vec<Style>,
      pub children: Vec<VNode>,
    }

    impl $struct_name {
      pub fn new() -> Self {
        Self {
          styles: Vec::new(),
          children: Vec::new(),
        }
      }

      pub fn render(&self) -> VNode {
        todo!()
      }

      pub fn style<const N: usize>(mut self, style: [Style; N]) -> Self {
        self.styles.extend(style);
        self
      }

      pub fn children<const N: usize>(self, children: [Node; N]) -> Self {
        todo!()
      }
    }

    pub fn $func_name<const N: usize>() -> Node {
      Node::$struct_name($struct_name::new())
    }
  };
}

basic_dom_elements!(Div, div);
basic_dom_elements!(Span, span);
