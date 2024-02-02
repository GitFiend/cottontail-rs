// use std::collections::HashMap;
// use std::rc::Rc;
//
// use web_sys::HtmlElement;
//
// use crate::component::order::OrderAttr;
// use crate::component::{ChildAttr, ElementComponent, SubComponent};
//
// // Component can be a parent.
// pub struct ParentAttr {
//   pub element: HtmlElement,
//   pub inserted: Vec<Rc<ElementComponent>>,
//   pub siblings: Vec<(HtmlElement, Option<HtmlElement>)>,
// }
//
// pub enum ParentDomComponent {
//   Root(RootComponent),
//   Dom(DomComponent),
// }
//
// pub struct RootComponent {
//   pub parent_attr: Rc<ParentAttr>,
//   pub order_attr: OrderAttr,
// }

// pub struct DomComponent {
//   pub parent_attr: Rc<ParentAttr>,
//   pub order_attr: OrderAttr,
//   pub child_attr: ChildAttr,
//   pub sub_components: HashMap<String, SubComponent>,
//   pub element: HtmlElement,
// }
