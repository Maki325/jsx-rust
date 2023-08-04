// Change to a Trait that accepts an element, and inserts a T as a child/children

use std::rc::Rc;

use wasm_bindgen::JsValue;

pub trait AppendToElement {
  fn append_to_element(&self, element: &web_sys::Element) -> Result<(), JsValue>;
}

impl AppendToElement for js_sys::Array {
  fn append_to_element(&self, element: &web_sys::Element) -> Result<(), JsValue> {
    return element.append_with_node(&self);
  }
}

macro_rules! impl_append_to_element {
  ($($name:path),+) => {
    $(impl AppendToElement for $name {
      fn append_to_element(&self, element: &web_sys::Element) -> Result<(), JsValue> {
        element.append_child(self)?;

        return Ok(());
      }
    }

    impl AppendToElement for Rc<$name> {
      fn append_to_element(&self, element: &web_sys::Element) -> Result<(), JsValue> {
        element.append_child(self)?;

        return Ok(());
      }
    }

    impl AppendToElement for Vec<$name> {
      fn append_to_element(&self, element: &web_sys::Element) -> Result<(), JsValue> {
        for el in self {
          element.append_child(el)?;
        }

        return Ok(());
      }
    }

    impl AppendToElement for Vec<Rc<$name>> {
      fn append_to_element(&self, element: &web_sys::Element) -> Result<(), JsValue> {
        for el in self {
          element.append_child(el)?;
        }

        return Ok(());
      }
    })*
  };
}

impl_append_to_element!(web_sys::Element, web_sys::Node, web_sys::Text);
