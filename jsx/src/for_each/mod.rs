use std::{collections::HashMap, hash::Hash, rc::Rc};

use wasm_bindgen::JsValue;
use web_sys::{console, Element};

use crate::signal::{
  add_listener_fn,
  create_signal,
  GetSignal,
  IntoReadSignal,
  ReadSignal,
  SetSignal,
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

fn non_zero_value_js_error() -> Result<(), JsValue> {
  Err("NonZeroUsize".into())
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
    type Map<K, T> = HashMap<K, (Rc<web_sys::Element>, SetSignal<T>)>;

    let each = each.into_read_signal();
    let (map, set_map) = create_signal((Map::<K, T>::new(), vec![]));

    let _key = key.clone();
    let _view = view.clone();

    let _set_map = set_map.clone();
    add_listener_fn(&each, move |list: I| {
      console::log_1(&"add_listener_fn".into());
      let key = &_key;
      let view = &_view;
      let set_map = &_set_map;
      let parent = parent.clone();

      let new_count = list.clone().into_iter().count();
      let list = list.into_iter();

      let update = |(mut old_map, old_list): (Map<K, T>, Vec<Rc<Element>>)| -> Result<(Map<K, T>, Vec<Rc<Element>>), JsValue> {
        console::log_1(&"update".into());
        let mut old_list = old_list.into_iter().peekable();
        console::log_1(&"update 1".into());

        let old_count = old_list.clone().count();
        console::log_1(&"update 2".into());

        let mut map: Map<K, T> = HashMap::with_capacity(new_count);
        let mut elements: Vec<Rc<Element>> = vec![];
        console::log_1(&"update 3".into());

        let mut last_advance = 0;
        let mut added_elements = 0;
        
        console::log_1(&"update 4".into());

        for (i, data) in list.enumerate() {
          let k = key(&data);

          console::log_1(&"update inside 1".into());

          match old_map.remove(&k) {
            Some((el, set_signal)) => {
              console::log_1(&"update inside SOME".into());
              set_signal.set(data);
              console::log_1(&"update inside SOME 1".into());
              elements.push(el.clone());
              console::log_1(&"update inside SOME 2".into());
              map.insert(k, (el, set_signal));
              console::log_1(&"update inside SOME 3".into());
            }
            None => {
              console::log_1(&"update inside NONE".into());
              let (get, set) = create_signal(data);

              console::log_1(&"update inside NONE 1".into());
              let el = view(get)?;

              console::log_1(&"update inside NONE 2".into());
              elements.push(el.clone());

              console::log_3(&"update inside NONE 3".into(), &i.into(), &old_count.into());
              if i >= old_count {
                console::log_1(&"update inside NONE Old Count".into());
                if let Some(parent) = &parent {
                  console::log_1(&"update inside NONE Old Count 1".into());
                  parent.append_child(&el)?;
                } else {
                  console::warn_1(&"Warning: `parent` is None!".into());
                  println!("Warning: `parent` is None!");
                }
              } else {
                console::log_4(&"update inside NONE NOT Old Count 1".into(), &i.into(), &added_elements.into(), &last_advance.into());
                let item = if  i == 0 || i - added_elements == last_advance {
                  console::log_1(&"update inside NONE NOT Old Count 1 1".into());
                  old_list.peek()
                } else {
                  console::log_1(&"update inside NONE NOT Old Count 1 2".into());
                  let advance_by = (i - added_elements) - last_advance;

                  last_advance = i - added_elements;
                  console::log_3(&"update inside NONE NOT Old Count 1 2".into(), &advance_by.into(), &last_advance.into());
                  if let Err(err) = old_list.advance_by(advance_by) {
                    console::log_1(&"update inside NONE NOT Old Count 1 2 non_zero_value_js_error".into());
                    non_zero_value_js_error()?;
                  }
                  old_list.peek()
                };
                console::log_1(&"update inside NONE NOT Old Count 2".into());
                let item = item.expect("Must exist!");

                console::log_1(&"update inside NONE NOT Old Count 3".into());
                item.insert_adjacent_element("beforebegin", &*el)?;
              }

              map.insert(k, (el, set));
              added_elements += 1;
            }
          }
        }

        console::log_1(&"update 5".into());

        Ok((map, elements))
      };

      set_map.update(|(old_map, old_list)| {
        let value = match update((old_map, old_list)) {
          Ok(value) => value,
          Err(err) => {
            console::log_1(&err);
            panic!("Error: {:?}", err);
          }
        };

        value
      });
    });

    let mut elements: Vec<Rc<web_sys::Element>> = vec![];
    let mut map = HashMap::new();

    for data in each.get().into_iter() {
      let k = key(&data);
      let (get, set) = create_signal(data);
      let el = view(get)?;

      map.insert(k, (el.clone(), set));
      elements.push(el);
    }
    set_map.set((map, elements.clone()));

    return Ok(elements);
  }

  return ForInner(document, parent, each, view, key);
}
