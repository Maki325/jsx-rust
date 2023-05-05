use quote::ToTokens;
use syn::ExprPath;

pub struct PathElement(pub ExprPath);

impl From<ExprPath> for PathElement {
  fn from(path: ExprPath) -> Self {
    PathElement(path)
  }
}

impl std::fmt::Debug for PathElement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    return f
      .debug_struct("PathElement")
      .field("path", &self.0.to_token_stream())
      .finish();
  }
}
