use amethyst::ecs::prelude::{Component, DenseVecStorage, NullStorage};

mod leap;
mod player;
mod reticle;

pub use self::{leap::Leap, player::Player, reticle::Reticle};

impl Component for Player {
  type Storage = NullStorage<Self>; // TODO: is this the right storage for an empty component?
}

impl Component for Leap {
  type Storage = DenseVecStorage<Self>;
}

impl Component for Reticle {
  type Storage = NullStorage<Self>; // TODO: again not sure on storage
}
