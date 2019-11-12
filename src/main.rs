use amethyst::{
  core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
  input::{InputBundle, StringBindings},
  prelude::*,
  renderer::{
    plugins::{RenderFlat2D, RenderToWindow},
    types::DefaultBackend,
    RenderingBundle,
  },
  ui::{RenderUi, UiBundle},
  utils::application_root_dir,
};
use std::time::Duration;

mod components;
mod config;
mod states;
mod systems;
use crate::states::Game;

fn main() -> amethyst::Result<()> {
  amethyst::start_logger(Default::default());

  let app_root = application_root_dir()?;

  let config_dir = app_root.join("config");
  let display_config_path = config_dir.join("display.ron");
  let key_bindings_path = config_dir.join("input.ron");
  let assets_dir = app_root.join("assets/");

  let game_data = GameDataBuilder::default()
    .with_bundle(TransformBundle::new())?
    .with_bundle(InputBundle::<StringBindings>::new().with_bindings_from_file(key_bindings_path)?)?
    .with_bundle(UiBundle::<StringBindings>::new())?
    .with(systems::LeapingSystem, "leaping_system", &["input_system"])
    .with(systems::AimingSystem, "aiming_system", &[])
    .with_bundle(
      RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
          RenderToWindow::from_config_path(display_config_path).with_clear([
            0.0,
            8.0 / 255.0,
            57.0 / 255.0,
            1.0,
          ]),
        )
        .with_plugin(RenderFlat2D::default())
        .with_plugin(RenderUi::default()),
    )?;

    // TODO: would it be possible to draw some kind of watery background - custom render passes maybe??

  let mut game = Application::build(assets_dir, Game::default())?
    .with_frame_limit(
      FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
      144,
    )
    .build(game_data)?;

  game.run();
  Ok(())
}
