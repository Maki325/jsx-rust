#[macro_export]
macro_rules! mount_to_body {
  ($el:tt) => {
    let window = web_sys::window().expect("no global `window` exists");
    let document = ::std::rc::Rc::new(window.document().expect("should have a document on window"));
    let body = document.body().expect("document should have a body");
    // let element: Option<&web_sys::Element> = None;
    let element: Option<::std::rc::Rc<web_sys::Element>> = None;

    let el = view!$el;
    body.append_child(&el)?;
  };
}
