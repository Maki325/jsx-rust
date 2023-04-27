mod view;

use jsx::element::Element;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use view::View;

#[proc_macro]
pub fn view(input: TokenStream) -> TokenStream {
  let view = parse_macro_input!(input as View);
  let view: Element = view.into();

  let expanded = quote! { #view };

  TokenStream::from(expanded)
}
