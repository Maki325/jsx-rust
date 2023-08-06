use std::{
  rc::Rc,
  sync::atomic::{AtomicUsize, Ordering},
};

use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

use jsx::{for_each::*, signal::create_signal};
use jsx_macros::{component, view};
use wasm_bindgen::JsValue;

static ADJECTIVES: &[&str] = &[
  "pretty",
  "large",
  "big",
  "small",
  "tall",
  "short",
  "long",
  "handsome",
  "plain",
  "quaint",
  "clean",
  "elegant",
  "easy",
  "angry",
  "crazy",
  "helpful",
  "mushy",
  "odd",
  "unsightly",
  "adorable",
  "important",
  "inexpensive",
  "cheap",
  "expensive",
  "fancy",
];

static COLOURS: &[&str] = &[
  "red", "yellow", "blue", "green", "pink", "brown", "purple", "brown", "white", "black", "orange",
];

static NOUNS: &[&str] = &[
  "table", "chair", "house", "bbq", "desk", "car", "pony", "cookie", "sandwich", "burger", "pizza",
  "mouse", "keyboard",
];

static ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

fn create_data(count: usize) -> Vec<RowData> {
  let mut thread_rng = thread_rng();

  let mut data = Vec::with_capacity(count);
  for _ in 0..count {
    let adjective = ADJECTIVES
      .choose(&mut thread_rng)
      .expect("Will exist as array is not empty!");
    let colour = COLOURS
      .choose(&mut thread_rng)
      .expect("Will exist as array is not empty!");
    let noun = NOUNS
      .choose(&mut thread_rng)
      .expect("Will exist as array is not empty!");

    let capacity = adjective.len() + 1 + colour.len() + 1 + noun.len();

    let mut label = String::with_capacity(capacity);
    label.push_str(adjective);
    label.push(' ');
    label.push_str(colour);
    label.push(' ');
    label.push_str(noun);

    data.push(RowData {
      id: ID_COUNTER.load(Ordering::Relaxed),
      label,
    });

    ID_COUNTER.store(ID_COUNTER.load(Ordering::Relaxed) + 1, Ordering::Relaxed);
  }

  return data;
}

#[derive(Clone, Serialize, Deserialize)]
struct RowData {
  id: usize,
  label: String,
}

#[component]
pub fn Benchmark() -> Result<Rc<web_sys::Element>, JsValue> {
  let (data, set_data) = create_signal::<Vec<RowData>>(vec![]);

  let create_1k = move |_| {
    set_data.set(create_data(1_000));
  };
  let create_10k = move |_| {
    set_data.set(create_data(10_000));
  };

  let update_every_tenth = move |_| {
    set_data.update(|data| {
      let mut data = data.clone();
      for i in (0..data.len()).step_by(10) {
        data[i].label.push_str(" !!!");
      }
      data
    });
  };

  let update_every_tenth_with = move |_| {
    set_data.with(|data| {
      for i in (0..data.len()).step_by(10) {
        data[i].label.push_str(" !!!");
      }
    });
  };

  let append_1k = move |_| {
    set_data.with(|data| {
      data.extend(create_data(1_000));
    });
  };

  let clear = move |_| {
    set_data.set(vec![]);
  };

  let swap_rows = move |_| {
    set_data.with(|data| {
      if data.len() > 998 {
        data.swap(1, 998);
      }
    });
  };

  return Ok(view! {
    <div>
      <h1>"Hellooooo, How are you?a bcdefghijklmopqrst"</h1>

      <button id="run" on::click=create_1k>
        "Create 1,000 rows"
      </button>
      <button id="runlots" on::click=create_10k>
        "Create 10,000 rows"
      </button>

      <button id="update-tenth" on::click=update_every_tenth>
        "Update every 10th row"
      </button>
      <button id="update" on::click=update_every_tenth_with>
        "Update every 10th row with"
      </button>

      <button id="add" on::click=append_1k>
        "Append 1,000 rows"
      </button>
      <button id="clear" on::click=clear>
        "Clear"
      </button>

      <button id="swaprows" on::click=swap_rows>
        "Swap"
      </button>
      <table>
        <tbody>
          <For
            each=data
            key=|x| x.id
            view=move |x| {
              Ok(view! {
                <tr>
                  <td>{x.id}</td>
                  <td>{x.label}</td>
                  <td>"Remove"</td>
                </tr>
              })
            }
          />
        </tbody>
      </table>
    </div>
  });
}
