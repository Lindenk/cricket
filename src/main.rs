mod player;
mod background;

use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin};

use background::{spawn_background, update_background, test_camera};
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

fn spawn(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform.translation.z = 1.;
    commands.spawn_bundle(camera);

    // The ground
    let size = Vec2::new(1000.0, 20.0);
    commands
        // Spawn a bundle that contains at least a `GlobalTransform`
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(size),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -200.0, 0.0)),
            ..Default::default()
        })
        // Make it a rigid body
        .insert(RigidBody::Static)
        // Attach a collision shape
        .insert(CollisionShape::Cuboid {
            half_extends: size.extend(0.0) / 2.0,
            border_radius: None,
        })
        // Define restitution (so that it bounces)
        .insert(PhysicMaterial {
            restitution: 0.,
            friction: 1.0,
            ..Default::default()
        });

    commands
        // Spawn a bundle that contains at least a `GlobalTransform`
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(size),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
            ..Default::default()
        })
        // Make it a rigid body
        .insert(RigidBody::Static)
        // Attach a collision shape
        .insert(CollisionShape::Cuboid {
            half_extends: size.extend(0.0) / 2.0,
            border_radius: None,
        })
        // Define restitution (so that it bounces)
        .insert(PhysicMaterial {
            restitution: 1.0,
            ..Default::default()
        });
    // The Ball
    //let size = Vec2::new(30.0, 30.0);
    /*commands
        // Spawn a bundle that contains at least a `GlobalTransform`
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(size),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(-400.0, 0.0, 0.0)),
            ..Default::default()
        })
        // Make it a rigid body
        .insert(RigidBody::Dynamic)
        // Attach a collision shape
        .insert(CollisionShape::Cuboid {
            half_extends: size.extend(0.0) / 2.0,
            border_radius: None,
        })
        // Add an initial velocity. (it is also possible to read/mutate this component later)
        .insert(Velocity::from(Vec2::X * 200.0).with_angular(AxisAngle::new(Vec3::Z, -PI)))
        // Define restitution (so that it bounces)
        .insert(PhysicMaterial {
            restitution: 1.2,
            ..Default::default()
        });*/
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
