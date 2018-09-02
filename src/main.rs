use std::fmt;

#[derive(Debug)]
enum Event {
  Registered,
}

trait Extension {
  // Name of the extension
  fn name(&self) -> &'static str;

  // Invoke when the extension is loaded
  fn init(&self, engine: &Engine);

  // Invoke when events are emitted
  fn handle(&self, event: Event, engine: &Engine);
}

struct Editor {}

impl Editor {
  fn new() -> Editor {
    Editor {}
  }
}

impl Extension for Editor {
  fn name(&self) -> &'static str {
    "core.editor"
  }

  fn init(&self, engine: &Engine) {
    println!("Editor is initialized.");

    println!("Host: {}", engine.host);
  }

  fn handle(&self, event: Event, engine: &Engine) {
    println!("Incoming Event: {:?}", event);
  }
}

struct Engine {
  pub host: String,
  pub extensions: Vec<Box<Extension>>,
}

impl Engine {
  fn new(host: &str) -> Engine {
    Engine {
      host: String::from(host),
      extensions: vec![],
    }
  }

  fn start(&self) {
    for extension in self.extensions.iter() {
      extension.init(self);

      let name = &*extension.name();
      println!("[Extension::Init] {}", name);
    }
  }
}

fn main() {
  let mut engine = Engine::new("phoom.in.th");

  let editor = Editor::new();

  engine.extensions.push(Box::new(editor));

  engine.start();
}
