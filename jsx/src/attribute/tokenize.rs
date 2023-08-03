use super::{Attribute, AttributeValue};
use quote::quote;

impl Attribute {
  pub fn to_client_tokens(&self) -> proc_macro2::TokenStream {
    let name = &self.name;

    return match &self.value {
      AttributeValue::Function(func) => {
        quote! {
          let closure: wasm_bindgen::closure::Closure<dyn FnMut(web_sys::Event)> = wasm_bindgen::closure::Closure::new(#func);
          element.add_event_listener_with_callback(#name, wasm_bindgen::JsCast::unchecked_ref(closure.as_ref()))?;
          closure.forget();
        }
      }
      AttributeValue::Custom(expr) => {
        quote! {
          element.set_attribute(#name, &#expr.to_string())?;
        }
      }
      _ => unimplemented!("Attribute::to_client_tokens"),
    };
  }

  pub fn to_server_tokens(&self) -> proc_macro2::TokenStream {
    unimplemented!();
  }
}
