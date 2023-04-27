use jsx::element::Element;
use jsx_macros::view;

fn test() -> Element {
  return view! {
    <div>
      "Hellooooo, How are you?a" 3.14159265358
    </div>
  };
}

fn test_string() -> Element {
  return view! {
    "Šell"
  };
}

fn main() {
  println!("{:#?}", test());
  println!("{:#?}", test_string());
}
