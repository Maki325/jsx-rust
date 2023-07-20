use component::{Component, PropType};
use jsx::element::Element;
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, GenericArgument, ItemFn, PatIdent, PathArguments, Type};

use crate::component::Prop;
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
  let ItemFn {
    vis, sig, block, ..
  } = &body;

  let lifetimes = sig.generics.lifetimes();
  let lifetimes2 = sig.generics.lifetimes();

  let props_struct_name = format_ident!("{name}Props");

  let where_clause = &sig.generics.where_clause.as_ref();

  let stmts = &block.stmts;

  fn get_generic_data(
    props: &Vec<Prop>,
  ) -> (
    Vec<PatIdent>,
    Vec<PatIdent>,
    Vec<proc_macro2::TokenStream>,
    Vec<proc_macro2::TokenStream>,
    Vec<proc_macro2::Ident>,
    Vec<proc_macro2::TokenStream>,
    Vec<proc_macro2::TokenStream>,
  ) {
    let mut names_and_types = vec![];
    let mut names = vec![];
    let mut required_names_and_types = vec![];
    let mut required_names = vec![];
    let mut generics_with_bounds = vec![];
    let mut generic_names = vec![];
    let mut phantom = vec![];

    fn is_optional_type(ty: &Type) -> bool {
      let path = match ty {
        Type::Path(path) => &path.path,
        _ => return false,
      };

      if path.segments.len() != 1 {
        return false;
      }

      let first_segment = match path.segments.first() {
        Some(first_segment) => first_segment,
        None => return false,
      };

      if first_segment.ident != "Option" {
        return false;
      }

      let angle_bracketed = match &first_segment.arguments {
        PathArguments::AngleBracketed(angle_bracketed) => angle_bracketed,
        _ => return false,
      };

      if angle_bracketed.args.len() != 1 {
        return false;
      }

      let first_generic_arg = match angle_bracketed.args.first() {
        Some(first_generic_arg) => first_generic_arg,
        None => return false,
      };

      return match first_generic_arg {
        GenericArgument::Type(_) => true,
        _ => false,
      };
    }

    for prop in props {
      let name = &prop.name;
      let (ty, is_optional) = match &prop.ty {
        PropType::Type(ty) => {
          // match &ty {
          //   Type::Array(_) => println!("Array!"),
          //   Type::BareFn(_) => println!("BareFn!"),
          //   Type::Group(_) => println!("Group!"),
          //   Type::ImplTrait(_) => println!("ImplTrait!"),
          //   Type::Infer(_) => println!("Infer!"),
          //   Type::Macro(_) => println!("Macro!"),
          //   Type::Never(_) => println!("Never!"),
          //   Type::Paren(_) => println!("Paren!"),
          //   Type::Path(_) => println!("Path!"),
          //   Type::Ptr(_) => println!("Ptr!"),
          //   Type::Reference(_) => println!("Reference!"),
          //   Type::Slice(_) => println!("Slice!"),
          //   Type::TraitObject(_) => println!("TraitObject!"),
          //   Type::Tuple(_) => println!("Tuple!"),
          //   Type::Verbatim(_) => println!("Verbatim!"),
          //   _ => println!("Other!"),
          // }

          (quote! { #ty }, is_optional_type(&ty))
        }
        PropType::ReadSignal(ty) => {
          let generic = &ty.generic;

          let i = phantom.len();

          let read_ident = format_ident!("___R___{i}___");
          let into_read_ident = format_ident!("{read_ident}{i}___");

          generic_names.push(read_ident.clone());
          generic_names.push(into_read_ident.clone());

          generics_with_bounds.push(quote! { #read_ident: ReadSignal<#generic> });
          generics_with_bounds.push(quote! {
            #into_read_ident: jsx::signal::IntoReadSignal<#generic, #read_ident>
          });

          let phantom_ident = format_ident!("___phantom_{i}___");
          phantom.push(quote! {
            #phantom_ident: std::marker::PhantomData<#read_ident>,
          });

          (quote! { #into_read_ident }, false)
        }
      };

      if !is_optional {
        required_names.push(name.clone());
        required_names_and_types.push(quote! { #name: #ty, });
      }

      names.push(name.clone());
      names_and_types.push(quote! { #name: #ty, });
    }

    return (
      names,
      required_names,
      names_and_types,
      required_names_and_types,
      generic_names,
      generics_with_bounds,
      phantom,
    );
  }

  let (
    prop_names,
    _required_prop_names,
    prop_names_and_types,
    required_prop_names_and_types,
    prop_generic_names,
    prop_generics_with_bounds,
    prop_phantom,
  ) = get_generic_data(&props);

  let generics = &sig.generics.params;

  let builder_struct_name = format_ident!("{props_struct_name}Builder");
  let name_inner = format_ident!("{name}Inner");

  let a = quote! {
    #[allow(non_camel_case_types)]
    pub struct #props_struct_name<#(#prop_generics_with_bounds),*> {
      #(#prop_names_and_types)*
      #(#prop_phantom)*
    }

    #[allow(non_camel_case_types)]
    pub struct #builder_struct_name <#(#prop_generics_with_bounds),*> {
      inter: #props_struct_name <#(#prop_generic_names),*>
    }

    #[allow(non_camel_case_types)]
    impl <#(#prop_generics_with_bounds),*> #builder_struct_name <#(#prop_generic_names),*> {
      fn new(#(#required_prop_names_and_types)*) {
      }
    }

    #[allow(non_camel_case_types, non_snake_case, unused_variables, clippy::too_many_arguments)]
    #vis fn #name <#generics #(#prop_generics_with_bounds),*> (
      document: &::web_sys::Document,
      props: #props_struct_name <#generics #(#prop_generic_names),*>
    ) #ret #(+ #lifetimes)*
    #where_clause
    {
      let #props_struct_name { #(#prop_names),* ,.. } = props;

      // #({let _ = &#prop_names;})*

      #[allow(non_camel_case_types, non_snake_case, unused_variables, clippy::too_many_arguments)]
      #vis fn #name_inner <#generics #(#prop_generics_with_bounds),*> (
        document: &::web_sys::Document,
        #(#prop_names_and_types)*
      ) #ret #(+ #lifetimes2)*
      #where_clause
      {
        #(#stmts)*
      }

      return #name_inner(document, #(#prop_names),*);
    }
  };

  return a.into();
}
