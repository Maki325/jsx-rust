use jsx::element::Element;
use jsx_macros::view;

fn test() -> Element {
  return view! {
    <h1>
    </h1>
  };
}

fn main() {
  let view = test();
  println!("{:#?}", view);
}
