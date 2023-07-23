use jsx::signal::ReadSignal;
use jsx_macros::{component, view};
use wasm_bindgen::JsValue;
use web_sys::{console, Event};

#[component]
pub fn TestComponent(
  count: dyn IntoReadSignal<i32>,
  last_count: dyn IntoReadSignal<i32>,
  some_number: i32,
  some_opt_number: Option<i32>,
) -> Result<web_sys::Element, JsValue> {
  console::log_1(&"Start".into());

  return Ok(view! {
    <div>
      <span>"Value: " {count} "!"</span>
      <span>"Value: " {some_number} "!"</span>
    </div>
  });
}

#[component]
pub fn ExampleComponent<F: Fn(Event) + 'static>(
  count: dyn IntoReadSignal<i32>,
  on_click: F,
) -> Result<web_sys::Element, JsValue> {
  console::log_1(&"Start".into());

  return Ok(view! {
    <div on::click=on_click style="user-select: none;">
      <span>"Value: " {count} "!"</span>
    </div>
  });
}
