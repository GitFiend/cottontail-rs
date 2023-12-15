use crate::component::{DomComponent, ElementComponent, RootComponent};
use std::collections::HashMap;
use web_sys::{Element, HtmlElement};

pub trait ChildNode<T> {
  fn get_element(&self) -> &T;
}

impl ChildNode<HtmlElement> for DomComponent {
  fn get_element(&self) -> &HtmlElement {
    &self.element
  }
}

/*
Alt idea:

We write the implementations in one go. A macro for that.

Don't make a general macro?

 */

pub trait ParentNode {
  fn get(&self) -> ParentFields;
}

pub struct ParentFields<'a> {
  pub element: &'a HtmlElement,
  pub inserted: &'a Vec<ElementComponent>,
  pub siblings: &'a HashMap<Element, Element>,
}

macro_rules! parent_component {
  ($($struct_name: ident),*) => {
    $(
    impl ParentNode for $struct_name {
      fn get(&self) -> ParentFields {
        ParentFields {
          element: &self.element,
          inserted: &self.inserted,
          siblings: &self.siblings
        }
      }
    }
    )*
  };
}
// parent_component!(RootComponent, DomComponent);

/*

trait name
struct names (RootComponent, DomComponent)
struct fields (element, inserted)

We make a trait, we impl getters and setters(?) for those structs.

Or, we could generate a struct with those properties. This would let
us do destructuring. What about mutation?

get and get_mut?

 */
#[macro_export]
macro_rules! make_trait {
  ($trait_name:ident, $($struct_name:ident),+ {$($field:ident),+}) => {
    pub trait $trait_name {
      $(
      fn get_$field(&self) -> &$return_type;
      )*
    }

    $(
    impl $trait_name for $struct_name {
      $(
      fn get_$field(&self) -> &$return_type {
        &self.$field
      }
      )*
    }
    )*
  };
}

// make_trait!(MyParent, DomComponent { a, b, c });
