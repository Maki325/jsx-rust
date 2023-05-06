use quote::ToTokens;

pub struct Path(pub syn::Path);

impl From<syn::Path> for Path {
  fn from(path: syn::Path) -> Self {
    Path(path)
  }
}

impl std::fmt::Debug for Path {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    return f
      .debug_struct("Path")
      .field("path", &self.0.to_token_stream())
      .finish();
  }
}
