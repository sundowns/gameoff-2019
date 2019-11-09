extern crate amethyst;
use amethyst::{
  assets::{AssetStorage, Handle, Loader},
  core::transform::Transform,
  ecs::prelude::World,
  prelude::*,
  renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::components;
use crate::config::ArenaConfig;

#[derive(Default)]
pub struct Game {
  sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for Game {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;
    self
      .sprite_sheet_handle
      .replace(load_sprite_sheet(world, "entity_spritesheet"));

    initialise_world(world);
    initialise_player(world, self.sprite_sheet_handle.clone().unwrap());
    initialise_camera(world);
  }

  fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    // do we need this function?
    Trans::None
  }
}

fn initialise_world(world: &mut World) {
  world.insert(ArenaConfig::default());
}

fn initialise_player(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
  let (arena_height, arena_width) = {
    let config = &world.read_resource::<ArenaConfig>();
    (config.height, config.width)
  };
  let mut local_transform = Transform::default();
  local_transform.set_translation_xyz(arena_width / 2.0, arena_height / 2.0, 0.0);

  world
    .create_entity()
    .with(SpriteRender {
      sprite_sheet: sprite_sheet_handle,
      sprite_number: 1,
    })
    .with(local_transform)
    .with(components::Player)
    .with(components::Leap::default())
    .build();
}

fn initialise_camera(world: &mut World) {
  let (arena_height, arena_width) = {
    let config = &world.read_resource::<ArenaConfig>();
    (config.height, config.width)
  };
  let mut transform = Transform::default();
  transform.set_translation_xyz(arena_width / 2.0, arena_height / 2.0, 1.0);
  world
    .create_entity()
    .with(Camera::standard_2d(arena_width, arena_height))
    .with(transform)
    .build();
}

fn load_sprite_sheet(world: &mut World, file_name: &str) -> Handle<SpriteSheet> {
  // Load the sprite sheet necessary to render the graphics.
  // The texture is the pixel data
  // `texture_handle` is a cloneable reference to the texture
  let texture_handle = {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
      format!("{}.png", file_name),
      ImageFormat::default(),
      (),
      &texture_storage,
    )
  };

  let loader = world.read_resource::<Loader>();
  let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
  loader.load(
    format!("{}.ron", file_name), // Here we load the associated ron file
    SpriteSheetFormat(texture_handle),
    (),
    &sprite_sheet_store,
  )
}
