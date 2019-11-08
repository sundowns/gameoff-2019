use amethyst::ecs::prelude::{Component, DenseVecStorage, NullStorage};

mod leap;
mod player;

pub use self::{leap::Leap, player::Player};

impl Component for Player {
  type Storage = NullStorage<Self>; // TODO: is this the right storage for an empty component?
}

impl Component for Leap {
  type Storage = DenseVecStorage<Self>;
}
