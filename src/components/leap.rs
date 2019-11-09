pub struct Leap {
  pub is_leaping: bool,
  pub leap_ready: bool,
}

impl Default for Leap {
  fn default() -> Leap {
    Leap {
      is_leaping: false,
      leap_ready: true,
    }
  }
}
