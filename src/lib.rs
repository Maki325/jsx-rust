use jsx::signal::create_signal;
use jsx_macros::view;
use wasm_bindgen::prelude::*;
use web_sys::console;

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
  // Use `web_sys`'s global `window` function to get a handle on the global
  // window object.
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let body = document.body().expect("document should have a body");

  let (count, set_count) = create_signal(0);

  console::log_1(&"Start".into());

  let val = view! {
    <div>
      "Hellooooo, How are you?a bcde"
      <br/>
      <h1>3.14159265358</h1>
      <h2>"Seconds 1: "{count}</h2>
      <h2>"Seconds 2: "{count}</h2>
    </div>
  };

  console::log_1(&"Created Element".into());

  body.append_child(&val.into())?;

  console::log_1(&"Appended Element".into());

  let cb = Closure::new(move || {
    console::log_1(&"Updating Element".into());
    set_count.update(|x| x + 1);
    console::log_1(&"Updated Element".into());
  });
  let interval_id = set_interval(&window, &cb, 1000);
  console::log_2(&"Set interval:".into(), &interval_id.into());

  cb.forget();

  return Ok(());
}

fn set_interval(window: &web_sys::Window, f: &Closure<dyn FnMut()>, timeout_ms: i32) -> i32 {
  window
    .set_interval_with_callback_and_timeout_and_arguments_0(f.as_ref().unchecked_ref(), timeout_ms)
    .expect("should register `setTimeout` OK")
}
