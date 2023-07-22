use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::{collections::HashMap, fmt};
use syn::Expr;

#[derive(Debug)]
pub struct Attribute {
  // pub name: Path, // I don't knw why I put it as Path, maybe I'll remember some day
  pub name: String,
  pub value: AttributeValue,
}

pub enum AttributeValue {
  String(String),
  Number(f64),
  Boolean(bool),
  Function(TokenStream),
  Custom(Expr),
  EndOfAttributes,
}

impl fmt::Debug for AttributeValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    return match self {
      Self::Custom(expr) => write!(f, "Custom {:#?}", expr.clone().into_token_stream()),
      others => others.fmt(f),
    };
  }
}

#[allow(dead_code)]
pub enum AttributeType {
  String,
  Number,
  Boolean,
  Function,
  Custom,
}

lazy_static! {
  pub static ref ATTRIBUTE_NAME_MAP: HashMap<&'static str, (AttributeType, &'static str)> = {
    let mut m = HashMap::new();
    m.insert("on::click", (AttributeType::Function, "click"));
    m
  };
}
