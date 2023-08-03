use quote::ToTokens;
use syn::{ExprField, ExprPath};

pub enum PathElement {
  Field(ExprField),
  Path(ExprPath),
}

impl From<ExprField> for PathElement {
  fn from(path: ExprField) -> Self {
    PathElement::Field(path)
  }
}

impl From<ExprPath> for PathElement {
  fn from(path: ExprPath) -> Self {
    PathElement::Path(path)
  }
}

impl std::fmt::Debug for PathElement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let data = match &self {
      Self::Field(path) => path.to_token_stream(),
      Self::Path(path) => path.to_token_stream(),
    };
    return f.debug_struct("PathElement").field("path", &data).finish();
  }
}
