use crate::updateable::Updateable;
use std::{cell::RefCell, rc::Rc};

pub struct Signal<T: ToString + Clone> {
  pub value: T,
  pub listeners: Vec<Rc<RefCell<dyn Updateable<T>>>>,
}

impl<T: ToString + Clone> Signal<T> {
  fn get(&self) -> T {
    return self.value.clone();
  }
  fn set(&mut self, val: T) {
    self.value = val.clone();

    for listener in &self.listeners {
      listener.borrow_mut().update(val.clone());
    }
  }
}

#[derive(Clone)]
pub struct GetSignal<T: ToString + Clone> {
  pub signal: Rc<RefCell<Signal<T>>>,
}

#[derive(Clone)]
pub struct SetSignal<T: ToString + Clone> {
  pub signal: Rc<RefCell<Signal<T>>>,
}

macro_rules! impl_get_functions {
  ($struct:ident, $function_name:ident) => {
    #[cfg(not(feature = "stable"))]
    impl<T: ToString + Clone> FnOnce<()> for $struct<T> {
      type Output = T;

      // Required method
      #[inline(always)]
      extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        return self.$function_name();
      }
    }

    #[cfg(not(feature = "stable"))]
    impl<T: ToString + Clone> FnMut<()> for $struct<T> {
      // Required method
      #[inline(always)]
      extern "rust-call" fn call_mut(&mut self, _args: ()) -> Self::Output {
        return self.$function_name();
      }
    }

    #[cfg(not(feature = "stable"))]
    impl<T: ToString + Clone> Fn<()> for $struct<T> {
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
    impl<T: ToString + Clone> FnOnce<(T,)> for $struct<T> {
      type Output = ();

      // Required method
      #[inline(always)]
      extern "rust-call" fn call_once(self, args: (T,)) -> Self::Output {
        return self.$function_name(args.0);
      }
    }

    #[cfg(not(feature = "stable"))]
    impl<T: ToString + Clone> FnMut<(T,)> for $struct<T> {
      // Required method
      #[inline(always)]
      extern "rust-call" fn call_mut(&mut self, args: (T,)) -> Self::Output {
        return self.$function_name(args.0);
      }
    }

    #[cfg(not(feature = "stable"))]
    impl<T: ToString + Clone> Fn<(T,)> for $struct<T> {
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

impl<T: ToString + Clone> GetSignal<T> {
  pub fn get(&self) -> T {
    return self.signal.borrow().get();
  }
}

impl<T: ToString + Clone> SetSignal<T> {
  pub fn set(&self, val: T) {
    self.signal.borrow_mut().set(val);
  }

  pub fn update<F>(&self, f: F)
  where
    F: FnOnce(T) -> T,
  {
    let old_value = self.signal.borrow().get();
    self.set(f(old_value));
  }
}

pub fn create_signal<T: ToString + Clone>(value: T) -> (GetSignal<T>, SetSignal<T>) {
  let signal = Rc::new(RefCell::new(Signal {
    value: value,
    listeners: Vec::new(),
  }));

  return (
    GetSignal {
      signal: signal.clone(),
    },
    SetSignal {
      signal: signal.clone(),
    },
  );
}
