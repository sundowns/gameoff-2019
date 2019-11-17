use amethyst::{
  core::{
    math::{Point2, Point3, Vector2, Vector3},
    Transform,
  },
  ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
  input::{InputHandler, StringBindings},
  renderer::{ActiveCamera, Camera},
  window::ScreenDimensions,
};

use crate::components::Leap;
use crate::components::Player;

pub struct StartLeapingSystem;

impl<'s> System<'s> for StartLeapingSystem {
  type SystemData = (
    ReadStorage<'s, Player>,
    Read<'s, InputHandler<StringBindings>>,
    WriteStorage<'s, Leap>,
    ReadStorage<'s, Transform>,
    ReadStorage<'s, Camera>,
    Read<'s, ActiveCamera>,
    ReadExpect<'s, ScreenDimensions>,
  );

  fn run(
    &mut self,
    (players, input, mut leaps, transforms, cameras, active_camera, screen): Self::SystemData,
  ) {
    for (_player, _leap, _transform) in (&players, &mut leaps, &transforms).join() {
      if _leap.leap_ready {
        if let Some((screen_x, screen_y)) = input.mouse_position() {
          if let Some(camera_entity) = active_camera.entity {
            let camera = cameras.get(camera_entity).unwrap();
            let transform = transforms.get(camera_entity).unwrap();
            let world_position = camera.projection().screen_to_world_point(
              Point3::new(screen_x, screen_y, 0.0),
              Vector2::new(screen.width(), screen.height()),
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
                  _leap.is_leaping = true;
                  let target = Point2::new(world_position.x, world_position.y);
                  _leap.target = Some(target);

                  let directional = Vector3::new(
                    target.x - _transform.translation().x,
                    target.y - _transform.translation().y,
                    0.0,
                  );

                  _leap.velocity = directional / directional.magnitude() * 100.0; // TODO: put speed var somewhere
                  println!("leapead towards {},{}", world_position.x, world_position.y);
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
