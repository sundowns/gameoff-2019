use amethyst::{
  core::{
    math::{Point3, Vector2},
    Transform,
  },
  ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
  input::{InputHandler, StringBindings},
  renderer::{ActiveCamera, Camera},
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
    ReadStorage<'s, Camera>,
    Read<'s, ActiveCamera>,
  );

  fn run(
    &mut self,
    (mut players, input, mut leaps, transforms, cameras, active_camera): Self::SystemData,
  ) {
    for (_player, _leap, _transform) in (&mut players, &mut leaps, &transforms).join() {
      if _leap.leap_ready {
        if let Some((screen_x, screen_y)) = input.mouse_position() {
          if let Some(camera_entity) = active_camera.entity {
            let camera = cameras.get(camera_entity).unwrap();
            let transform = transforms.get(camera_entity).unwrap();
            let world_position = camera.projection().screen_to_world_point(
              Point3::new(screen_x, screen_y, 0.0),
              Vector2::new(500.0, 500.0), // TODO: get screen res
              transform,
            );

            let attempted_leap_distance = Vector2::new(
              world_position.x - _transform.translation().x,
              world_position.y - _transform.translation().y,
            )
            .magnitude();

            match input.action_is_down("leap") {
              Some(_is_down) => {
                println!("magnitude {}", attempted_leap_distance);
                if _is_down && attempted_leap_distance <= _leap.range {
                  _leap.leap_ready = false;

                  // TODO: Apply a force?? idk
                  println!("leaped towards {},{}", world_position.x, world_position.y);
                }
              }
              _ => {}
            }
          }
        }
      }
    }
  }
}
