use amethyst::{
  core::Transform,
  ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
  input::{InputHandler, StringBindings},
};

use crate::components::Leap;
use crate::components::Player;

pub struct LeapingSystem;

impl<'s> System<'s> for LeapingSystem {
  type SystemData = (
    WriteStorage<'s, Player>,
    Read<'s, InputHandler<StringBindings>>,
    WriteStorage<'s, Leap>,
    ReadStorage<'s, Transform>,
  );

  fn run(&mut self, (mut players, input, mut leaps, transforms): Self::SystemData) {
    for (_player, _leap, _transform) in (&mut players, &mut leaps, &transforms).join() {
      match input.action_is_down("leap") {
        Some(_is_down) => {
          if _is_down {
            let pos = _transform.translation();
            println!("I leaped! {}, {}", pos.x, pos.y);
          }
        }
        _ => {}
      }
    }
  }
}
