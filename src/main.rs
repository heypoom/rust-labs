use std::fmt;

#[derive(Debug)]
enum Event {
  ExtensionReady(&'static str),
}

#[derive(Clone)]
struct ExtensionMeta {
  pub name: &'static str,
  pub display_name: &'static str,
  pub version: &'static str,
}

trait Extension {
  // Metadata of the extension
  fn meta(&self) -> ExtensionMeta;

  // Invoke when the extension is loaded
  fn init(&self, engine: &Engine);

  // Invoke when events are emitted
  fn handle(&self, event: &Event, engine: &Engine);
}

struct Editor {}

impl Editor {
  fn new() -> Editor {
    Editor {}
  }
}

impl Extension for Editor {
  fn meta(&self) -> ExtensionMeta {
    ExtensionMeta {
      name: "core.editor",
      display_name: "Editor",
      version: "v0.0.1",
    }
  }

  fn init(&self, engine: &Engine) {
    println!("Editor is initialized.");

    println!("Host: {}", engine.host);

    let ready_event = Event::ExtensionReady(self.meta().name);
    engine.emit(ready_event);
  }

  fn handle(&self, event: &Event, engine: &Engine) {
    println!("Incoming Event: {:?} @ {}", event, engine.host);
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

  fn configure(&mut self, ext: Box<Extension>) {
    self.extensions.push(ext);
  }

  fn start(&self) {
    for ext in self.extensions.iter() {
      ext.init(self);

      let meta = ext.meta();

      println!(
        "[Extension] Initialized {} ({})",
        meta.display_name, meta.name
      );
    }
  }

  fn emit(&self, event: Event) {
    for extension in self.extensions.iter() {
      extension.handle(&event, self)
    }
  }
}

fn main() {
  let mut engine = Engine::new("phoom.in.th");

  let editor = Editor::new();
  engine.configure(Box::new(editor));

  engine.start();
}
