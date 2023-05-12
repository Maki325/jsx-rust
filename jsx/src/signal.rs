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

#[derive(Clone, Copy)]
pub struct GetSignal<T: ToString + Clone> {
  pub signal: *mut Signal<T>,
}

#[derive(Clone, Copy)]
pub struct ConstGetSignal<T: ToString + Clone> {
  pub value: T,
}

pub trait ReadSignal<T: ToString + Clone> {
  fn get(&self) -> T;

  fn add_listener<U>(&self, listener: Rc<RefCell<U>>)
  where
    U: Updateable<T> + 'static;
}

#[derive(Clone, Copy)]
pub struct SetSignal<T: ToString + Clone> {
  pub signal: *mut Signal<T>,
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

    impl<T: ToString + Clone> ToString for $struct<T> {
      fn to_string(&self) -> String {
        return self.$function_name().to_string();
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
impl_get_functions!(ConstGetSignal, get);
impl_set_functions!(SetSignal, set);

impl<T: ToString + Clone> ReadSignal<T> for GetSignal<T> {
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

impl<T: ToString + Clone> ReadSignal<T> for ConstGetSignal<T> {
  fn get(&self) -> T {
    return self.value.clone();
  }

  fn add_listener<U>(&self, _listener: Rc<RefCell<U>>)
  where
    U: Updateable<T> + 'static,
  {
  }
}

impl<T: ToString + Clone> SetSignal<T> {
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

pub fn create_signal<T: ToString + Clone>(value: T) -> (GetSignal<T>, SetSignal<T>) {
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
      impl From<$from> for ConstGetSignal<$from> {
        fn from(value: $from) -> Self {
          return ConstGetSignal { value };
        }
      }

      impl IntoReadSignal<$from, ConstGetSignal<$from>> for $from {
        fn into(self) -> ConstGetSignal<$from> {
          return ConstGetSignal { value: self };
        }
      }
    )*
  };
}
pub use into_const_read_signal;

pub trait IntoReadSignal<T: ToString + Clone, R> {
  fn into(self) -> R;
}

pub fn create_const_get_signal<T: ToString + Clone>(val: T) -> ConstGetSignal<T> {
  return ConstGetSignal { value: val };
}

pub fn create_get_signal<T: ToString + Clone, R: ReadSignal<T>>(val: R) -> R {
  return val.into();
}

pub fn into_read_signal<T, R, I>(val: I) -> R
where
  T: ToString + Clone,
  I: IntoReadSignal<T, R>,
{
  return val.into();
}

// fn a(document: Document) {
//   let (get, _set) = create_signal(5);

//   let ___text___ = std::rc::Rc::new(std::cell::RefCell::new(document.create_text_node("")));

//   let q = into_read_signal(get);
//   q.add_listener(___text___.clone());

//   let const_get = ConstGetSignal { value: 8 };
//   let const_q = into_read_signal(const_get);
//   const_q.add_listener(___text___.clone());
// }

impl<T: ToString + Clone> IntoReadSignal<T, GetSignal<T>> for GetSignal<T> {
  fn into(self) -> Self {
    return self;
  }
}

impl<T: ToString + Clone> IntoReadSignal<T, ConstGetSignal<T>> for ConstGetSignal<T> {
  fn into(self) -> Self {
    return self;
  }
}

// impl IntoReadSignal<char, ConstGetSignal<char>> for char {
//   fn into(self) -> ConstGetSignal<char> {
//     return ConstGetSignal { value: self };
//   }
// }

// impl IntoReadSignal<&'static str, ConstGetSignal<&'static str>> for &'static str {
//   fn into(self) -> ConstGetSignal<&'static str> {
//     return ConstGetSignal { value: self };
//   }
// }

into_const_read_signal!(
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
