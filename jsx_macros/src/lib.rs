use jsx::element::Element;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn view(input: TokenStream) -> TokenStream {
  let view = parse_macro_input!(input as Element);

  let expanded = quote! { #view };

  TokenStream::from(expanded)
}
