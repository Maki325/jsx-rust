use convert_case::{Case::Pascal, Casing};
use proc_macro2::{Ident, TokenStream};
use proc_macro_error::abort;
use quote::{quote, ToTokens};
use syn::{
  parse::{Parse, ParseStream},
  punctuated::Punctuated,
  FnArg, GenericArgument, ItemFn, Pat, PatIdent, PathArguments, ReturnType, Token, Type,
  TypeParamBound, TypeTraitObject,
};

pub struct Component {
  pub body: ItemFn,
  pub name: Ident,
  pub ret: ReturnType,
  pub props: Vec<Prop>,
}

impl Parse for Component {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let item: ItemFn = input.parse()?;

    let (_, generics, where_clause) = item.sig.generics.split_for_impl();

    let props = item
      .sig
      .inputs
      .clone()
      .into_iter()
      .map(Prop::new)
      .collect::<Vec<_>>();

    Ok(Component {
      name: convert_into_pascal_case(&item.sig.ident),
      ret: item.sig.output.clone(),
      body: item,
      props,
    })
  }
}

pub fn convert_into_pascal_case(name: &Ident) -> Ident {
  let name_str = name.to_string();
  if name_str.is_case(Pascal) {
    name.clone()
  } else {
    Ident::new(&name_str.to_case(Pascal), name.span())
  }
}

#[derive(Clone)]
pub struct PropTypeReadSignal {
  pub generic: Punctuated<GenericArgument, Token![,]>,
}
#[derive(Clone)]
pub enum PropType {
  Type(Type),
  ReadSignal(PropTypeReadSignal),
}

pub struct Prop {
  // prop_opts: PropOpt,
  pub name: PatIdent,
  pub ty: PropType,
}

impl Prop {
  pub fn new(arg: FnArg) -> Self {
    let typed: syn::PatType = if let FnArg::Typed(ty) = arg {
      ty
    } else {
      abort!(arg, "receiver not allowed in `fn`");
    };

    // let prop_opts = PropOpt::from_attributes(&typed.attrs).unwrap_or_else(|e| {
    //   // TODO: replace with `.unwrap_or_abort()` once https://gitlab.com/CreepySkeleton/proc-macro-error/-/issues/17 is fixed
    //   abort!(e.span(), e.to_string());
    // });

    let name = if let Pat::Ident(i) = *typed.pat {
      i
    } else {
      abort!(
        typed.pat,
        "only `prop: bool` style types are allowed within the \
          `#[component]` macro"
      );
    };

    let ty = *typed.ty;

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

    fn get_prop_type(ty: Type) -> PropType {
      if let Type::TraitObject(TypeTraitObject { bounds, dyn_token }) = &ty {
        for bound in bounds {
          // match bound {
          //   TypeParamBound::Trait(_) => println!("Trait"),
          //   TypeParamBound::Lifetime(_) => println!("Lifetime"),
          //   TypeParamBound::Verbatim(_) => println!("Verbatim"),
          //   _ => println!("Other"),
          // }
          if let TypeParamBound::Trait(t) = bound {
            // println!("Trait: {:#?}", t.to_token_stream());
            // println!("Trait lifetimes: {:#?}", t.lifetimes.to_token_stream());
            // println!("Trait modifier: {:#?}", t.modifier.to_token_stream());
            // println!("Trait path: {:#?}", t.path.to_token_stream());

            for segment in &t.path.segments {
              // println!("Trait ident: {:#?}", segment.ident.to_token_stream());
              // println!("Trait segment: {:#?}", segment.arguments.to_token_stream());

              if segment.ident.eq("ReadSignal") {
                // println!("Trait ident is ReadSignal!!!");
                // match &segment.arguments {
                //   PathArguments::None => println!("Trait segment None!"),
                //   PathArguments::AngleBracketed(_) => println!("Trait segment AngleBracketed!"),
                //   PathArguments::Parenthesized(_) => println!("Trait segment Parenthesized!"),
                // }

                let ab = if let PathArguments::AngleBracketed(ab) = &segment.arguments {
                  ab
                } else {
                  abort!(segment.arguments, "Should be ReadSignal<T>!");
                };

                // println!(
                //   "Trait segment.arguments ab.args {:#?}",
                //   ab.args.to_token_stream()
                // );
                return PropType::ReadSignal(PropTypeReadSignal {
                  generic: ab.args.clone(),
                });
              }
              // segment.a
            }
            // println!("Trait: {:#?}", t.lifetimes.to_token_stream());
            // println!("Trait: {:#?}", t.lifetimes.to_token_stream());
          }
          // println!("Bound: {:#?}", bound);
        }
        return PropType::Type(ty);
        // println!("Bound: {:#?}", bounds.());
        // if bounds.len() != 1 {
        //   abort!(
        //     bounds,
        //     "only one trait bound is allowed within the `#[component]` macro"
        //   );
        // }
      }
      return PropType::Type(ty);
    }

    let ty = get_prop_type(ty);

    Self {
      // prop_opts,
      name,
      ty,
    }
  }
}
