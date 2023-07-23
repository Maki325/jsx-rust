#[macro_export]
macro_rules! mount_to_body {
  ($el:tt) => {
    let window = web_sys::window().expect("no global `window` exists");
    let document = &window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    body.append_child(&view! $el.into())?;
  };
}
