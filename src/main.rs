use jsx::element::Element;
use jsx_macros::ssr;

fn test() -> Element {
  return ssr! {
    <div>
      "Hellooooo, How are you?a"
      <br/>
      3.14159265358
    </div>
  };
}

fn test_string() -> Element {
  return ssr! {
    "Šell"
  };
}

fn main() {
  println!("{:#?}", test());
  println!("{:#?}", test_string());
}
