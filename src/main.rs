mod player;
mod background;
mod level;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin};

use background::{spawn_background, update_background, test_camera};
use level::{spawn};
use player::{PlayerPlugin};

use heron::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(PhysicsPlugin::default()) // Add the plugin
        .insert_resource(Gravity::from(Vec2::new(0.0, -600.0))) // Define the gravity
        .add_plugin(PlayerPlugin)
        .add_startup_system(spawn)
        .add_startup_system(spawn_background)
        //.add_system(log_collisions)
        .add_system(update_background)
        //.add_system(test_camera)
        .run();
}


fn log_collisions(mut events: EventReader<CollisionEvent>, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(d1, d2) => {
                println!("Collision started between {:?} and {:?}", d1, d2);
                audio.play(asset_server.load("sounds/Sango_Snap_4.wav"));
		()
            }
            CollisionEvent::Stopped(d1, d2) => {
                println!("Collision stopped between {:?} and {:?}", d1, d2)
            }
        }
    }
}
