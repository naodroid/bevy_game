use bevy::{prelude::*, window::CursorMoved};

#[derive(Default)]
struct SpaceShip {
    target_x: f32,
    target_y: f32,
}

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

fn move_ship_system(mut query: Query<(&SpaceShip, &mut Transform)>) {
    let (ship, mut transform) = query.iter_mut().next().unwrap();
    let target = Vec2 {
        x: ship.target_x,
        y: ship.target_y,
    };
    let translation = transform.translation;
    let pos = Vec2 { x: translation.x, y: translation.y };
    let diff = target - pos;
    if diff.length() > 1.0 {
        let angle = diff.angle_between(Vec2 { x: 0.0, y: 1.0 });
        transform.rotation = Quat::from_rotation_z(-angle);
        let norm = diff.normalize() / 2.0;
        transform.translation.x += norm.x;
        transform.translation.y += norm.y;
    }
}

fn cursor_detect_system(
    mut query: Query<&mut SpaceShip>,
    windows: Res<Windows>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mut cursor_moved_reader: Local<EventReader<CursorMoved>>,
) {
    let mut ship = query.iter_mut().next().unwrap();
    for event in cursor_moved_reader.iter(&cursor_moved_events) {
        let window = windows.get_primary().unwrap();
        let mx = event.position.x - window.width() / 2.0;
        let my = event.position.y - window.height() / 2.0;
        ship.target_x = mx;
        ship.target_y = my;
        break;
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup.system())
        .add_system(move_ship_system.system())
        .add_system(cursor_detect_system.system())
        .run();
}
