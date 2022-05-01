use bevy::prelude::*;
use heron::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(spawn_platforms)
      .add_startup_system(spawn_exit)
      .add_event::<VictoryEvent>()
      .add_system(on_touch_exit);
  }
}

pub struct VictoryEvent;

#[derive(Component)]
pub struct Exit;

#[derive(PhysicsLayer)]
pub enum PhysicsLayers {
  Player,
  Pickup,
  Platform,
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
      })
      .insert(CollisionLayers::none()
        .with_group(PhysicsLayers::Platform)
        .with_mask(PhysicsLayers::Player));
  }
}

pub fn spawn_exit(mut commands: Commands) {
  commands.spawn()
    .insert(Exit)
    .insert_bundle(SpriteBundle {
      sprite: Sprite {
        color: Color::BLUE,
        custom_size: Some(Vec2::new(25., 25.)),
        ..default()
      },
      transform: Transform::from_translation(Vec3::new(1800., 450., 0.)),
      ..default()
    })
    .insert(RigidBody::Sensor)
    .insert(CollisionShape::Cuboid {
      half_extends: Vec3::new(12.5, 12.5, 0.),
      border_radius: None,
    })
    .insert(CollisionLayers::none()
      .with_group(PhysicsLayers::Pickup)
      .with_mask(PhysicsLayers::Player));
}

pub fn on_touch_exit(mut commands: Commands, mut events: EventReader<CollisionEvent>, mut victory_ev: EventWriter<VictoryEvent>) {
  for ev in events.iter() {
    match ev { 
      CollisionEvent::Started(d1, d2) => {
        // println!("{:?}", ev);
        for d in [d1, d2].iter().filter(|d| d.collision_layers().contains_group(PhysicsLayers::Pickup)) 
        {
          // println!("{:?}", d);
          commands.entity(d.rigid_body_entity()).despawn();
          println!("I SHOULD SENT THE VICTORY EVENT");
          victory_ev.send(VictoryEvent);
          println!("I SHOULD HAVE SENT THE VICTORY EVENT");

        }
        //f d1.collision_layers.contains(PhysicsLayers::Pickup) || d2.collision_layers.contains(Phy)
      }
      _ => ()
    }
  }
}