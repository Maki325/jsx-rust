use crate::updateable::Updateable;
use std::{cell::RefCell, rc::Rc};

pub struct Signal<T: Clone> {
  pub value: T,
  pub listeners: Vec<Rc<RefCell<dyn Updateable<T>>>>,
}

impl<T: Clone> Signal<T> {
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

#[derive(Clone, Copy)]
pub struct GetSignal<T: Clone> {
  pub signal: *mut Signal<T>,
}

#[derive(Clone, Copy)]
pub struct ConstGetSignal<T: Clone> {
  pub value: T,
}

pub trait ReadSignal<T: Clone> {
  fn get(&self) -> T;

  fn add_listener<U>(&self, listener: Rc<RefCell<U>>)
  where
    U: Updateable<T> + 'static;
}

#[derive(Clone, Copy)]
pub struct SetSignal<T: Clone> {
  pub signal: *mut Signal<T>,
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

    impl<T: Clone + ToString> ToString for $struct<T> {
      fn to_string(&self) -> String {
        return self.$function_name().to_string();
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
impl_get_functions!(ConstGetSignal, get);
impl_set_functions!(SetSignal, set);

// impl<T: Clone> GetSignal<T> {
//   pub fn add_listener_fn<F>(&self, listener: F)
//   where
//     F: Fn(T) + 'static,
//   {
//     let signal = unsafe { self.signal.as_mut().expect("Signal should exist!") };

//     self.add_listener(Rc::new(RefCell::new(listener)));
//     // self.add_listener(listener);
//     // let l: U = listener.into();

//     // signal.listeners.push(Rc::new(RefCell::new(l)));
//   }
// }

pub fn add_listener_fn<T, R, F>(signal: &R, listener: F)
where
  T: Clone,
  R: ReadSignal<T>,
  F: Fn(T) + 'static,
{
  signal.add_listener(Rc::new(RefCell::new(listener)));
}

impl<T: Clone> ReadSignal<T> for GetSignal<T> {
  fn get(&self) -> T {
    let signal = unsafe { self.signal.as_ref().expect("Signal should exist!") };
    return signal.get();
  }

  fn add_listener<U>(&self, listener: Rc<RefCell<U>>)
  where
    U: Updateable<T> + 'static,
  {
    let signal = unsafe { self.signal.as_mut().expect("Signal should exist!") };
    signal.listeners.push(listener);
  }
}

impl<T: Clone> ReadSignal<T> for ConstGetSignal<T> {
  fn get(&self) -> T {
    return self.value.clone();
  }

  fn add_listener<U>(&self, _listener: Rc<RefCell<U>>)
  where
    U: Updateable<T> + 'static,
  {
  }
}

impl<T: Clone> SetSignal<T> {
  pub fn set(&self, val: T) {
    let signal = unsafe { self.signal.as_mut().expect("Signal should exist!") };
    signal.set(val);
  }

  pub fn update<F>(&self, f: F)
  where
    F: FnOnce(T) -> T,
  {
    let signal = unsafe { self.signal.as_ref().expect("Signal should exist!") };
    let old_value = signal.get();
    self.set(f(old_value));
  }
}

pub fn create_signal<T: Clone>(value: T) -> (GetSignal<T>, SetSignal<T>) {
  let a = Box::new(Signal {
    value: value,
    listeners: Vec::new(),
  });
  let signal = Box::leak(a);

  return (GetSignal { signal }, SetSignal { signal });
}

#[macro_export]
macro_rules! into_const_read_signal {
  ($($from:ty),*) => {
    $(
      impl From<$from> for jsx::signal::ConstGetSignal<$from> {
        fn from(value: $from) -> Self {
          return jsx::signal::ConstGetSignal { value };
        }
      }

      impl jsx::signal::IntoReadSignal<$from, jsx::signal::ConstGetSignal<$from>> for $from {
        fn into_read_signal(self) -> jsx::signal::ConstGetSignal<$from> {
          return jsx::signal::ConstGetSignal { value: self };
        }
      }
    )*
  };
  ($cgs:ident; $($from:ty),*) => {
    $(
      impl From<$from> for crate::signal::ConstGetSignal<$from> {
        fn from(value: $from) -> Self {
          return crate::signal::ConstGetSignal { value };
        }
      }

      impl crate::signal::IntoReadSignal<$from, crate::signal::ConstGetSignal<$from>> for $from {
        fn into_read_signal(self) -> crate::signal::ConstGetSignal<$from> {
          return crate::signal::ConstGetSignal { value: self };
        }
      }
    )*
  };
}
pub use into_const_read_signal;

/**
 * Try convert IntoReadSignal to use and maybe return a borrowed R?
 */
pub trait IntoReadSignal<T: Clone, R> {
  fn into_read_signal(self) -> R;
}

impl<T: Clone, R: ReadSignal<T>> IntoReadSignal<T, R> for R {
  fn into_read_signal(self) -> Self {
    return self;
  }
}

// impl From<u8> for ConstGetSignal<u8> {
//   fn from(value: u8) -> Self {
//     return ConstGetSignal { value };
//   }
// }

// impl From<u8> for dyn ReadSignal<u8> {
//   fn from(value: u8) -> Self {
//     return ConstGetSignal { value };
//   }
// }

pub fn into_read_signal<T, R, I>(val: I) -> R
where
  T: Clone,
  I: IntoReadSignal<T, R>,
{
  return val.into_read_signal();
}

into_const_read_signal!(
  ConstGetSignal;
  i8,
  i16,
  i32,
  i64,
  i128,
  isize, // signed
  u8,
  u16,
  u32,
  u64,
  u128,
  usize, // unsigned
  f32,
  f64, // floating point
  bool,
  char,
  String,
  &'static str
);
