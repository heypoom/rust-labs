use std::collections::HashMap;

macro_rules! map {
  ($( $key:expr => $value:expr ),*) => {{
    let mut hm = HashMap::new();
    $( hm.insert($key, $value); )*
    hm
  }};
}

fn main() {
  let team = map!("dan" => "react", "evan" => "vue");
  println!("Team {:?}", team);
}
