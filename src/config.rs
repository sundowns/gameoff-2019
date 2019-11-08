#[derive(Debug)]
pub struct ArenaConfig {
  pub height: f32,
  pub width: f32,
}

impl Default for ArenaConfig {
  fn default() -> Self {
    ArenaConfig {
      height: 100.0,
      width: 100.0,
    }
  }
}
