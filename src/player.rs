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
      .add_system(offscreen_death)
      .add_event::<JumpEvent>()
      .add_event::<OffscreenDeathEvent>();
  }
}

pub struct JumpEvent(pub i32);
pub struct OffscreenDeathEvent();

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
    .insert(Player{movespeed: 400., jumps: 0, is_grounded: true})
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

pub fn handle_input(input: Res<Input<KeyCode>>, mut ev_jump: EventWriter<JumpEvent>, mut players: Query<(&mut Velocity, &mut Player)>) {
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
    println!("I have {} jumps", p.jumps);
    if p.jumps > 0 {
      if should_kill_x_vel {
        v.linear = Vec3::new(vel_vec.x * p.movespeed, vel_vec.y, 0.);
      } else {
        v.linear.y = vel_vec.y;
        v.linear.x += vel_vec.x * p.movespeed;
      }
      ev_jump.send(JumpEvent(3-p.jumps));
      p.jumps -= 1;
    }
  }
}

pub fn check_grounded(
  mut set: ParamSet<
    (
      &World, 
      Query<Entity, With<Player>>, 
      Query<(Entity, &mut Player)>
    )>,
  mut events: EventReader<CollisionEvent>) 
{
  let player_ids : Vec<_> = set.p1().iter().collect();
  let mut players_to_reset_jump = vec![];

  for event in events.iter() {
    match event {
      CollisionEvent::Started(d1, d2) => {
        for d in [d1, d2].iter() {
          for p_id in player_ids.iter() {
            if p_id == &d.rigid_body_entity() {
              let platform_id = if d1.rigid_body_entity() != *p_id 
              { 
                d1.rigid_body_entity()
              } else { 
                d2.rigid_body_entity() 
              };
              let player_y = set.p0().entity(*p_id).get::<Transform>().unwrap().translation.y;
              let platform_y = set.p0().entity(platform_id).get::<Transform>().unwrap().translation.y;

              if platform_y < player_y {
                players_to_reset_jump.push(*p_id);
              }
            }
          }
        }
      }

      // TODO: This only works if the player is contacting exactly 1 platform. Fix this.
      CollisionEvent::Stopped(d1, d2) => {
        /*for d in [d1, d2].iter() {
          for (p_id, mut p_data) in players.iter_mut() {
            if p_id == d.rigid_body_entity() {
              p_data.is_grounded = false;
            }
          }
        }*/
      }
    }
  }

  for (p_id, mut data) in set.p2().iter_mut() {
    if players_to_reset_jump.contains(&p_id) {
      data.is_grounded = true;
      data.jumps = 3;
      println!("Reset jumps");
    }
  }
}

pub fn camera_follow(mut set: ParamSet<(Query<&mut Transform, With<Camera>>, Query<&Transform, With<Player>>)>) {
  let mut player_trans = set.p1().single().translation.clone();
  let mut camera_trans = set.p0();
  let mut camera_trans = camera_trans.single_mut();

  player_trans.y = player_trans.y.max(-50.);
  camera_trans.translation = player_trans;
}

pub fn offscreen_death(mut players: Query<&mut Transform, With<Player>>, mut offscreen_event: EventWriter<OffscreenDeathEvent>) {
  for mut player in players.iter_mut() {
    if player.translation.y < -500. {
      player.translation = Vec3::new(0., 100., 0.);
      offscreen_event.send(OffscreenDeathEvent());
    }
  }
}
