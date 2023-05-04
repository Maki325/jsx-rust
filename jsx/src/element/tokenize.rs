use super::{element::ElementValue, Element};
use quote::quote;

impl Element {
  pub fn to_client_tokens(&self) -> proc_macro2::TokenStream {
    return match self {
      Element::Element(ElementValue { name, children, .. }) => {
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
        quote! {
          {
            let element = document.create_element(#name)?;
            #(#children)*

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
            let value: String = #path.get().to_string();
            let text = std::rc::Rc::new(std::cell::RefCell::new(document.create_text_node(value.as_str())));
            count.signal.borrow_mut().listeners.push(text.clone());
            text.clone()
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
