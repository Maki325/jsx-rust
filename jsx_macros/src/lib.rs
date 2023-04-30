use jsx::element::Element;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn view(input: TokenStream) -> TokenStream {
  let element = parse_macro_input!(input as Element);

  return element.to_client_tokens().into();
}

#[proc_macro]
pub fn ssr(input: TokenStream) -> TokenStream {
  let element = parse_macro_input!(input as Element);

  return element.to_server_tokens().into();
}
