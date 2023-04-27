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
