use super::{Attribute, AttributeValue};
use quote::quote;

impl Attribute {
  pub fn to_client_tokens(&self) -> proc_macro2::TokenStream {
    let value = match &self.value {
      AttributeValue::Function(func) => func,
      _ => unimplemented!(),
    };
    let name = &self.name;

    return quote! {
      let closure: Closure<dyn FnMut(Event)> = Closure::new(#value);
      element.add_event_listener_with_callback(#name, closure.as_ref().unchecked_ref())?;
      closure.forget();
    };
  }

  pub fn to_server_tokens(&self) -> proc_macro2::TokenStream {
    unimplemented!();
  }
}
