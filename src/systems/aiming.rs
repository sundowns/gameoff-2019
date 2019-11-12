use amethyst::{
  core::{
    math::{Point3, Vector2},
    Transform,
  },
  ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
  input::{InputHandler, StringBindings},
  renderer::{ActiveCamera, Camera},
  window::ScreenDimensions,
};

use crate::components::Reticle;

pub struct AimingSystem;

impl<'s> System<'s> for AimingSystem {
  type SystemData = (
    ReadStorage<'s, Reticle>,
    WriteStorage<'s, Transform>,
    ReadStorage<'s, Camera>,
    Read<'s, ActiveCamera>,
    ReadExpect<'s, ScreenDimensions>,
    Read<'s, InputHandler<StringBindings>>,
  );

  fn run(
    &mut self,
    (reticles, mut transforms, cameras, active_camera, screen, input): Self::SystemData,
  ) {
      // TODO: Implement this aiming system:
      // - get mouse position - done
      // - get world coords of mouse - done
      // - draw reticle at that position - wip
      // - Can we colour the reticle based on player/game state maybe?? - not started

      println!("We're aiming!! " ); // TODO: delete debug line
    for (_reticle, _transform) in (&reticles, &mut transforms).join() {
      if let Some((screen_x, screen_y)) = input.mouse_position() {
        if let Some(camera_entity) = active_camera.entity {
          let camera = cameras.get(camera_entity).unwrap();
          let camera_transform = transforms.get_mut(camera_entity).unwrap();
          let world_position = camera.projection().screen_to_world_point(
            Point3::new(screen_x, screen_y, 0.0),
            Vector2::new(screen.width(), screen.height()),
            camera_transform,
          );

          // Move reticle to our mouse position
          _transform.set_translation_xyz(world_position.x, world_position.y, 0.0);
        }
      }


    }
  }
}
