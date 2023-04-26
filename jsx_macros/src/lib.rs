mod view;

use jsx::element::ElementValue;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use view::View;

#[proc_macro]
pub fn view(input: TokenStream) -> TokenStream {
  let view = parse_macro_input!(input as View);

  let expanded = match view {
    View::Element(ElementValue { name, .. }) => {
      quote! {
        jsx::element::Element::Element({
          jsx::element::ElementValue {
            name: #name.to_string(),
            attributes: Vec::new(),
            children: Vec::new(),
          }
        })
      }
    }
    View::String(value) => {
      quote! {
        jsx::element::Element::String(#value)
      }
    }
  };

  TokenStream::from(expanded)
}
