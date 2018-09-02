use std::fmt;

struct RGBA(u32, u32, u32, f32);
struct HSLA(u32, u32, u32, f32);

impl fmt::Display for RGBA {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "RGBA({}, {}, {}, {})", self.0, self.1, self.2, self.3)
  }
}

impl fmt::Display for HSLA {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "HSLA({}, {}, {}, {})", self.0, self.1, self.2, self.3)
  }
}

impl RGBA {
  fn to_hsla(&self) -> HSLA {
    let (r, g, b, a) = (self.0, self.1, self.2, self.3);

    return HSLA(r, g, b, a);
  }
}

macro_rules! rgba_color {
  ($r:expr, $g:expr, $b:expr, $a:expr) => {
    RGBA($r, $g, $b, $a)
  };
}

fn main() {
  let color = RGBA(255, 255, 255, 0.5);

  println!("{}", color);
  println!("{}", color.to_hsla());
  println!("{}", rgba_color!(255, 255, 255, 0.5))
}
