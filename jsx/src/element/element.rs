use crate::attribute::Attribute;

use super::path_element::PathElement;

#[derive(Debug)]
pub struct ElementValue {
  pub name: String,
  pub attributes: Vec<Attribute>,
  pub children: Vec<Element>,
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
