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

impl AppendToElement for web_sys::Element {
  fn append_to_element(&self, element: &web_sys::Element) -> Result<(), JsValue> {
    element.append_child(self)?;

    return Ok(());
  }
}

impl AppendToElement for Rc<web_sys::Element> {
  fn append_to_element(&self, element: &web_sys::Element) -> Result<(), JsValue> {
    element.append_child(self)?;

    return Ok(());
  }
}

impl AppendToElement for Vec<web_sys::Element> {
  fn append_to_element(&self, element: &web_sys::Element) -> Result<(), JsValue> {
    for el in self {
      element.append_child(el)?;
    }

    return Ok(());
  }
}

impl AppendToElement for Vec<Rc<web_sys::Element>> {
  fn append_to_element(&self, element: &web_sys::Element) -> Result<(), JsValue> {
    for el in self {
      element.append_child(el)?;
    }

    return Ok(());
  }
}

impl AppendToElement for web_sys::Node {
  fn append_to_element(&self, element: &web_sys::Element) -> Result<(), JsValue> {
    element.append_child(self)?;

    return Ok(());
  }
}

impl AppendToElement for Rc<web_sys::Node> {
  fn append_to_element(&self, element: &web_sys::Element) -> Result<(), JsValue> {
    element.append_child(self)?;

    return Ok(());
  }
}

impl AppendToElement for Vec<web_sys::Node> {
  fn append_to_element(&self, element: &web_sys::Element) -> Result<(), JsValue> {
    for el in self {
      element.append_child(el)?;
    }

    return Ok(());
  }
}

impl AppendToElement for Vec<Rc<web_sys::Node>> {
  fn append_to_element(&self, element: &web_sys::Element) -> Result<(), JsValue> {
    for el in self {
      element.append_child(el)?;
    }

    return Ok(());
  }
}

impl AppendToElement for web_sys::Text {
  fn append_to_element(&self, element: &web_sys::Element) -> Result<(), JsValue> {
    element.append_child(self)?;

    return Ok(());
  }
}

impl AppendToElement for Rc<web_sys::Text> {
  fn append_to_element(&self, element: &web_sys::Element) -> Result<(), JsValue> {
    element.append_child(self)?;

    return Ok(());
  }
}

impl AppendToElement for Vec<web_sys::Text> {
  fn append_to_element(&self, element: &web_sys::Element) -> Result<(), JsValue> {
    for el in self {
      element.append_child(el)?;
    }

    return Ok(());
  }
}

impl AppendToElement for Vec<Rc<web_sys::Text>> {
  fn append_to_element(&self, element: &web_sys::Element) -> Result<(), JsValue> {
    for el in self {
      element.append_child(el)?;
    }

    return Ok(());
  }
}
