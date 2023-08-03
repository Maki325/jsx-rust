// pub trait IntoElementOption<'a> {
//   fn into_element_option(self) -> Option<&'a web_sys::Element>;
// }

// impl<'a> IntoElementOption<'a> for Option<&'a web_sys::Element> {
//   fn into_element_option(self) -> Option<&'a web_sys::Element> {
//     self
//   }
// }

// impl<'a> IntoElementOption<'a> for &'a Option<&'a web_sys::Element> {
//   fn into_element_option(self) -> Option<&'a web_sys::Element> {
//     None
//   }
// }

// impl<'a> IntoElementOption<'a> for &'a web_sys::Element {
//   fn into_element_option(self) -> Option<&'a web_sys::Element> {
//     return Some(self);
//   }
// }

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

// impl<'a> IntoElementOption for &'a Option<Rc<web_sys::Element>> {
//   fn into_element_option(self) -> Option<Rc<web_sys::Element>> {
//     None
//   }
// }

// impl<'a> IntoElementOption<'a> for &'a web_sys::Element {
//   fn into_element_option(self) -> Option<&'a web_sys::Element> {
//     return Some(self);
//   }
// }
