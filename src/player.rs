use bevy::prelude::*;
use heron::prelude::*;

#[derive(Component)]
pub struct Player {
  pub movespeed: f32,
  pub is_grounded: bool
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
  let cricket_sprite = asset_server.load("sprites/scary_cricket_70_200.png");

  let size = Vec2::new(70., 200.) / 2.;
  commands.spawn_bundle(SpriteBundle {
    texture: cricket_sprite,
    sprite: Sprite {
      custom_size: Some(size),
      ..default()
    },
    transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
    ..default()
  })
    .insert(Player{movespeed: 200., is_grounded: false})
    .insert(RigidBody::Dynamic)
    .insert(Velocity::from(Vec2::new(0., 0.)))
    .insert(CollisionShape::Capsule {
      half_segment: (size.y - size.x) / 2.,
      radius: size.x / 2.
    })
    .insert(PhysicMaterial {
      ..default()
    })
    .insert(RotationConstraints::lock());
}

pub fn handle_input(input: Res<Input<KeyCode>>, mut players: Query<(&mut Velocity, &Player)>) {
  let mut vel_vec = Vec2::default();

  // handle left/right movement. Add a hop to each
  if input.just_pressed(KeyCode::Right) {
    vel_vec.x += 1.;
    vel_vec.y += 120.;
  } else if input.just_pressed(KeyCode::Left) {
    vel_vec.x -= 1.;
    vel_vec.y += 120.;
  }

  for (mut v, p) in players.iter_mut() {
    if p.is_grounded {
      v.linear += Vec3::new(vel_vec.x * p.movespeed, vel_vec.y, 0.);
    }
  }
}
