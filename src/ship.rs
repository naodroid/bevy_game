use bevy::{prelude::*, window::CursorMoved};


#[derive(Default)]
pub struct SpaceShip {
    pub target_x: f32,
    pub target_y: f32
}
trait ShipBuilder {
    fn build_ship(&mut self) -> &Self ;
}
impl ShipBuilder for AppBuilder {
    fn build_ship(&mut self) -> &AppBuilder {
        self.add_startup_system(startup.system())
            .add_system(update.system())
            .add_system(on_event.system())
    }
}


pub fn build(builder: &mut AppBuilder) {
    builder.add_startup_system(startup.system())
        .add_system(update.system())
        .add_system(on_event.system());
}

fn startup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: ResMut<AssetServer>,
) {
    // SpriteBundle {
    //     material: materials.add(asset_server.load("triangle.png").into()),
    //     transform: Transform::identity(),
    //     sprite: Sprite::new(Vec2::new(40.0, 80.0)),
    //     ..Default::default()
    // }
}

fn update(mut query: Query<(&SpaceShip, &mut Transform)>) {
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

fn on_event(
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
