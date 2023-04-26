use jsx::element::{Element, ElementValue};
use syn::{
  parse::{Parse, ParseStream},
  Ident, Result, Token,
};

#[derive(Debug)]
pub enum View {
  Element(ElementValue),
  String(String),
}

impl Parse for View {
  fn parse(input: ParseStream) -> Result<Self> {
    match input.parse::<Token![<]>() {
      Err(_) => {
        let mut values = vec![];
        while !input.is_empty() {
          let string: Ident = input.parse()?;
          values.push(string.to_string());
        }

        return Ok(View::String(values.concat()));
      }
      Ok(_) => {}
    };
    let name: Ident = input.parse()?;
    input.parse::<Token![>]>()?;

    input.parse::<Token![<]>()?;
    input.parse::<Token![/]>()?;
    let closing: Ident = input.parse()?;

    if !closing.eq(&name) {
      return Err(input.error("Closing tag does not match opening tag"));
    }
    input.parse::<Token![>]>()?;

    return Ok(View::Element(ElementValue {
      name: name.to_string(),
      attributes: vec![],
      children: vec![],
    }));
  }
}

impl Into<Element> for View {
  fn into(self) -> Element {
    return match self {
      Self::Element(element) => Element::Element(element),
      Self::String(string) => Element::String(string),
    };
  }
}
