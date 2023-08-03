use web_sys::Text;

pub trait Updateable<T: Clone> {
  fn update(&mut self, new_value: T);
}

impl<T: Clone + ToString> Updateable<T> for Text {
  fn update(&mut self, new_value: T) {
    self.set_data(new_value.to_string().as_str());
  }
}

impl<T: Clone, F> Updateable<T> for F
where
  F: Fn(T),
{
  fn update(&mut self, new_value: T) {
    self(new_value);
  }
}

// impl<F> Runnable for F
// where
//   F: Fn(T),
// {
//   fn run(&self, x: &i32) {
//     self(x);
//   }
// }
