use component::{Component, Prop, PropType};
use jsx::element::Element;
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, ItemFn};
mod component;

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

#[proc_macro_attribute]
pub fn component(_args: TokenStream, s: TokenStream) -> TokenStream {
  let Component {
    name,
    ret,
    body,
    props,
  } = parse_macro_input!(s as Component);
  let lifetimes = body.sig.generics.lifetimes();

  let ItemFn {
    vis, sig, block, ..
  } = &body;

  // println!(
  //   "{:#?}",
  //   &attrs
  //     .iter()
  //     .map(|attr| attr.to_token_stream())
  //     .collect::<Vec<_>>()
  // );
  // println!("{:#?}", &body.to_token_stream());
  let props_name = format_ident!("{name}Props");

  let (_, generics, where_clause) = &sig.generics.split_for_impl();

  // let a = quote! {
  //   #[allow(non_snake_case, clippy::too_many_arguments)]
  //   #vis fn #name #generics (
  //       #[allow(unused_variables)]
  //       #scope_name: ::leptos::Scope,
  //       props: #props_name #generics
  //   ) #ret #(+ #lifetimes)*
  //   #where_clause
  //   #block
  // };

  let stmts = &block.stmts;
  let props_destructor = &props.iter().map(|prop| &prop.name).collect::<Vec<_>>();

  // println!(
  //   "props_destructor: {:#?}",
  //   &props_destructor
  //     .iter()
  //     .map(|ident| ident.to_token_stream())
  //     .collect::<Vec<_>>()
  // );

  let props_generics_idents = &props
    .iter()
    .enumerate()
    .filter(|(_, prop)| {
      if let PropType::ReadSignal(_) = prop.ty {
        true
      } else {
        false
      }
    })
    .map(|(i, prop)| {
      let generic = if let PropType::ReadSignal(t) = &prop.ty {
        t.generic.clone()
      } else {
        unreachable!();
      };

      return (format_ident!("___R___{i}___"), i, generic);
      // let stream = quote! {
      //   #ident: jsx::signal::ReadSignal<#generic>,
      // };
      // // println!("stream: {:#?}", &stream);
      // return stream;

      // return ty.generic.to_token_stream().into();
    })
    .collect::<Vec<_>>();

  let props_generics = props_generics_idents
    .iter()
    .map(|(ident, i, generic)| {
      let ident_into = format_ident!("{ident}{i}___");
      quote! {
        #ident: ReadSignal<#generic>,
        #ident_into: jsx::signal::IntoReadSignal<#generic, #ident>,
      }
    })
    .collect::<Vec<_>>();

  let props_generics_phantom_data = props_generics_idents
    .iter()
    .map(|(generic_ident, i, _)| {
      let ident = format_ident!("phantom_{i}");
      quote! {
        #ident: std::marker::PhantomData<#generic_ident>,
      }
    })
    .collect::<Vec<_>>();

  let props_generics_names = props_generics_idents
    .iter()
    .map(|(ident, i, _)| {
      let ident_into = format_ident!("{ident}{i}___");
      quote! {
        #ident, #ident_into,
      }
    })
    .collect::<Vec<_>>();

  let props = props.iter().enumerate().map(|(i, prop)| {
    let ty = match &prop.ty {
      PropType::Type(ty) => quote! { #ty },
      PropType::ReadSignal(_) => {
        let ident = format_ident!("___R___{i}___{i}___");
        quote! { #ident }
      }
    };
    let name = &prop.name;
    return quote! {
      #name: #ty,
    };
  });

  let generics = &sig.generics.params;

  let a = quote! {
    #[allow(non_camel_case_types)]
    struct #props_name<#(#props_generics),*> {
      #(#props),*
      #(#props_generics_phantom_data),*
    }

    #[allow(non_snake_case, clippy::too_many_arguments)]
    #vis fn #name <#generics #(#props_generics),*> (
      document: &::web_sys::Document,
      props: #props_name <#generics #(#props_generics_names),*>
    ) #ret #(+ #lifetimes)*
    #where_clause
    {
      let #props_name { #(#props_destructor),* ,.. } = props;
      #(#stmts)*
    }
  };

  return a.into();
}
