use amethyst::{
  core::{math::Vector3, timing::Time, Transform},
  ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::Leap;
use crate::components::Player;

pub struct LeapingSystem;

impl<'s> System<'s> for LeapingSystem {
  type SystemData = (
    ReadStorage<'s, Player>,
    WriteStorage<'s, Leap>,
    WriteStorage<'s, Transform>,
    Read<'s, Time>,
  );
  fn run(&mut self, (players, mut leaps, mut transforms, time): Self::SystemData) {
    for (_player, _leap, _transform) in (&players, &mut leaps, &mut transforms).join() {
      match _leap.target {
        Some(_target) => {
          if _leap.is_leaping {
            //TODO: this update loop
            // - check if colliding with target
            //  -- true -> spawn lily pad, set is_leaping=false, set leaping target, start leap cooldown
            //  -- false -> move towards target
            _transform.append_translation(_leap.velocity * time.delta_seconds());
          }
        }
        None => {}
      };
    }
  }
}
