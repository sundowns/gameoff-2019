use amethyst::core::math::Point2;

pub struct Leap {
  pub is_leaping: bool,
  pub leap_ready: bool,
  pub range: f32,
  pub target: Option<Point2<f32>>,
}

impl Default for Leap {
  fn default() -> Leap {
    Leap {
      is_leaping: false,
      leap_ready: true,
      range: 50.0,
      target: None,
    }
  }
}

impl Leap {
  pub fn new(range: f32) -> Leap {
    Leap {
      is_leaping: false,
      leap_ready: true,
      range: range,
      target: None,
    }
  }
}
