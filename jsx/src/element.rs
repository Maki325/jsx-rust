#[derive(Debug)]
pub struct ElementValue {
  pub name: String,
  pub attributes: Vec<Attribute>,
  pub children: Vec<Element>,
}

#[derive(Debug)]
pub enum Element {
  Element(ElementValue),
  String(String),
}

#[derive(Debug)]
pub struct Attribute {
  pub name: String,
  pub value: AttributeValue,
}

#[derive(Debug)]
pub enum AttributeValue {
  String(String),
  Number(f64),
  Boolean(bool),
}

pub trait Render {
  fn render(&self) -> String;
}
