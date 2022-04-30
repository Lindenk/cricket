use bevy::prelude::*;
use heron::prelude::*;

#[derive(Component)]
struct Player;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
  let cricket_sprite = asset_server.load("sprites/scary_cricket_70_200.png");

  let size = Vec2::new(70., 200.);
  commands.spawn_bundle(SpriteBundle {
    texture: cricket_sprite,
    transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
    ..default()
  })
    .insert(Player)
    .insert(RigidBody::Dynamic)
    .insert(Velocity::from(Vec2::new(0., 1000.)))
    .insert(CollisionShape::Capsule {
      half_segment: (size.y - size.x) / 2.,
      radius: size.x / 2.
    })
    .insert(PhysicMaterial {
      ..default()
    });
}