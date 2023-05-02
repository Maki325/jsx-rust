use crate::attribute::Attribute;
use quote::quote;
use syn::{
  parse::{Parse, ParseStream},
  Ident, Lit, Result, Token,
};

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
}

pub trait Updateable<T: Clone>: Clone {
  fn update(&mut self, new_value: T);
}

impl Element {
  fn parse_element(input: ParseStream) -> Result<Self> {
    input.parse::<Token![<]>()?;
    let name: Ident = input.parse()?;
    let self_closing = input.peek(Token![/]);
    if self_closing {
      input.parse::<Token![/]>()?;
    }
    input.parse::<Token![>]>()?;

    if self_closing {
      return Ok(Element::Element(ElementValue {
        name: name.to_string(),
        attributes: vec![],
        children: vec![],
      }));
    }

    let mut children: Vec<Element> = vec![];

    while !input.is_empty() {
      if input.peek(Token![<]) && input.peek2(Token![/]) {
        break;
      }
      let child: Element = input.parse()?;
      children.push(child.into());
    }

    input.parse::<Token![<]>()?;
    input.parse::<Token![/]>()?;
    let closing: Ident = input.parse()?;

    if !closing.eq(&name) {
      return Err(syn::Error::new(
        closing.span(),
        "Closing tag does not match opening tag",
      ));
    }
    input.parse::<Token![>]>()?;

    return Ok(Element::Element(ElementValue {
      name: name.to_string(),
      attributes: vec![],
      children: children,
    }));
  }

  fn parse_literal(input: ParseStream) -> Result<Self> {
    if !input.peek(Lit) {
      return Err(syn::Error::new(
        input.span(),
        "Expected a Literal or an Element",
      ));
    }
    let lit: Lit = input.parse()?;

    match lit {
      Lit::Str(value) => return Ok(Element::Literal(value.value())),
      Lit::ByteStr(value) => {
        let value = match std::str::from_utf8(&value.value()[..]) {
          Ok(value) => value.to_string(),
          Err(_) => return Err(syn::Error::new(value.span(), "Invalid UTF-8 byte string")),
        };
        return Ok(Element::Literal(value));
      }
      Lit::Byte(value) => return Ok(Element::Literal(value.value().to_string())),
      Lit::Char(value) => return Ok(Element::Literal(value.value().to_string())),
      Lit::Int(value) => return Ok(Element::Literal(value.base10_digits().to_string())),
      Lit::Float(value) => return Ok(Element::Literal(value.base10_digits().to_string())),
      Lit::Bool(value) => return Ok(Element::Literal(value.value().to_string())),
      Lit::Verbatim(value) => return Ok(Element::Literal(value.to_string())),
      _ => return Err(syn::Error::new(lit.span(), "Unknown literal type")),
    };
  }

  pub fn to_client_tokens(&self) -> proc_macro2::TokenStream {
    return match self {
      Element::Element(ElementValue { name, children, .. }) => {
        let children = children.iter().map(|child| child.to_client_tokens());
        quote! {
          {
            let element = document.create_element(#name)?;
            #(element.append_child(&#children.into())?;)*

            element
          }
        }
      }
      Element::Literal(value) => {
        quote! {
          document.create_text_node(#value)
        }
      }
    };
  }

  pub fn to_server_tokens(&self) -> proc_macro2::TokenStream {
    return match self {
      Element::Element(ElementValue { name, children, .. }) => {
        let children = children.iter().map(|child| child.to_server_tokens());
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
      Element::Literal(value) => {
        quote! {
          jsx::element::Element::Literal(#value.to_string())
        }
      }
    };
  }
}

impl Parse for Element {
  fn parse(input: ParseStream) -> Result<Self> {
    match input.peek(Token![<]) {
      false => return Element::parse_literal(input),
      true => return Element::parse_element(input),
    };
  }
}
