use jsx::signal::create_signal;
use jsx_macros::view;
use wasm_bindgen::prelude::*;

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
  // Use `web_sys`'s global `window` function to get a handle on the global
  // window object.
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let body = document.body().expect("document should have a body");

  let (count, _set_count) = create_signal(0);

  let _test_value = count();

  let val = view! {
    <div>
      "Hellooooo, How are you?a"
      <br/>
      <h1>3.14159265358</h1>
    </div>
  };
  body.append_child(&val)?;

  Ok(())
}
