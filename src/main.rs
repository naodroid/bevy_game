mod ship;
use ship::{SpaceShip};

use bevy::{prelude::*, window::CursorMoved};

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: ResMut<AssetServer>,
) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle {
            material: materials.add(asset_server.load("triangle.png").into()),
            transform: Transform::identity(),
            sprite: Sprite::new(Vec2::new(40.0, 80.0)),
            ..Default::default()
        })
        .with(SpaceShip::default());
}


fn main() {
    let mut builder = App::build();
    builder.add_plugins(DefaultPlugins)
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup.system());
    ship::build(&mut builder);
    builder.run();
}
