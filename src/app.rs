use std::rc::Rc;

use serde::{Deserialize, Serialize};

use jsx::{for_each::*, signal::create_signal};
use jsx_macros::{component, view};
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone, Serialize, Deserialize)]
struct Test {
  id: i32,
  value: i32,
}

#[component]
pub fn App() -> Result<Rc<web_sys::Element>, JsValue> {
  let (data, set_data) = create_signal(vec![
    Test { id: 1, value: 1 },
    Test { id: 2, value: 2 },
    Test { id: 3, value: 3 },
    Test { id: 4, value: 4 },
  ]);

  let _set_data = set_data.clone();
  let insert_start = move |_| {
    _set_data.update(|x| {
      let mut x = x.clone();

      x.insert(
        0,
        Test {
          id: x.len() as i32 + 1,
          value: x.len() as i32 + 1,
        },
      );

      let a = serde_json::to_string(&x).unwrap();
      console::log_1(&a.into());

      x
    });
  };

  let _set_data = set_data.clone();
  let insert_end = move |_| {
    _set_data.update(|x| {
      let mut x = x.clone();

      x.push(Test {
        id: x.len() as i32 + 1,
        value: x.len() as i32 + 1,
      });

      x
    });
  };

  let _set_data = set_data.clone();
  let insert_almost_end = move |_| {
    _set_data.update(|x| {
      let mut x = x.clone();

      x.insert(
        x.len() - 1,
        Test {
          id: x.len() as i32 + 1,
          value: x.len() as i32 + 1,
        },
      );

      x
    });
  };

  let _set_data = set_data.clone();
  let increase_by_one = move |_| {
    _set_data.update(|x| {
      x.into_iter()
        .enumerate()
        .map(|(i, x)| Test {
          id: x.id,
          value: if i % 2 == 0 { x.value + 1 } else { x.value },
        })
        .collect()
      // x.into_iter()
      //   .map(|x| Test {
      //     id: x.id,
      //     value: x.value + 1,
      //   })
      //   .collect()
    });
  };

  return Ok(view! {
    <div>
      <h1>"Hellooooo, How are you?a bcdefghi"</h1>

      <button on::click=insert_start>
        "Insert At Start"
      </button>
      <button on::click=insert_end>
        "Insert At End"
      </button>
      <button on::click=insert_almost_end>
        "Insert At Almost"
      </button>

      <button on::click=increase_by_one>
        "Increase by One"
      </button>
      <For
        each=data
        key=|x| x.id
        view=move |x| {
          Ok(view! { <div>"Id: " {x.id} " : " {x.value}</div> })
        }
      />
    </div>
  });
}
