use jsx::signal::{create_signal, ReadSignal};
use jsx_macros::{component, view};
use wasm_bindgen::JsValue;
use web_sys::console;

#[component]
pub fn ExampleComponent(count: dyn ReadSignal<i32>) -> Result<web_sys::Element, JsValue> {
  console::log_1(&"Start".into());

  // create user interfaces with the declarative `view!` macro
  return Ok(view! {
    <div>
      <span>"Value: " {count} "!"</span>
    </div>
  });
}

pub fn test(document: &::web_sys::Document) {
  ExampleComponent(
    document,
    ExampleComponentProps {
      count: 5,
      phantom_0: std::marker::PhantomData,
    },
  );

  let (count, _set_count) = create_signal(0);
  ExampleComponent(
    document,
    ExampleComponentProps {
      count,
      phantom_0: std::marker::PhantomData,
    },
  );
}
