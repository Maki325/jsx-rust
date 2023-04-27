use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn ifs(input: TokenStream) -> TokenStream {
  let input = proc_macro2::TokenStream::from(input);
  let recursive = input.into_iter().map(|tok| {
    let value = tok.to_string();
    // let value2 = format!("syn::Token![{}]", value.as_str());
    // let i = Ident::new(value2.as_str(), tok.span());
    // println!("{:#?}", i);
    // let i = Ident::new(value.as_str(), tok.span());
    // if input.peek(#i) {
    //   input.parse::<#i>()?;
    //   values.push(#value.into());
    //   continue;
    // }
    let a = quote! {
      if input.peek(syn::Token![#tok]) {
        input.parse::<syn::Token![#tok]>()?;
        values.push(#value.into());
        // values.push(#tok);
        continue;
      }
    };
    println!("{:#?}", a);

    return a;
  });

  let expanded = quote! {
    #(#recursive)*
  };
  // println!("expanded: {:#?}", expanded);

  TokenStream::from(expanded)
}

#[proc_macro]
pub fn stringify_tokens(input: TokenStream) -> TokenStream {
  let input = proc_macro2::TokenStream::from(input);
  let expanded = quote! {
    stringify! {
      #input
    }
  };

  TokenStream::from(expanded)
}
