use jsx_macros::my_macro;

fn main() {
  my_macro! {
    println!("From lib");
  };
}
