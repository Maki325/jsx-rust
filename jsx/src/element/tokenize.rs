use super::{element::ElementValue, Element};
use quote::quote;

impl Element {
  pub fn to_client_tokens(&self) -> proc_macro2::TokenStream {
    return match self {
      Element::Element(ElementValue {
        name,
        children,
        attributes,
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
      Element::Literal(value) => {
        quote! {
          document.create_text_node(#value)
        }
      }
      Element::Updateable(path) => {
        let path = &path.0.path;
        quote! {
          {
            let ___text___ = std::rc::Rc::new(std::cell::RefCell::new(document.create_text_node(#path().to_string().as_str())));
            jsx::signal::ReadSignal::add_listener(&#path, ___text___.clone());
            ___text___.clone()
          }
        }
      }
    };
  }

  pub fn to_server_tokens(&self) -> proc_macro2::TokenStream {
    return match self {
      Element::Element(ElementValue { name, children, .. }) => {
        let children = children.iter().map(|child| child.to_server_tokens());
        quote! {
          jsx::element::Element::Element({
            jsx::element::ElementValue {
              name: #name.to_string(),
              attributes: Vec::new(),
              children: vec![#(#children),*],
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
