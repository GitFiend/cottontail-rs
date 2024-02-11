use std::collections::{HashMap, HashSet, VecDeque};

use crate::component::order::NodeOrder;
use crate::components::dom_component::DomComponentInfo;
use crate::console_log;
use web_sys::HtmlElement;

// TODO: Consider having id specific to each component type for type safety.
pub type Id = usize;
pub const NONE_ID: Id = 0;

#[derive(Debug)]
pub enum ComponentInfo {
  Dom(DomComponentInfo),
  Text,
  None,
}

// TODO: Do we need to go this far into data-oriented(?) style?
//  Don't we just need to decouple references between nodes using IDs?
// Update yes, we probably do due to how wierd everything is.
#[derive(Debug)]
pub struct CTStore {
  pub kind: Vec<ComponentInfo>,
  pub element: Vec<Option<HtmlElement>>,
  pub key: Vec<String>,
  pub order: Vec<NodeOrder>,

  // Index is the element id, value is the previous sibling element id.
  pub sibling: Vec<Id>,

  // Index is the parent, and inserted are the ordered child elements.
  // Optional as not all components can have subcomponents.
  pub inserted: Vec<Option<Vec<Id>>>,
  pub sub_components: Vec<Option<HashMap<String, Id>>>,

  pub direct_parent: Vec<Id>,
  pub dom_parent: Vec<Id>,

  insert_stack: HashSet<Id>,
  // TODO
  deleted: Vec<bool>,
  next_id: Id,
  recycled_ids: VecDeque<Id>,
}

impl CTStore {
  pub fn new(root_element: HtmlElement) -> Self {
    // Root is position 1. 0 is always empty.
    // Do we need anything for Root?
    CTStore {
      kind: vec![ComponentInfo::None, ComponentInfo::None],
      element: vec![None, Some(root_element)],
      key: vec![String::default(), String::from("root")],
      order: vec![NodeOrder::none(), NodeOrder::default()],
      sibling: vec![NONE_ID, NONE_ID],
      inserted: vec![None, Some(Vec::new())],
      sub_components: vec![None, Some(HashMap::new())],
      direct_parent: vec![NONE_ID, NONE_ID],
      dom_parent: vec![NONE_ID, NONE_ID],
      insert_stack: HashSet::new(),
      deleted: vec![true, false],
      next_id: 2,
      recycled_ids: Default::default(),
    }
  }

  pub fn print(&self) {
    console_log!("{:?}", self);
  }

  #[allow(clippy::too_many_arguments)]
  pub fn add(
    &mut self,
    kind: ComponentInfo,
    element: Option<HtmlElement>,
    key: String,
    index: usize,
    direct_parent: Id,
    dom_parent: Id,
    inserted: Option<Vec<Id>>,
    sub_components: Option<HashMap<String, Id>>,
  ) -> Id {
    // TODO: Check for reusable ids first.

    self.kind.push(kind);
    self.element.push(element);
    self.key.push(key);
    self.order.push(self.order[direct_parent].next(index));
    self.sibling.push(NONE_ID);
    self.direct_parent.push(direct_parent);
    self.dom_parent.push(dom_parent);
    self.inserted.push(inserted);
    self.sub_components.push(sub_components);
    self.deleted.push(false);

    let id = self.next_id;
    self.next_id += 1;

    id
  }

  // TODO: Push to stack of inserts to do later, rather than immediately.
  pub fn insert(&mut self, parent_id: Id, child_id: Id) {
    let order = &self.order[child_id];
    let key = &self.key[child_id];

    // self.print();
    console_log!("parent {parent_id}, child: {child_id}, order: {:?}", order);

    if let Some(inserted) = &mut self.inserted[parent_id] {
      for (i, current) in inserted.iter().rev().cloned().enumerate() {
        let next = inserted.get(i + 1);

        // If order is the same, we expect the keys to be different.
        // This is expected for a virtual list.
        if order >= &self.order[current] {
          if key != &self.key[current] {
            if next.is_some() {
              console_log!("a insert {child_id} at {i}");
              inserted.insert(i, child_id);
            } else {
              console_log!("b insert {child_id} at end");
              inserted.push(child_id);
            }
            self.queue_insert(parent_id);
          }

          return;
        }
      }

      console_log!("c insert {child_id} at end");
      inserted.push(child_id);
      // self.apply_inserts(parent_id);
      self.queue_insert(parent_id);
    }
  }

  fn queue_insert(&mut self, id: Id) {
    self.insert_stack.insert(id);
  }

  pub fn apply_inserts(&mut self) {
    let mut stack = self.insert_stack.iter().cloned().collect::<Vec<Id>>();
    stack.sort();

    console_log!("apply_inserts {:?}", self.insert_stack);

    // TODO: Order?

    for id in stack {
      self.apply_inserts_for_parent(id);
    }
  }

  fn apply_inserts_for_parent(&mut self, parent: Id) -> Option<()> {
    let parent_element = self.element[parent].as_ref()?;
    let mut next: Option<Id> = None;

    console_log!("inserted: {:?}, parent: {parent}", self.inserted[parent]);
    let mut i = self.inserted[parent].clone()?.len() - 1;

    // console_log!("start inserted.len(): {i}, parent_id: {parent}");

    for id in self.inserted[parent].as_mut()?.iter().rev().cloned() {
      console_log!("loop i: {i}, id: {id}");
      let current_element = self.element[id].as_ref().unwrap();

      match next {
        // This means current is the last element.
        None => {
          // If current has no sibling, it may not have been inserted.
          if let Some(sibling_id) = self.sibling.get(id) {
            if *sibling_id == NONE_ID {
              // Insert at the end
              console_log!("insert {id} at end (no sibling)");
              parent_element.insert_before(current_element, None).unwrap();
            }
          }
        }
        Some(next) => {
          console_log!("next {next}, {:?}", self.sibling.get(next));
          if self.sibling[next] != id {
            let next_node = self.element[next].as_ref();
            console_log!(
              "insert {id} before {:?}, {:?}",
              self.kind[id],
              self.kind[next]
            );

            console_log!("insert {id} {:?} {:?}", current_element, self.kind[id]);
            parent_element
              .insert_before(current_element, Some(&**next_node.unwrap()))
              .unwrap();

            self.sibling[next] = id;
          }
        }
      }

      i = i.saturating_sub(1);
      next = Some(id);
    }

    if cfg!(debug_assertions)
      && parent_element.child_element_count() as usize
        != self.inserted[parent].as_ref().unwrap().len()
    {
      console_log!("inserted and children count don't match!!");
    }

    Some(())
  }

  pub fn move_component(&mut self, parent: Id, child: Id) {
    self.remove_from_inserted(parent, child);

    self.insert(parent, child);
  }

  pub fn remove(&mut self, parent: Id, child: Id) {
    self.remove_from_inserted(parent, child);

    if let Some(child) = &self.element[child] {
      child.remove();
    }
    self.sibling[child] = NONE_ID;
  }

  fn remove_from_inserted(&mut self, parent: Id, child: Id) {
    if let Some(inserted) = &mut self.inserted[parent] {
      let key = &self.key[child];
      if let Some(i) = inserted.iter().position(|ins| self.key[*ins] == *key) {
        inserted.remove(i);
      }
    }
  }
}
