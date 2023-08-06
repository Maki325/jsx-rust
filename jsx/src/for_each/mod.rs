use std::{collections::HashMap, hash::Hash, rc::Rc};

use wasm_bindgen::JsValue;
use web_sys::{console, Element};

use crate::signal::{
  add_listener_fn, create_signal, GetSignal, IntoReadSignal, ReadSignal, SetSignal,
};

#[allow(non_camel_case_types, dead_code)]
pub struct ForProps<
  T: Clone + 'static,
  I: IntoIterator<Item = T> + Clone,
  Each: ReadSignal<I>,
  IntoEach: IntoReadSignal<I, Each>,
  View: Fn(GetSignal<T>) -> Result<Rc<web_sys::Element>, JsValue> + Clone + 'static,
  Key: Fn(&T) -> K + Clone + 'static,
  K: Eq + Hash + Clone + 'static,
> {
  phantom_t: std::marker::PhantomData<T>,
  phantom_i: std::marker::PhantomData<I>,
  each: IntoEach,
  phantom_each: std::marker::PhantomData<Each>,
  view: View,
  key: Key,
}

#[allow(non_camel_case_types, dead_code)]
pub struct ForPropsBuilder<
  T: Clone + 'static,
  I: IntoIterator<Item = T> + Clone,
  Each: ReadSignal<I>,
  IntoEach: IntoReadSignal<I, Each>,
  View: Fn(GetSignal<T>) -> Result<Rc<web_sys::Element>, JsValue> + Clone + 'static,
  Key: Fn(&T) -> K + Clone + 'static,
  K: Eq + Hash + Clone + 'static,
> {
  phantom_t: std::marker::PhantomData<T>,
  phantom_i: std::marker::PhantomData<I>,
  each: Option<IntoEach>,
  phantom_each: std::marker::PhantomData<Each>,
  view: Option<View>,
  key: Option<Key>,
}

#[allow(non_camel_case_types, dead_code)]
impl<
    T: Clone + 'static,
    I: IntoIterator<Item = T> + Clone,
    Each: ReadSignal<I>,
    IntoEach: IntoReadSignal<I, Each>,
    View: Fn(GetSignal<T>) -> Result<Rc<web_sys::Element>, JsValue> + Clone + 'static,
    Key: Fn(&T) -> K + Clone + 'static,
    K: Eq + Hash + Clone + 'static,
  > ForPropsBuilder<T, I, Each, IntoEach, View, Key, K>
{
  pub fn new() -> ForPropsBuilder<T, I, Each, IntoEach, View, Key, K> {
    ForPropsBuilder {
      each: None,
      phantom_each: std::marker::PhantomData,
      phantom_t: std::marker::PhantomData,
      phantom_i: std::marker::PhantomData,
      key: None,
      view: None,
    }
  }

  #[allow(dead_code)]
  pub fn build(self) -> Result<ForProps<T, I, Each, IntoEach, View, Key, K>, &'static str> {
    let each = match self.each {
      Some(val) => val,
      None => return Err("Required prop `each` not set"),
    };
    let key = match self.key {
      Some(val) => val,
      None => return Err("Required prop `key` not set"),
    };
    let view = match self.view {
      Some(val) => val,
      None => return Err("Required prop `view` not set"),
    };
    let ForPropsBuilder { .. } = self;
    Ok(ForProps {
      each,
      phantom_each: std::marker::PhantomData,
      phantom_t: std::marker::PhantomData,
      phantom_i: std::marker::PhantomData,
      key,
      view,
    })
  }

  #[allow(non_camel_case_types, dead_code)]
  pub fn set_each(&mut self, each: IntoEach) -> &mut Self {
    self.each = Some(each);
    self
  }

  #[allow(non_camel_case_types, dead_code)]
  pub fn set_key(&mut self, key: Key) -> &mut Self {
    self.key = Some(key);
    self
  }

  #[allow(non_camel_case_types, dead_code)]
  pub fn set_view(&mut self, view: View) -> &mut Self {
    self.view = Some(view);
    self
  }
}

#[allow(
  non_camel_case_types,
  non_snake_case,
  unused_variables,
  clippy::too_many_arguments,
  dead_code
)]
pub fn For<
  T: Clone + 'static,
  I: IntoIterator<Item = T> + Clone,
  Each: ReadSignal<I>,
  IntoEach: IntoReadSignal<I, Each>,
  View: Fn(GetSignal<T>) -> Result<Rc<web_sys::Element>, JsValue> + Clone + 'static,
  Key: Fn(&T) -> K + Clone + 'static,
  K: Eq + Hash + Clone + 'static,
>(
  document: Rc<web_sys::Document>,
  parent: Option<Rc<::web_sys::Element>>,
  props: ForProps<T, I, Each, IntoEach, View, Key, K>,
) -> Result<Vec<Rc<web_sys::Element>>, JsValue> {
  let ForProps {
    each, view, key, ..
  } = props;

  #[allow(
    non_camel_case_types,
    non_snake_case,
    unused_variables,
    clippy::too_many_arguments
  )]
  pub fn ForInner<
    T: Clone + 'static,
    I: IntoIterator<Item = T> + Clone,
    Each: ReadSignal<I>,
    IntoEach: IntoReadSignal<I, Each>,
    View: Fn(GetSignal<T>) -> Result<Rc<web_sys::Element>, JsValue> + Clone + 'static,
    Key: Fn(&T) -> K + Clone + 'static,
    K: Eq + Hash + Clone + 'static,
  >(
    document: Rc<web_sys::Document>,
    parent: Option<Rc<::web_sys::Element>>,
    each: IntoEach,
    view: View,
    key: Key,
  ) -> Result<Vec<Rc<web_sys::Element>>, JsValue> {
    type Position = usize;
    type Map<K, T> = HashMap<K, (Rc<web_sys::Element>, SetSignal<T>, Position)>;

    let each = each.into_read_signal();
    let (map, set_map) = create_signal(Map::<K, T>::new());

    let _key = key.clone();
    let _view = view.clone();

    let _set_map = set_map.clone();
    add_listener_fn(&each, move |list: I| {
      let key = &_key;
      let view = &_view;
      let set_map = &_set_map;
      let parent = parent.clone();

      let new_count = list.clone().into_iter().count();

      if new_count == 0 {
        set_map.with(|old_map| {
          old_map.values().for_each(|(element, _, _)| {
            element.remove();
          });
          old_map.clear();
        });

        return;
      }

      let update = |mut old_map: Map<K, T>| -> Result<Map<K, T>, JsValue> {
        let mut map: Map<K, T> = HashMap::with_capacity(new_count);
        let mut elements: Vec<Rc<Element>> = vec![];

        let mut last: Option<Rc<Element>> = None;

        for (i, data) in list.into_iter().enumerate() {
          let k = key(&data);

          let el = match old_map.remove(&k) {
            Some((el, set_signal, old_position)) => {
              set_signal.set(data);
              elements.push(el.clone());
              map.insert(k, (el.clone(), set_signal, i));

              if old_position != i {
                if i == 0 {
                  if let Some(parent) = &parent {
                    parent.insert_adjacent_element("afterbegin", &*el)?;
                  } else {
                    println!("Warning: `parent` is None!");
                  }
                } else {
                  let last = last.expect("Last must exist!");
                  last.insert_adjacent_element("afterend", &*el)?;
                }
              }

              el
            }
            None => {
              let (get, set) = create_signal(data);

              let el = view(get)?;

              elements.push(el.clone());

              if i == 0 {
                if let Some(parent) = &parent {
                  parent.insert_adjacent_element("afterbegin", &*el)?;
                } else {
                  println!("Warning: `parent` is None!");
                }
              } else {
                let last = last.expect("Last must exist!");
                last.insert_adjacent_element("afterend", &*el)?;
              }

              map.insert(k, (el.clone(), set, i));

              el
            }
          };

          last = Some(el);
        }

        for (key, (el, _, _)) in old_map.into_iter() {
          if !map.contains_key(&key) {
            el.remove();
          }
        }

        Ok(map)
      };

      set_map.update(|old_map| {
        let value = match update(old_map) {
          Ok(value) => value,
          Err(err) => {
            console::error_2(&"Update ForEach error:".into(), &err);
            panic!("Error: {:?}", err);
          }
        };

        value
      });
    });

    let mut elements: Vec<Rc<web_sys::Element>> = vec![];
    let mut map = HashMap::new();

    for (i, data) in each.get().into_iter().enumerate() {
      let k = key(&data);
      let (get, set) = create_signal(data);
      let el = view(get)?;

      map.insert(k, (el.clone(), set, i));
      elements.push(el);
    }
    set_map.set(map);

    return Ok(elements);
  }

  return ForInner(document, parent, each, view, key);
}
