use helper_macros::{ifs, stringify_tokens};
use jsx::element::{Element, ElementValue};
use proc_macro2::{Punct, TokenStream, TokenTree};
use quote::quote;
use syn::{
  parse::{Parse, ParseStream},
  token::{self, Token},
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
        // let mut values: Vec<String> = vec![];
        // while !input.is_empty() {
        //   if input.peek(Token![<]) {
        //     break;
        //   }
        //   ifs!(;:,?);
        //   let string: Ident = input.parse()?;
        //   values.push(string.to_string());
        // }

        let mut tokens: Vec<TokenTree> = vec![];
        let mut values: Vec<String> = vec![];
        while !input.peek(Token![<]) && !input.is_empty() {
          let token: TokenTree = input.parse()?;
          tokens.push(token);
          // values.push(token.to_string());
        }

        let stream = TokenStream::from_iter(tokens);

        let expanded = quote! {
          stringify! {
            #stream
          }
        };

        // return Ok(View::String(String::from(stringify_tokens!(quote! {
        //   #stream
        // }))));
        // return Ok(View::String(TokenStream::from(expanded).to_string()));
        return Ok(View::String(stream.to_string()));
        // return Ok(View::String(values.concat()));
      }
      Ok(_) => {}
    };
    let name: Ident = input.parse()?;
    input.parse::<Token![>]>()?;

    let mut children: Vec<Element> = vec![];

    while !input.is_empty() {
      if !input.peek(Token![<]) && !input.peek(Ident) {
        break;
      }
      if input.peek(Token![<]) && input.peek2(Token![/]) {
        break;
      }
      let child: View = input.parse()?;
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

    return Ok(View::Element(ElementValue {
      name: name.to_string(),
      attributes: vec![],
      children: children,
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
