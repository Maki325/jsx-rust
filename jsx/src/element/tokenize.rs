use crate::{attribute::AttributeValue, element::path_element::PathElement};

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
              let _document = document.clone();
              jsx::utils::AppendToElement::append_to_element(&#child, &element)?;
              let document = _document;
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
              let _document = document.clone();
              builder.#fn_name(#value);
              let document = _document;
            }
          });

          quote! {
            {
              let mut builder = #builder_struct_name::new();
              #(#attributes)*

              let props = builder.build()?;

              // let view = #component_name(document.clone(), jsx::utils::IntoElementOption::into_element_option(&element), props)?;
              let view = #component_name(document.clone(), jsx::utils::IntoElementOption::into_element_option(element.clone()), props)?;
              view
            }
          }
        } else {
          let attributes = attributes
            .iter()
            .map(|attribute| attribute.to_client_tokens());

          quote! {
            {
              let element = ::std::rc::Rc::new(document.create_element(#name)?);
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
        // Sooo
        // For something like PATH we do what we've done before
        // BUT for Field we do something different
        // We convert the base into the signal
        // And use a custom add_Listener fn to set the data
        // I think we can do that at least lol

        match path {
          PathElement::Field(field) => {
            let base = &field.base;
            let member = &field.member;

            // let a = A(0u32);
            // let (map, set_map) = create_signal(a);
            // let ___signal___ = crate::signal::into_read_signal(map);
            // // fn get_data<I, O>(val: I) -> O {
            // //   return val.0;
            // // }
            // let ___text___ = std::rc::Rc::new(std::cell::RefCell::new(
            //   document.create_text_node(
            //     crate::signal::ReadSignal::get(&___signal___)
            //       .0
            //       .to_string()
            //       .as_str(),
            //   ),
            // ));
            // let ___text___2 = ___text___.clone();
            // crate::signal::add_listener_fn(&___signal___, move |val| {
            //   let a = val.0;
            //   ___text___2.borrow_mut().set_data(a.to_string().as_str());
            // });
            // let b = ___text___.clone();

            quote! {
              {
                let ___signal___ = jsx::signal::into_read_signal(#base.clone());
                let ___text___ = std::rc::Rc::new(std::cell::RefCell::new(document.create_text_node(jsx::signal::ReadSignal::get(&___signal___).#member.to_string().as_str())));
                let ___text___2 = ___text___.clone();
                jsx::signal::add_listener_fn(&___signal___, move |val| {
                  ___text___2.borrow_mut().set_data(val.#member.to_string().as_str());
                });
                ___text___.clone()
              }
            }
          }
          PathElement::Path(path) => {
            let path = &path.path;
            quote! {
              {
                let ___signal___ = jsx::signal::into_read_signal(#path);
                let ___text___ = std::rc::Rc::new(std::cell::RefCell::new(document.create_text_node(jsx::signal::ReadSignal::get(&___signal___).to_string().as_str())));
                jsx::signal::ReadSignal::add_listener(&___signal___, ___text___.clone());
                ___text___.clone()
              }
            }
          }
        }

        // // let base = *path.0.base;
        // // let path = &path.0.member;
        // quote! {
        //   // {
        //   //   let ___signal___ = jsx::signal::into_read_signal(#path);
        //     // let ___text___ = std::rc::Rc::new(std::cell::RefCell::new(document.create_text_node(jsx::signal::ReadSignal::get(&___signal___).to_string().as_str())));
        //   //   jsx::signal::ReadSignal::add_listener(&___signal___, ___text___.clone());
        //   //   ___text___.clone()
        //   // }
        //   {
        //     let ___text___ = std::rc::Rc::new(std::cell::RefCell::new(document.create_text_node("Test")));
        //     ___text___.clone()
        //   }
        // }
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
