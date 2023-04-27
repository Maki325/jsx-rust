use quote::{quote, ToTokens};

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

impl ToTokens for Element {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let expanded = match self {
      Element::Element(ElementValue { name, children, .. }) => {
        quote! {
          jsx::element::Element::Element({
            jsx::element::ElementValue {
              name: #name.to_string(),
              attributes: Vec::new(),
              children: vec![#(#children),*],
            }
          })
        }
      }
      Element::String(value) => {
        quote! {
          jsx::element::Element::String(#value.to_string())
        }
      }
    };

    tokens.extend(expanded);
  }
}
