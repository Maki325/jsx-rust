use crate::utils;

use super::{
  attribute::{AttributeType, ATTRIBUTE_NAME_MAP},
  Attribute, AttributeValue,
};
use quote::ToTokens;
use syn::{
  parse::{Parse, ParseStream},
  spanned::Spanned,
  Path, Result, Token,
};

impl Attribute {
  fn parse(input: ParseStream) -> Result<Self> {
    let path = utils::get_name(&input)?;

    let ty = ATTRIBUTE_NAME_MAP.get(&path.as_str());
    let (ty, name) = match ty {
      Some(ty) => ty,
      None => {
        return Err(syn::Error::new(
          path.span(),
          format!("Unknown attribute name: {}", path),
        ))
      }
    };

    input.parse::<Token![=]>()?;

    match ty {
      AttributeType::Function => {
        let function_name: Path = input.parse()?;
        return Ok(Attribute {
          name: name.to_string(),
          value: AttributeValue::Function(function_name.to_token_stream()),
        });
      }
      _ => unimplemented!(),
    }
  }
}

impl Parse for Attribute {
  fn parse(input: ParseStream) -> Result<Self> {
    return Attribute::parse(input);
  }
}
