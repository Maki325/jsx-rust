pub trait IntoElementArray {
  fn into_element_array(self) -> js_sys::Array;
}

impl IntoElementArray for js_sys::Array {
  fn into_element_array(self) -> js_sys::Array {
    return self;
  }
}

impl IntoElementArray for web_sys::Element {
  fn into_element_array(self) -> js_sys::Array {
    let array = js_sys::Array::new();
    let node: web_sys::Node = self.into();
    array.push(&node);

    return array;
  }
}

impl IntoElementArray for Vec<web_sys::Element> {
  fn into_element_array(self) -> js_sys::Array {
    return self
      .into_iter()
      .map(|el| {
        let node: web_sys::Node = el.into();
        node
      })
      .collect::<js_sys::Array>();
  }
}

impl IntoElementArray for web_sys::Node {
  fn into_element_array(self) -> js_sys::Array {
    let array = js_sys::Array::new();
    array.push(&self);

    return array;
  }
}

impl IntoElementArray for Vec<web_sys::Node> {
  fn into_element_array(self) -> js_sys::Array {
    return self.iter().collect::<js_sys::Array>();
  }
}

impl IntoElementArray for web_sys::Text {
  fn into_element_array(self) -> js_sys::Array {
    let array = js_sys::Array::new();
    let node: web_sys::Node = self.into();
    array.push(&node);

    return array;
  }
}

impl IntoElementArray for Vec<web_sys::Text> {
  fn into_element_array(self) -> js_sys::Array {
    return self
      .into_iter()
      .map(|el| {
        let node: web_sys::Node = el.into();
        node
      })
      .collect::<js_sys::Array>();
  }
}
