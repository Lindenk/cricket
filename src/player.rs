use bevy::prelude::*;
use heron::prelude::*;

#[derive(Component)]
struct Player;

pub fn spawn_player(mut commands: Commands) {
  let size = Vec2::new(20., 20.);
  commands.spawn_bundle(SpriteBundle {
    sprite: Sprite {
      color: Color::YELLOW,
      custom_size: Some(size),
      ..default()
    },
    transform: Transform::from_translation(Vec3::new(0., -300., 0.)),
    ..default()
  })
    .insert(Player)
    .insert(RigidBody::KinematicVelocityBased)
    .insert(CollisionShape::Capsule {
      half_segment: size.y - size.x,
      radius: size.x
    });
}