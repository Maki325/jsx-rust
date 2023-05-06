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

fn path_to_string(path: &Path) -> String {
  return path
    .segments
    .iter()
    .map(|s| s.ident.to_string())
    .collect::<Vec<String>>()
    .join("::");
}

impl Attribute {
  fn parse(input: ParseStream) -> Result<Self> {
    let path: Path = input.parse()?;
    let name = path_to_string(&path);

    let ty = ATTRIBUTE_NAME_MAP.get(&name.as_str());
    let (ty, name) = match ty {
      Some(ty) => ty,
      None => {
        return Err(syn::Error::new(
          path.span(),
          format!("Unknown attribute name: {}", name),
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
