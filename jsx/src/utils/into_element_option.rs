use std::rc::Rc;

pub trait IntoElementOption {
  fn into_element_option(self) -> Option<Rc<web_sys::Element>>;
}

impl IntoElementOption for Option<Rc<web_sys::Element>> {
  fn into_element_option(self) -> Option<Rc<web_sys::Element>> {
    self
  }
}

impl IntoElementOption for Rc<web_sys::Element> {
  fn into_element_option(self) -> Option<Rc<web_sys::Element>> {
    Some(self)
  }
}
