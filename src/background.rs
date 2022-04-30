use bevy::prelude::*;

#[derive(Component)]
pub struct BackgroundLayer;

pub fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>) {
  let window = windows.get_primary_mut().unwrap();
  println!("Window size was: {},{}", window.width(), window.height());
  window.set_resolution(1920., 1080.);
  let bg_1 = asset_server.load("sprites/backgrounds/BG1_Sky_1920_1080.png");
  let bg_2 = asset_server.load("sprites/backgrounds/BG2_Mountains_6000_1000.png");
  let bg_3 = asset_server.load("sprites/backgrounds/BG3_FrontMountains_4000_300.png");
  let bg_4 = asset_server.load("sprites/backgrounds/BG4_Trees_2500_200.png");

  let bg_1_size = Vec2::new(1920., 1080.);
  let bg_2_size = Vec2::new(3000., 500.);
  let bg_3_size = Vec2::new(2000., 300.);
  let bg_4_size = Vec2::new(2000., 200.);

  // sprite, size, y, z away from 1000 (you're at 1000)
  let bg_layers = [
    (bg_1, bg_1_size, 0., -990.),
    (bg_2, bg_2_size, -200., -400.),
    (bg_3, bg_3_size, -250., -200.),
    (bg_4, bg_4_size, -350., -100.)
  ];

  let mut layer_number = 1.;
  for layer in bg_layers {
    commands
    .spawn_bundle(SpriteBundle {
      sprite :Sprite {
        custom_size:Some(layer.1),
        ..default()
      },
      texture: layer.0,
      transform: Transform::from_translation(Vec3::new(0., layer.2, (layer.3))),
      ..default()
    })
    .insert(BackgroundLayer);
    layer_number = layer_number + 1.;
  }
}

pub fn update_background(mut set: ParamSet<(Query<& Transform, With<Camera>>, Query<&mut Transform, With<BackgroundLayer>>)>) {
  let mut camera = set.p0();
  let mut camera = camera.single_mut();
  let camera_x = camera.translation.x;

  
  for mut layer in set.p1().iter_mut() {
    //println!("Layer: {:?}", layer);
    layer.translation.x = (camera_x * -(layer.translation.z/999.));
  }
}

pub fn test_camera(time: Res<Time>, mut camera: Query<&mut Transform, With<Camera>>) {
  for mut c in camera.iter_mut() {
    //println!("Camera!!!!!!: {:?}", c);
    c.translation.x = (time.seconds_since_startup().sin() * 100.) as f32;
  }
}
