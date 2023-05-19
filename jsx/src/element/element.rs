use crate::attribute::Attribute;

use super::path_element::PathElement;

#[derive(Debug)]
pub struct ElementValue {
  pub name: String,
  pub attributes: Vec<Attribute>,
  pub children: Vec<Element>,
  pub is_custom: bool,
}

impl ElementValue {
  pub fn new(name: String) -> Self {
    Self {
      is_custom: ElementValue::is_custom(&name),
      name,
      attributes: vec![],
      children: vec![],
    }
  }

  pub fn new_with_attributes(name: String, attributes: Vec<Attribute>) -> Self {
    Self {
      is_custom: ElementValue::is_custom(&name),
      name,
      attributes,
      children: vec![],
    }
  }

  pub fn new_with_children(name: String, children: Vec<Element>) -> Self {
    Self {
      is_custom: ElementValue::is_custom(&name),
      name,
      attributes: vec![],
      children,
    }
  }

  pub fn new_with_attributes_and_children(
    name: String,
    attributes: Vec<Attribute>,
    children: Vec<Element>,
  ) -> Self {
    Self {
      is_custom: ElementValue::is_custom(&name),
      name,
      attributes,
      children,
    }
  }

  pub fn is_custom(name: &String) -> bool {
    return name
      .chars()
      .nth(0)
      .map_or_else(|| false, |c| c.is_uppercase());
  }
}

#[derive(Debug)]
pub enum Element {
  Element(ElementValue),

  // I see no need to have different types of literals,
  // as they will be rendered as strings anyway
  // So we may as well just save them as strings
  Literal(String),

  // For like, updatable with a signal or something
  Updateable(PathElement),
}
