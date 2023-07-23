use crate::attribute::AttributeValue;

use super::{element::ElementValue, Element};
use quote::{format_ident, quote};

impl Element {
  pub fn to_client_tokens(&self) -> proc_macro2::TokenStream {
    return match self {
      Element::Element(ElementValue {
        name,
        children,
        attributes,
        is_custom,
      }) => {
        let children = children.iter().map(|child| match child {
          Element::Element(_) | Element::Literal(_) => {
            let child = child.to_client_tokens();
            quote! {
              element.append_child(&#child.into())?;
            }
          }
          Element::Updateable(_) => {
            let child = child.to_client_tokens();
            quote! {
              let child = #child;
              element.append_child(child.borrow().as_ref())?;
            }
          }
        });

        if *is_custom {
          let builder_struct_name = format_ident!("{name}PropsBuilder");
          let component_name = format_ident!("{name}");

          let attributes = attributes.iter().map(|attribute| {
            let value = match &attribute.value {
              AttributeValue::Custom(expr) => expr,
              _ => panic!("Must be a custom attribute!"),
            };

            let attribute_name = &attribute.name;
            let fn_name = format_ident!("set_{attribute_name}");
            quote! {
              builder.#fn_name(#value);
            }
          });

          quote! {
            {
              let mut builder = #builder_struct_name::new();
              #(#attributes)*

              let props = builder.build()?;

              let view = #component_name(document, props)?;
              view
            }
          }
        } else {
          let attributes = attributes
            .iter()
            .map(|attribute| attribute.to_client_tokens());

          quote! {
            {
              let element = document.create_element(#name)?;
              #(#children)*
              #(#attributes)*

              element
            }
          }
        }
      }
      Element::Literal(value) => {
        quote! {
          document.create_text_node(#value)
        }
      }
      Element::Updateable(path) => {
        let path = &path.0.path;
        quote! {
          {
            let ___signal___ = jsx::signal::into_read_signal(#path);
            let ___text___ = std::rc::Rc::new(std::cell::RefCell::new(document.create_text_node(___signal___.get().to_string().as_str())));
            ___signal___.add_listener(___text___.clone());
            ___text___.clone()
          }
        }
      }
    };
  }

  pub fn to_server_tokens(&self) -> proc_macro2::TokenStream {
    return match self {
      Element::Element(ElementValue {
        name,
        children,
        is_custom,
        ..
      }) => {
        let children = children.iter().map(|child| child.to_server_tokens());
        quote! {
          jsx::element::Element::Element({
            jsx::element::ElementValue {
              name: #name.to_string(),
              attributes: Vec::new(),
              children: vec![#(#children),*],
              is_custom: #is_custom,
            }
          })
        }
      }
      Element::Literal(value) => {
        quote! {
          jsx::element::Element::Literal(#value.to_string())
        }
      }
      _ => unimplemented!(),
    };
  }
}
