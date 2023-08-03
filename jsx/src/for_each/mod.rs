use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

// use jsx_macros::{component, view};
use wasm_bindgen::JsValue;
use web_sys::{console, Element};
// use web_sys::{console, Event};

use crate::{
  into_const_read_signal,
  signal::{
    add_listener_fn, create_signal, GetSignal, IntoReadSignal, ReadSignal, SetSignal, Signal,
  },
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

#[derive(Clone)]
struct A(u32);
into_const_read_signal!(a;A);

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
    // let (map, set_map) = create_signal(HashMap::<K, &'static Element>::new());
    // type TyyS<K, T> = SetSignal<HashMap<K, (GetSignal<T>, SetSignal<T>)>>;
    // type TyyG<K, T> = GetSignal<HashMap<K, (GetSignal<T>, SetSignal<T>)>>;
    // let (map, set_map): (TyyG<K, T>, TyyS<K, T>) =
    //   create_signal(HashMap::<K, (GetSignal<T>, SetSignal<T>)>::new());

    // let a = document.create_text_node("a");
    // let a = Rc::new(a);
    // parent.unwrap().append_child(&a)?;

    type Map<K, T> = HashMap<K, (Rc<Element>, SetSignal<T>)>;

    let each = each.into_read_signal();
    let (map, set_map) = create_signal((Map::<K, T>::new(), vec![]));

    // let _key = key.clone();
    let _key = key.clone();
    let _view = view.clone();

    let _set_map = set_map.clone();
    add_listener_fn(&each, move |list: I| {
      console::log_1(&"add_listener_fn".into());
      // _set_map.set(HashMap::new());
      // _set_map.update(|data| data);
      let key = &_key;
      let view = &_view;
      let set_map = &_set_map;
      let parent = parent.clone();

      let new_count = list.clone().into_iter().count();
      let list = list.into_iter();

      // let update: impl FnOnce() -> (Map<K, T>, Vec<Rc<Element>>) =
      let update = |(mut old_map, old_list): (Map<K, T>, Vec<Rc<Element>>)| -> Result<(Map<K, T>, Vec<Rc<Element>>), JsValue> {
        console::log_1(&"update".into());
        let mut old_list = old_list.into_iter().peekable();
        console::log_1(&"update 1".into());

        let old_count = old_list.clone().count();
        console::log_1(&"update 2".into());

        let mut map: Map<K, T> = HashMap::with_capacity(new_count);
        let mut elements: Vec<Rc<Element>> = vec![];
        console::log_1(&"update 3".into());
        // let mut map = map.clone();

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

                // item.insert_before(node, child);
                console::log_1(&"update inside NONE NOT Old Count 3".into());
                item.insert_adjacent_element("beforebegin", &*el)?;
              }

              // let el = view(get_signal(data.clone()))?;
              // let el = Rc::new(el);
              // parent.unwrap().append_child(&el)?;
              map.insert(k, (el, set));
              added_elements += 1;
            }
          }
          // if let Some((el, set_signal)) = map.get(&k) {
          //   set_signal.set(data);
          //   // println!("A");
          //   // set_signal()
          // }
          // .expect("Must exist!")
          // .1
          // .set(element);
          // let v = view(element)?;

          // map.insert(k, v);
        }

        console::log_1(&"update 5".into());

        Ok((map, elements))

        // let mut new_map = HashMap::<K, &'static Element>::new();

        // for el in x.into_iter() {
        //   let k = key(&el);
        //   let v = view(el)?;

        //   new_map.insert(k, v);
        // }

        // for (k, v) in map.iter() {
        //   if !new_map.contains_key(k) {
        //     v.remove();
        //   }
        // }

        // for (k, v) in new_map.iter() {
        //   if !map.contains_key(k) {
        //     parent.unwrap().append_child(v)?;
        //   }
        // }

        // *map = new_map;

        // Ok(map)
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

        // let old_list = old_list.into_iter().peekable();

        // let mut map: Map<K, T> = HashMap::with_capacity(list.count());
        // let mut elements: Vec<Rc<Element>> = vec![];
        // // let mut map = map.clone();

        // let mut last_advance = 0;
        // let mut added_elements = 0;

        // for (i, data) in list.enumerate() {
        //   let k = key(&data);

        //   match old_map.remove(&k) {
        //     Some((el, set_signal)) => {
        //       set_signal.set(data);
        //       elements.push(el.clone());
        //       map.insert(k, (el, set_signal));
        //     }
        //     None => {
        //       let (get, set) = create_signal(data);

        //       let el = Rc::new(view(get)?);

        //       elements.push(el.clone());

        //       if i == list.count() - 1 {
        //         if let Some(parent) = parent {
        //           parent.append_child(&el)?;
        //         } else {
        //           println!("Warning: parent is None!");
        //         }
        //       } else {
        //         let item = if i - added_elements - 1 == last_advance {
        //           old_list.peek()
        //         } else {
        //           let advance_by = (i - added_elements - 1) - last_advance;
        //           last_advance = i - added_elements - 1;
        //           old_list.advance_by(advance_by);
        //           old_list.peek()
        //         };
        //         let item = item.expect("Must exist!");

        //         // item.insert_before(node, child);
        //         item.insert_adjacent_element("beforebegin", el);
        //       }

        //       // let el = view(get_signal(data.clone()))?;
        //       // let el = Rc::new(el);
        //       // parent.unwrap().append_child(&el)?;
        //       map.insert(k, (el, set));
        //       added_elements += 1;
        //     }
        //   }
        //   if let Some((el, set_signal)) = map.get(&k) {
        //     set_signal.set(data);
        //     // println!("A");
        //     // set_signal()
        //   }
        //   // .expect("Must exist!")
        //   // .1
        //   // .set(element);
        //   // let v = view(element)?;

        //   // map.insert(k, v);
        // }

        // (map, elements)

        // // let mut new_map = HashMap::<K, &'static Element>::new();

        // // for el in x.into_iter() {
        // //   let k = key(&el);
        // //   let v = view(el)?;

        // //   new_map.insert(k, v);
        // // }

        // // for (k, v) in map.iter() {
        // //   if !new_map.contains_key(k) {
        // //     v.remove();
        // //   }
        // // }

        // // for (k, v) in new_map.iter() {
        // //   if !map.contains_key(k) {
        // //     parent.unwrap().append_child(v)?;
        // //   }
        // // }

        // // *map = new_map;

        // // Ok(map)
      });
      // console::log_1(&"Updating Element".into());
      // console::log_1(&x.to_string().into());
      // console::log_1(&"Updated Element".into());
    });
    // each.add_listener(Rc::new(RefCell::new()));

    let mut elements: Vec<Rc<web_sys::Element>> = vec![];
    // let mut map = HashMap::<K, &'static Element>::new();
    // let mut map = HashMap::<K, (GetSignal<T>, SetSignal<T>)>::new();
    let mut map = HashMap::new();

    for el in each.get().into_iter() {
      let k = key(&el);
      let (get, set) = create_signal(el);
      // let v = view(get)?;
      // let v = Box::new(view(get)?);
      // let v = Box::leak(v);
      // let v = Rc::new(view(get)?);
      let v = view(get)?;

      map.insert(k, (v.clone(), set));
      elements.push(v);
    }
    set_map.set((map, elements.clone()));
    // each.get().into_iter().for_each(|x| {
    //   view(x);
    //   // console::log_1(&"Updating Element".into());
    //   // console::log_1(&x.to_string().into());
    //   // console::log_1(&"Updated Element".into());
    // });

    // each.get()

    // let a = {
    //   let element: web_sys::Element = document.create_element("div")?;
    //   // element.append_with_node(nodes)
    //   element.append_child(
    //     &{
    //       let element = document.create_element("span")?;
    //       element.append_child(&document.create_text_node("Value: ").into())?;
    //       let child = {
    //         let ___signal___ = super::signal::into_read_signal(each);
    //         let ___text___ = std::rc::Rc::new(std::cell::RefCell::new(
    //           document.create_text_node(___signal___.get().to_string().as_str()),
    //         ));
    //         ___signal___.add_listener(___text___.clone());
    //         ___text___.clone()
    //       };
    //       element.append_child(child.borrow().as_ref())?;
    //       element.append_child(&document.create_text_node("!").into())?;
    //       element
    //     }
    //     .into(),
    //   )?;

    //   // let node: web_sys::Node = element.into();
    //   // node
    //   element
    // };

    return Ok(elements);
  }

  return ForInner(document, parent, each, view, key);
}
