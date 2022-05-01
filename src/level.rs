use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
  fn build(&self, app: &mut App) {
    app;
  }
}

#[derive(Bundle)]
pub struct Exit {
  #[bundle]
  sprite_bundle: SpriteBundle
}

//pub fn spawn_exit()