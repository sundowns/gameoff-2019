pub struct Leap {
  pub is_leaping: bool,
  pub leap_ready: bool,
  pub range: f32,
}

impl Default for Leap {
  fn default() -> Leap {
    Leap {
      is_leaping: false,
      leap_ready: true,
      range: 50.0,
    }
  }
}

impl Leap {
  pub fn new(range: f32) -> Leap {
    Leap {
      is_leaping: false,
      leap_ready: true,
      range: range,
    }
  }
}
