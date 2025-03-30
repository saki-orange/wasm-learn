use clap::Parser;
use wasmtime::component::bindgen;

use component::greet::greetable::Host;

bindgen!({
    path: "../greet/wit",
    world: "hello-world"
});

struct Greet {
  name: String,
}

impl Greet {
  fn new(name: String) -> Self {
    Self { name }
  }
}

impl Host for Greet {
  fn name(&mut self) -> String {
    self.name.clone()
  }
  fn greet(&mut self, name: String) -> String {
    format!("Hello, from {name}!")
  }
}

#[derive(Parser, Debug)]
struct Cli {
  wasm_file: String,
}

fn main() {
  let cli = Cli::parse();

  if let Err(e) = start(cli) {
    println!("{e}");
  }
}

fn start(cli: Cli) -> anyhow::Result<()> {
  let engin = wasmtime::Engine::default();
  let mut linker = wasmtime::component::Linker::new(&engin);
  let mut store = wasmtime::Store::new(&engin, Greet::new("Native code".to_string()));

  let component = wasmtime::component::Component::from_file(&engin, cli.wasm_file)?;

  HelloWorld::add_to_linker(&mut linker, |greet: &mut Greet| greet)?;
  let hello_world = HelloWorld::instantiate(&mut store, &component, &linker)?;

  let message = hello_world.component_greet_sayable().call_say(&mut store)?;
  println!("{message}");
  Ok(())
}
