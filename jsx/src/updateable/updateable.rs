use web_sys::Text;

pub trait Updateable<T: ToString + Clone> {
  fn update(&mut self, new_value: T);
}

impl<T: ToString + Clone> Updateable<T> for Text {
  fn update(&mut self, new_value: T) {
    self.set_data(new_value.to_string().as_str());
  }
}
