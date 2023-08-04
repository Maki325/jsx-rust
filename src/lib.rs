use jsx::{
  mount_to_body,
  signal::{create_signal, ReadSignal},
};
use jsx_macros::view;
use wasm_bindgen::prelude::*;
use web_sys::console;
mod app;
mod component;

use app::*;
use component::*;

#[derive(Clone)]
struct Test {
  a: i32,
  b: i32,
}
impl ToString for Test {
  fn to_string(&self) -> String {
    return format!("Test {{a: {} b: {}}}", self.a, self.b);
  }
}
mod a {
  use jsx::into_const_read_signal;

  into_const_read_signal!(crate::Test);
}

#[allow(dead_code)]
fn example_second_ticker() -> Result<(), JsValue> {
  let (count, set_count) = create_signal(0);

  console::log_1(&"Start".into());

  let window = web_sys::window().expect("no global `window` exists");
  let document = ::std::rc::Rc::new(window.document().expect("should have a document on window"));
  let body = document.body().expect("document should have a body");

  let el = view! {
    <div>
      "Hellooooo, How are you?a bcdefghi"
      <br/>
      <h1>3.14159265358</h1>
      <h2>"Seconds 1: "{count}</h2>
      <h2>"Seconds 2: "{count}</h2>
    </div>
  };

  body.append_child(&el)?;

  console::log_1(&"Appended Element".into());

  let cb = Closure::new(move || {
    console::log_1(&"Updating Element".into());
    set_count.update(|x| x + 1);
    console::log_1(&"Updated Element".into());
  });

  let interval_id = set_interval(&window, &cb, 1000);
  console::log_2(&"Set interval:".into(), &interval_id.into());
  cb.forget();

  Ok(())
}

#[allow(dead_code)]
fn example_element_names() -> Result<(), JsValue> {
  console::log_1(&"Start".into());

  mount_to_body!({
    <div>
      <br/> <span-a>"Dash: <span-a>Dash</span-a>"</span-a>
      <br/> <span:a>"Colon: <span:a>Colon</span:a>"</span:a>
      <br/> <span::a>"Double colon: <span::a>Double colon</span::a>"</span::a>
    </div>
  });

  console::log_1(&"Appended Element".into());

  return Ok(());
}

#[allow(dead_code)]
fn example_counter() -> Result<(), JsValue> {
  let (value, set_value) = create_signal(0);

  let clear = move |_| set_value(0);
  let decrement = move |_| set_value.update(|value| value - 1);
  let increment = move |_| set_value.update(|value| value + 1);

  console::log_1(&"Start".into());

  // create user interfaces with the declarative `view!` macro
  mount_to_body!({
    <div>
      <button on::click=clear>"Clear"</button>
      <button on::click=decrement>"-1"</button>
      <span>"Value 1: " {value} "!"</span>
      <span>"Value 2: " {value} "!"</span>
      <span>"Value 3: " {value} "!"</span>
      <button on::click=increment>"+1"</button>
    </div>
  });

  console::log_1(&"Appended Element".into());

  return Ok(());
}

#[allow(dead_code)]
fn example_const_read_signals() -> Result<(), JsValue> {
  console::log_1(&"Start".into());

  // For values like ints and floatss
  // We need to specify the exact type
  // Because the compiler can't infer it exactly
  // I.e.
  // It doesn't know it it's a 8, 16, 32, 64 or a 128 bit int
  let num: i32 = 5;

  let static_str = "It works!";
  let string = String::from("STRIIIINNNGGG!!");
  let custon_struct = Test { a: 1, b: 2 };

  mount_to_body!({
    <div>
      <span>"num 1: " {num}</span> <br/>
      <span>"num 2: " {num}</span> <br/>
      <span>"num 3: " {num}</span> <br/>
      <span>"static_str: \"" {static_str} "\""</span> <br/>
      <span>"string: \"" {string} "\""</span> <br/>
      <span>"custon_struct: " {custon_struct}</span> <br/>
    </div>
  });

  console::log_1(&"Appended Element".into());

  return Ok(());
}

#[allow(dead_code)]
fn example_component() -> Result<(), JsValue> {
  let (value, set_value) = create_signal(0);

  let clear = move |_| {
    set_value(if value.get() == 5 { 0 } else { 5 });
  };
  console::log_1(&"Start".into());

  mount_to_body!({
    <div>
      <ExampleComponent count=value on_click=clear />
    </div>
  });

  return Ok(());
}

#[allow(dead_code)]
fn hr() -> Result<(), JsValue> {
  mount_to_body!({ <hr/> });
  return Ok(());
}

#[allow(dead_code)]
fn milela() -> Result<(), JsValue> {
  mount_to_body!({
    <div>
      <h1 style="font-size: 100px;">"Milela!"</h1>
      <hr/>
    </div>
  });
  return Ok(());
}

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
  // milela()?;

  // example_second_ticker()?;
  // hr()?;
  // example_element_names()?;
  // hr()?;
  // example_counter()?;
  // hr()?;
  // example_const_read_signals()?;
  // hr()?;
  // example_component()?;

  mount_to_body!({<App/>});

  Ok(())
}

#[allow(dead_code)]
fn set_interval(window: &web_sys::Window, f: &Closure<dyn FnMut()>, timeout_ms: i32) -> i32 {
  window
    .set_interval_with_callback_and_timeout_and_arguments_0(f.as_ref().unchecked_ref(), timeout_ms)
    .expect("should register `setTimeout` OK")
}
