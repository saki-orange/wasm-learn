#[allow(warnings)]
mod bindings;

use bindings::component::greet::greetable::{greet, name};
use bindings::exports::component::greet::sayable::Guest;

struct Component;

impl Guest for Component {
  fn say() -> String {
    let name = name();
    let greetings = greet(&name);
    let mut buffer = Vec::new();

    ferris_says::say(&greetings, 80, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
  }
}

bindings::export!(Component with_types_in bindings);
