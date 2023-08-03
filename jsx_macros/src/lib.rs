use component::{Component, PropType};
use jsx::element::Element;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
  parse_macro_input, GenericArgument, GenericParam, ItemFn, PatIdent, PathArguments, Type,
};

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

struct PropField {
  pub name: PatIdent,
  pub ty: proc_macro2::TokenStream,
  pub is_optional: bool,
  pub phantom: Option<(proc_macro2::TokenStream, proc_macro2::Ident)>,
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
    Vec<proc_macro2::TokenStream>,
    Vec<proc_macro2::TokenStream>,
    Vec<proc_macro2::Ident>,
    Vec<proc_macro2::TokenStream>,
    Vec<proc_macro2::TokenStream>,
    Vec<PropField>,
  ) {
    let mut names_and_types = vec![];
    let mut names = vec![];
    let mut generics_with_bounds = vec![];
    let mut generic_names = vec![];
    let mut phantom = vec![];
    let mut prop_struct_fields = vec![];

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
      let (ty, phantom_option, is_optional) = match &prop.ty {
        PropType::Type(ty) => (quote! { #ty }, None, is_optional_type(&ty)),
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
          let phantom_tokens = quote! {
            #phantom_ident: std::marker::PhantomData<#read_ident>,
          };
          phantom.push(phantom_tokens.clone());

          (
            quote! { #into_read_ident },
            Some((phantom_tokens, phantom_ident)),
            false,
          )
        }
      };

      prop_struct_fields.push(PropField {
        name: name.clone(),
        ty: ty.clone(),
        is_optional,
        phantom: phantom_option,
      });

      names.push(quote! { #name, });
      names_and_types.push(quote! { #name: #ty, });
    }

    return (
      names,
      names_and_types,
      generic_names,
      generics_with_bounds,
      phantom,
      prop_struct_fields,
    );
  }

  let (
    prop_names,
    prop_names_and_types,
    prop_generic_names,
    prop_generics_with_bounds,
    prop_phantom,
    prop_struct_fields,
  ) = get_generic_data(&props);

  let generics = &sig.generics.params;

  let builder_struct_name = format_ident!("{props_struct_name}Builder");
  let name_inner = format_ident!("{name}Inner");

  let all_generic_names = {
    let mut stream = proc_macro2::TokenStream::new();

    stream.extend(Some(quote! {#(#prop_generic_names),*}));
    if generics.len() != 0 {
      stream.extend(Some(quote! {,}));
    }

    let generic_names = &generics
      .iter()
      .map(|generic| match generic {
        GenericParam::Type(ty) => &ty.ident,
        _ => unimplemented!("Only type generics are supported"),
      })
      .collect::<Vec<_>>();
    stream.extend(Some(quote! {#(#generic_names),*}));

    stream
  };
  let all_generics_with_bounds = {
    let mut stream = proc_macro2::TokenStream::new();

    stream.extend(Some(quote! {#(#prop_generics_with_bounds),*}));
    if generics.len() != 0 {
      stream.extend(Some(quote! {,}));
    }
    stream.extend(Some(quote! {#generics}));

    stream
  };

  let props_struct = {
    let mut builder_fields = vec![];
    let mut builder_fns = vec![];
    let mut builder_new = vec![];
    let mut names = vec![];
    let mut build_required = vec![];
    let mut build_destructuring = vec![];

    names.extend(prop_names.clone());

    for PropField {
      is_optional,
      name,
      ty,
      phantom,
    } in &prop_struct_fields
    {
      let ident = &name.ident;
      let fn_name = format_ident!("set_{ident}");

      if *is_optional {
        builder_fields.push(quote! { #name: #ty, });
        builder_fns.push(quote! {
          #[allow(non_camel_case_types, dead_code)]
          pub fn #fn_name(&mut self, #name: #ty) -> &mut Self {
            self.#name = #name;
            self
          }
        });
        builder_new.push(quote! { #name: None });
        build_destructuring.push(quote! { #name, });
      } else {
        let ident = &name.ident;

        builder_fields.push(quote! {
          #name: Option<#ty>,
        });
        builder_fns.push(quote! {
          #[allow(non_camel_case_types, dead_code)]
          pub fn #fn_name(&mut self, #name: #ty) -> &mut Self {
            self.#name = Some(#name);

            self
          }
        });
        builder_new.push(quote! { #name: None });

        let string = format!("Required prop `{}` not set", ident);

        build_required.push(quote! {
          let #name = match self.#name {
            Some(val) => val,
            None => return Err(#string),
          };
        });
      }

      if let Some((phantom, ident)) = phantom {
        builder_fields.push(quote! { #phantom });
        names.push(quote! { #ident: std::marker::PhantomData, });
        builder_new.push(quote! { #ident: std::marker::PhantomData });
      }
    }

    let struct_tokens = quote! {
      #[allow(non_camel_case_types, dead_code)]
      pub struct #props_struct_name<#all_generics_with_bounds> {
        #(#prop_names_and_types)*
        #(#prop_phantom)*
      }

      #[allow(non_camel_case_types, dead_code)]
      pub struct #builder_struct_name <#all_generics_with_bounds> {
        #(#builder_fields)*
      }

      #[allow(non_camel_case_types, dead_code)]
      impl <#all_generics_with_bounds> #builder_struct_name <#all_generic_names> {
        pub fn new() -> #builder_struct_name <#all_generic_names> {
          #builder_struct_name {
            #(#builder_new),*
          }
        }

        #[allow(dead_code)]
        pub fn build(self) -> Result<#props_struct_name <#all_generic_names>, &'static str> {
          #(#build_required)*

          let #builder_struct_name { #(#build_destructuring)* .. } = self;

          Ok(#props_struct_name {
            #(#names)*
          })
        }

        #(#builder_fns)*
      }
    };

    struct_tokens
  };

  let a = quote! {
    #props_struct

    #[allow(non_camel_case_types, non_snake_case, unused_variables, clippy::too_many_arguments, dead_code)]
    #vis fn #name <#all_generics_with_bounds> (
      document: ::std::rc::Rc<web_sys::Document>,
      // parent: Option<&::web_sys::Element>,
      parent: Option<::std::rc::Rc<web_sys::Element>>,
      props: #props_struct_name <#all_generic_names>
    ) #ret #(+ #lifetimes)*
    #where_clause
    {
      let #props_struct_name { #(#prop_names)* .. } = props;

      #[allow(non_camel_case_types, non_snake_case, unused_variables, clippy::too_many_arguments)]
      #vis fn #name_inner <#all_generics_with_bounds> (
        document: ::std::rc::Rc<web_sys::Document>,
        // parent: Option<&::web_sys::Element>,
        parent: Option<::std::rc::Rc<web_sys::Element>>,
        #(#prop_names_and_types)*
      ) #ret #(+ #lifetimes2)*
      #where_clause
      {
        #(#stmts)*
      }

      return #name_inner(document, parent, #(#prop_names)*);
    }
  };

  return a.into();
}
