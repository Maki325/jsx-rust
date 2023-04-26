pub struct Element {
  pub name: String,
  pub attributes: Vec<Attribute>,
  pub children: Vec<Element>,
}

pub struct Attribute {
  pub name: String,
  pub value: String,
}

pub trait Render {
  fn render(&self) -> String;
}
