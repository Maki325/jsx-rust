use convert_case::{Case::Pascal, Casing};
use proc_macro2::Ident;
use proc_macro_error::abort;
use quote::ToTokens;
use syn::{
  parse::{Parse, ParseStream},
  punctuated::Punctuated,
  FnArg, GenericArgument, ItemFn, Pat, PatIdent, PathArguments, ReturnType, Token, Type,
  TypeParamBound,
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

fn get_prop_type(ty: Type) -> PropType {
  let bounds = match &ty {
    Type::TraitObject(tto) => &tto.bounds,
    _ => return PropType::Type(ty),
  };

  // Souldn't it *always* be the first bound?
  // So we shouldn't need to loop through all bounds
  for bound in bounds {
    let trait_bound = match bound {
      TypeParamBound::Trait(trait_bound) => trait_bound,
      _ => continue,
    };
    for segment in &trait_bound.path.segments {
      if !segment.ident.eq("ReadSignal") && !segment.ident.eq("IntoReadSignal") {
        continue;
      }
      let ab = if let PathArguments::AngleBracketed(ab) = &segment.arguments {
        ab
      } else {
        abort!(segment.arguments, "Should be `dyn ReadSignal<T>`!");
      };

      return PropType::ReadSignal(PropTypeReadSignal {
        generic: ab.args.clone(),
      });
    }
  }
  return PropType::Type(ty);
}

impl Prop {
  pub fn new(arg: FnArg) -> Self {
    let typed: syn::PatType = if let FnArg::Typed(ty) = arg {
      ty
    } else {
      abort!(arg, "receiver not allowed in `fn`");
    };

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

    let ty = get_prop_type(ty);

    Self {
      // prop_opts,
      name,
      ty,
    }
  }
}
