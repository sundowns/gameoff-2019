pub struct Leap {
  pub is_leaping: bool,
}

impl Default for Leap {
  fn default() -> Leap {
    Leap { is_leaping: false }
  }
}
