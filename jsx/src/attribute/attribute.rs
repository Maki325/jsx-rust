use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Attribute {
  // pub name: Path, // I don't knw why I put it as Path, maybe I'll remember some day
  pub name: String,
  pub value: AttributeValue,
}

#[derive(Debug)]
pub enum AttributeValue {
  String(String),
  Number(f64),
  Boolean(bool),
  Function(TokenStream),
}

#[allow(dead_code)]
pub enum AttributeType {
  String,
  Number,
  Boolean,
  Function,
}

lazy_static! {
  pub static ref ATTRIBUTE_NAME_MAP: HashMap<&'static str, (AttributeType, &'static str)> = {
    let mut m = HashMap::new();
    m.insert("on::click", (AttributeType::Function, "click"));
    m
  };
}
