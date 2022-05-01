mod player;
mod background;
mod level;
mod text_overlay;

use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin};

use background::{spawn_background, update_background, test_camera};
use level::{LevelPlugin};
use player::{PlayerPlugin};
use text_overlay::TextOverlayPlugin;

use heron::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(PhysicsPlugin::default()) // Add the plugin
        .add_startup_system(spawn_background)
        .insert_resource(Gravity::from(Vec2::new(0.0, -600.0))) // Define the gravity
        .add_plugin(PlayerPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(TextOverlayPlugin)
        //.add_system(log_collisions)
        .add_system(update_background)
        .run();
}
