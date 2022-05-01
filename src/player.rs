use bevy::prelude::*;
use heron::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(spawn_player)
      .add_system(handle_input)
      .add_system(check_grounded);
  }
}

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
    transform: Transform::from_translation(Vec3::new(0., 100., 0.)),
    ..default()
  })
    .insert(Player{movespeed: 300., is_grounded: false})
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
  } else if input.just_pressed(KeyCode::Space) {
    vel_vec.y += 500.;
  }

  for (mut v, p) in players.iter_mut() {
    if p.is_grounded {
      v.linear += Vec3::new(vel_vec.x * p.movespeed, vel_vec.y, 0.);
    }
  }
}

pub fn check_grounded(mut players: Query<(Entity, &mut Player)>, mut events: EventReader<CollisionEvent>) {
  //println!("got here");
  for event in events.iter() {
    match event {
      CollisionEvent::Started(d1, d2) => {
        for d in [d1, d2].iter() {
          for (p_id, mut p_data) in players.iter_mut() {
            if p_id == d.rigid_body_entity() && d.normals()[0].y < 0.{
              p_data.is_grounded = true;
            }
          }
        }
      }
      CollisionEvent::Stopped(d1, d2) => {
        for d in [d1, d2].iter() {
          for (p_id, mut p_data) in players.iter_mut() {
            if p_id == d.rigid_body_entity() {
              p_data.is_grounded = false;
            }
          }
        }
      }
    }
  }
}
