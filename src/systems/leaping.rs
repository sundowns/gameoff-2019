use amethyst::{
  core::{math::Vector2, Transform},
  ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

use crate::components::Leap;
use crate::components::Player;

pub struct LeapingSystem;

impl<'s> System<'s> for LeapingSystem {
  type SystemData = (
    ReadStorage<'s, Player>,
    WriteStorage<'s, Leap>,
    WriteStorage<'s, Transform>,
  );
  fn run(&mut self, (players, mut leaps, mut transforms): Self::SystemData) {
    for (_player, _leap, _transform) in (&players, &mut leaps, &mut transforms).join() {
      match _leap.target {
        Some(target) => {
          if _leap.is_leaping {
            println!("I'm leaping!");

            let directional = Vector2::new(
              target.x - _transform.translation().x,
              target.y - _transform.translation().y,
            );

            // _transform

            //TODO: this update loop
            // - check if colliding with target
            //  -- true -> spawn lily pad, set is_leaping=false, set leaping target, start leap cooldown
            //  -- false -> move towards target (normalize directional & move a small amount each frame (some function of sine later to simulate 3D jump?))
          }
        }
        None => {}
      };
    }
  }
}
