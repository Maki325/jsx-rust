use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
  // Build the output, possibly using quasi-quotation
  let mut expanded = quote! {
    fn tf() {
      println!("Hello, World!");
    }
    tf();
  };

  expanded.extend(proc_macro2::TokenStream::from(input));

  // Hand the output tokens back to the compiler
  TokenStream::from(expanded)
}
