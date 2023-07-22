use jsx::signal::{create_signal, ReadSignal};
use jsx_macros::{component, view};
use wasm_bindgen::JsValue;
use web_sys::console;

use crate::Info;

#[component]
pub fn TestComponent(
  count: dyn IntoReadSignal<i32>,
  last_count: dyn IntoReadSignal<i32>,
  some_number: i32,
  some_opt_number: Option<i32>,
) -> Result<web_sys::Element, JsValue> {
  console::log_1(&"Start".into());

  // let some_number = some_opt_number.unwrap_or(0);

  return Ok(view! {
    <div>
      <span>"Value: " {count} "!"</span>
      <span>"Value: " {some_number} "!"</span>
    </div>
  });
}

#[component]
pub fn ExampleComponent(count: dyn IntoReadSignal<i32>) -> Result<web_sys::Element, JsValue> {
  console::log_1(&"Start".into());

  // let some_number = some_opt_number.unwrap_or(0);

  return Ok(view! {
    <div>
      <span>"Value: " {count} "!"</span>
    </div>
  });
}

#[allow(dead_code)]
pub fn example_component_test(Info(_, document, body): Info) -> Result<(), JsValue> {
  console::log_1(&"Start".into());

  let val = view! {
    <div>
      <span>"Our example component!"</span>
      <br/>
      <ExampleComponent count=5 />
    </div>
  };

  console::log_1(&"Created Element".into());

  body.append_child(&val.into())?;

  console::log_1(&"Appended Element".into());

  return Ok(());
}

pub fn test(document: &::web_sys::Document) -> Result<(), JsValue> {
  TestComponent(
    document,
    TestComponentProps {
      count: 5,
      last_count: 3,
      some_number: 7,
      some_opt_number: None,
      ___phantom_0___: std::marker::PhantomData,
      ___phantom_1___: std::marker::PhantomData,
    },
  )?;

  let (count, _set_count) = create_signal(0);
  TestComponent(
    document,
    TestComponentProps {
      count,
      last_count: 3,
      some_number: 7,
      some_opt_number: None,
      ___phantom_0___: std::marker::PhantomData,
      ___phantom_1___: std::marker::PhantomData,
    },
  )?;

  Ok(())
}

pub fn example(document: &::web_sys::Document) -> Result<(), JsValue> {
  ExampleComponent(
    document,
    ExampleComponentProps {
      count: 5,
      ___phantom_0___: std::marker::PhantomData,
    },
  )?;

  Ok(())
}
