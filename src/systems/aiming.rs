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

struct ReticleToUpdate {
  new_position: Vector2<f32>,
  transform: Transform
}

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
    
    // This system operates under the assumption there is a single reticle. Could use a list here if that's insufficient
    let mut reticle_to_update: Some(ReticleToUpdate) = None;
    
    for (_reticle, _transform) in (&reticles, &mut transforms).join() {
      if let Some((screen_x, screen_y)) = input.mouse_position() {
        if let Some(camera_entity) = active_camera.entity {
          let camera = cameras.get(camera_entity).unwrap();
          let camera_transform = transforms.get(camera_entity).unwrap();
          let world_position = camera.projection().screen_to_world_point(
            Point3::new(screen_x, screen_y, 0.0),
            Vector2::new(screen.width(), screen.height()),
            camera_transform,
          );

        // Mark reticle for position update
        reticle_to_update = Some(ReticleToUpdate {
          transform: _transform,
          new_position: Vector2::new(world_position.x, world_position.y)
        } _transform);
          // _transform.set_translation_xyz(world_position.x, world_position.y, 0.0);
        }
      }
    }

    // Perform our update on reticle position
    match reticles_to_update {
      Some(reticle) => {
        reticle.transform.set_translation_xyz(reticle.new_position.x, reticle.new_position.y, 0.0);

      },
      _ => {}
    };

  //   deletions.push(Deletion {
  //     hook: _entity,
  //     player: _hook.owner,
  // });
  }
}
