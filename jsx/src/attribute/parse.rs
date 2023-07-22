use crate::utils;

use super::{
  attribute::{AttributeType, ATTRIBUTE_NAME_MAP},
  Attribute, AttributeValue,
};
use quote::ToTokens;
use syn::{
  braced,
  parse::{discouraged::Speculative, Parse, ParseStream},
  token::Brace,
  Block, Expr, ExprBlock, Path, Result, Token,
};

fn block_expr(input: &ParseStream) -> Result<Expr> {
  let fork = input.fork();
  let content;
  let brace_token = braced!(content in fork);
  let block = ExprBlock {
    attrs: vec![],
    label: None,
    block: Block {
      brace_token,
      stmts: Block::parse_within(&content)?,
    },
  };
  input.advance_to(&fork);

  Ok(block.into())
}

fn parse_expr(input: &ParseStream) -> Result<Expr> {
  if input.peek(Brace) {
    return block_expr(input);
  } else {
    return input.parse();
  }
}

impl Attribute {
  fn parse(input: ParseStream) -> Result<Self> {
    if input.peek(Token![>]) || input.peek(Token![/]) {
      return Ok(Attribute {
        name: String::new(),
        value: AttributeValue::EndOfAttributes,
      });
    }

    let path = utils::get_name(&input)?;

    let custom_type = (AttributeType::Custom, path.as_str());

    let ty = ATTRIBUTE_NAME_MAP.get(&path.as_str());
    let (ty, name) = match ty {
      Some(ty) => ty,
      None => &custom_type,
    };

    input.parse::<Token![=]>()?;

    match ty {
      AttributeType::Function => {
        let function_name: Path = input.parse()?;
        return Ok(Attribute {
          name: name.to_string(),
          // Change practically all the values to take EXPR
          value: AttributeValue::Function(function_name.to_token_stream()),
        });
      }
      AttributeType::Custom => {
        let value: Expr = parse_expr(&input)?;
        return Ok(Attribute {
          name: name.to_string(),
          value: AttributeValue::Custom(value),
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
