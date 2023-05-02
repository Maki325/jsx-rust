use crate::element::Updateable;

// #[derive(Clone)]
struct Signal<T: Clone> {
  pub value: T,
  // pub listeners: Vec<Box<dyn Fn(T)>>,
  // pub listeners: Vec<Box<&'static dyn Updateable<T>>>,
  // pub listeners: Vec<Box<&'static dyn Updateable<T>>>,
  // pub listeners: Vec<Box<&'static Updateable<T>>>,
}

impl<T: Clone> Signal<T> {
  fn get(&self) -> T {
    return self.value.clone();
  }
  fn set(&mut self, val: T) {
    self.value = val;
  }
}

pub struct GetSignal<T: Clone> {
  signal: &'static Signal<T>,
}
pub struct SetSignal<T: Clone> {
  signal: Box<Signal<T>>,
}

macro_rules! impl_get_functions {
  ($struct:ident, $function_name:ident) => {
    #[cfg(not(feature = "stable"))]
    impl<T: Clone> FnOnce<()> for $struct<T> {
      type Output = T;

      // Required method
      #[inline(always)]
      extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        return self.$function_name();
      }
    }

    #[cfg(not(feature = "stable"))]
    impl<T: Clone> FnMut<()> for $struct<T> {
      // Required method
      #[inline(always)]
      extern "rust-call" fn call_mut(&mut self, _args: ()) -> Self::Output {
        return self.$function_name();
      }
    }

    #[cfg(not(feature = "stable"))]
    impl<T: Clone> Fn<()> for $struct<T> {
      // Required method
      #[inline(always)]
      extern "rust-call" fn call(&self, _args: ()) -> Self::Output {
        return self.$function_name();
      }
    }
  };
}

macro_rules! impl_set_functions {
  ($struct:ident, $function_name:ident) => {
    #[cfg(not(feature = "stable"))]
    impl<T: Clone> FnOnce<(T,)> for $struct<T> {
      type Output = ();

      // Required method
      #[inline(always)]
      extern "rust-call" fn call_once(self, args: (T,)) -> Self::Output {
        return self.$function_name(args.0);
      }
    }

    #[cfg(not(feature = "stable"))]
    impl<T: Clone> FnMut<(T,)> for $struct<T> {
      // Required method
      #[inline(always)]
      extern "rust-call" fn call_mut(&mut self, args: (T,)) -> Self::Output {
        return self.$function_name(args.0);
      }
    }

    #[cfg(not(feature = "stable"))]
    impl<T: Clone> Fn<(T,)> for $struct<T> {
      // Required method
      #[inline(always)]
      extern "rust-call" fn call(&self, args: (T,)) -> Self::Output {
        return self.$function_name(args.0);
      }
    }
  };
}

impl_get_functions!(GetSignal, get);
impl_set_functions!(SetSignal, set);

impl<T: Clone> GetSignal<T> {
  pub fn get(&self) -> T {
    // pub fn get(&self) -> () {
    // self.value.clone()
    return self.signal.value.clone();
  }
}

impl<T: Clone> SetSignal<T> {
  pub fn set(&self, val: T) {
    // self.value = val;
  }

  pub fn update<F>(&mut self, f: F)
  where
    F: FnOnce(T) -> T,
  {
    // self.value = f(self.value.clone());
  }
}

pub fn create_signal<T: Clone>(value: T) -> (GetSignal<T>, SetSignal<T>) {
  // let signal: &'static mut Signal<T> = Box::leak(Box::new(Signal { value: value }));
  let signal = Box::new(Signal {
    value: value,
    // listeners: Vec::new(),
  });
  let signal: &'static mut Signal<T> = Box::leak(signal);
  // let signal = Box::new(signal);

  let value = (
    GetSignal {
      signal: signal.clone(),
    },
    SetSignal {
      signal: signal.clone(),
    },
  );

  std::mem::forget(signal);

  return value;
}
