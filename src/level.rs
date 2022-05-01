use bevy::prelude::*;
use heron::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
  fn build(&self, app: &mut App) {
    app.add_startup_system(spawn_platforms);
  }
}

#[derive(Bundle)]
pub struct Exit {
  #[bundle]
  sprite_bundle: SpriteBundle,
}

fn spawn_platforms(mut commands: Commands) {
  let platforms = [
    (Vec2::new(1000.0, 20.0), Vec3::new(0.0, -200.0, 0.0)),  
    (Vec2::new(1000.0, 20.0), Vec3::new(500.0, 00.0, 0.0)),
    (Vec2::new(1000.0, 20.0), Vec3::new(1000.0, 200.0, 0.0)),
    (Vec2::new(1000.0, 20.0), Vec3::new(1500.0, 400.0, 0.0)),

  ];

  let mut camera = OrthographicCameraBundle::new_2d();
  camera.transform.translation.z = 1.;
  commands.spawn_bundle(camera);

  for (size, position) in platforms {
    // The ground
    commands
      // Spawn a bundle that contains at least a `GlobalTransform`
      .spawn_bundle(SpriteBundle {
        sprite: Sprite {
          color: Color::GREEN,
          custom_size: Some(size),
          ..Default::default()
        },
        transform: Transform::from_translation(position),
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
  }
}

//pub fn spawn_exit()
