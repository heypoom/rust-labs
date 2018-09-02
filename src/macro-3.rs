use std::collections::HashMap;

macro_rules! map {
  ($( $key:expr => $value:expr ),*) => {{
    let mut hm = HashMap::new();
    $( hm.insert($key, $value); )*
    hm
  }};
}

macro_rules! hello {
  () => {
    println!("Hello!");
  };

  ($name:expr) => {
    println!("Hello, {}!", $name);
  };
}

macro_rules! pipe {
  () => {
    println!("You need something to pipe...");
  };

  ( $val:ident $(|> $fn:ident)* ) => {
    println!("=>> Value {:?}", $val);

    let mut x = $val;

    $(
      x = $fn(x);
      println!("=> {:?}", x);
    )*
  };
}

fn main() {
  let user = map!(
    "name" => "Finn",
    "gender" => "Boy"
  );

  println!("User {:?}", user);

  hello!();
  hello!("Phoom");

  let x = 5;
  let plus_three = |x| x + 3;
  let mul_two = |x| x * 2;

  pipe!();
  pipe!(x |> plus_three |> mul_two);
}
