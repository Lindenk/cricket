use bevy::prelude::*;
use heron::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(spawn_player)
      .add_system(handle_input)
      .add_system(check_grounded)
      .add_system(camera_follow)
      .add_system(offscreen_death);
  }
}

#[derive(Component)]
pub struct Player {
  pub movespeed: f32,
  pub jumps: i32,
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
    .insert(Player{movespeed: 400., jumps: 2, is_grounded: true})
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

pub fn handle_input(input: Res<Input<KeyCode>>, mut players: Query<(&mut Velocity, &mut Player)>) {
  let mut vel_vec = Vec2::default();

  // handle left/right movement. Add a hop to each
  let mut should_kill_x_vel = true;
  if input.just_pressed(KeyCode::Right) {
    vel_vec.x += 1.;
    vel_vec.y += 180.;
  } else if input.just_pressed(KeyCode::Left) {
    vel_vec.x -= 1.;
    vel_vec.y += 180.;
  } else if input.just_pressed(KeyCode::Space) {
    vel_vec.y += 500.;
    should_kill_x_vel = false;
  } else {
    return;
  }

  for (mut v, mut p) in players.iter_mut() {
    if p.jumps > 0 || p.is_grounded {
      if should_kill_x_vel {
        v.linear = Vec3::new(vel_vec.x * p.movespeed, vel_vec.y, 0.);
      } else {
        v.linear.y = vel_vec.y;
        v.linear.x += vel_vec.x * p.movespeed;
      }
      p.jumps -= 1;
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
            if p_id == d.rigid_body_entity() /*&& d.normals()[0].y < 0.*/{
              p_data.is_grounded = true;
              p_data.jumps = 3;
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

pub fn camera_follow(mut set: ParamSet<(Query<&mut Transform, With<Camera>>, Query<&Transform, With<Player>>)>) {
  let mut player_trans = set.p1().single().translation.clone();
  let mut camera_trans = set.p0();
  let mut camera_trans = camera_trans.single_mut();

  player_trans.y = player_trans.y.max(-100.);
  camera_trans.translation = player_trans;
}

pub fn offscreen_death(mut players: Query<&mut Transform, With<Player>>) {
  for mut player in players.iter_mut() {
    if player.translation.y < -500. {
      println!("test");
      player.translation = Vec3::new(0., 100., 0.);
    }
  }
}
