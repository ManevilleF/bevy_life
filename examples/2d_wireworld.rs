use bevy::prelude::*;
use bevy_life::cell::Cell2d;
use bevy_life::{
    NewCell2d, NewWireWorldCellState, WireWorldCellState, WireWorldGameOfLife2dPlugin,
};

struct MapEntity(pub Entity);

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(WireWorldGameOfLife2dPlugin::default())
        .insert_resource(WindowDescriptor {
            title: "Game Of Life".to_string(),
            width: 1000.,
            height: 1000.,
            ..Default::default()
        })
        .add_startup_system(setup_camera.system())
        .add_startup_system(setup_map.system())
        .add_system(handle_reset.system())
        .add_system(handle_mouse_input.system())
        .run();
}

fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_map(mut commands: Commands, mut assets: ResMut<Assets<ColorMaterial>>) {
    // map
    spawn_map(&mut commands, &mut assets);
}

fn mouse_coords(window: &Window, position: Vec2) -> Vec2 {
    let window_size = Vec2::new(window.width(), window.height());
    position - window_size / 2.
}

fn handle_mouse_input(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    mut query: Query<(&Cell2d, &mut WireWorldCellState)>,
    windows: Res<Windows>,
    map: Res<MapEntity>,
) {
    let sprite_size = 20;
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }
    let window = windows.get_primary().unwrap();
    let mouse_position = match window.cursor_position() {
        None => return,
        Some(p) => mouse_coords(window, p),
    };
    let position = IVec2::new(
        (mouse_position.x / sprite_size as f32) as i32,
        (mouse_position.y / sprite_size as f32) as i32,
    );
    let mut found_cell_state = None;
    for (cell, state) in query.iter_mut() {
        if cell.coords == position {
            found_cell_state = Some(state);
            break;
        }
    }
    if let Some(mut state) = found_cell_state {
        println!(
            "Cell at {:?} ({:?}) becomes an electron head",
            position, mouse_position
        );
        *state = WireWorldCellState::ElectronHead;
    } else {
        println!(
            "Spawning new conductor at {:?} ({:?}) ",
            position, mouse_position
        );
        commands.entity(map.0).with_children(|builder| {
            builder
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite::new(Vec2::splat((sprite_size - 1) as f32)),
                    transform: Transform::from_xyz(
                        (sprite_size * position.x) as f32,
                        (sprite_size * position.y) as f32,
                        0.,
                    ),
                    ..Default::default()
                })
                .insert(NewCell2d::new(position))
                .insert(WireWorldCellState::Conductor);
        });
    }
}

fn handle_reset(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    map: Res<MapEntity>,
    mut assets: ResMut<Assets<ColorMaterial>>,
) {
    if keys.just_released(KeyCode::Space) {
        commands.entity(map.0).despawn_recursive();
        commands.remove_resource::<MapEntity>();
        println!("regenerating map");
        spawn_map(&mut commands, &mut assets);
    }
}

fn spawn_map(commands: &mut Commands, assets: &mut Assets<ColorMaterial>) {
    let map_size = 5;
    let sprite_size = 20.;
    let material = assets.add(Color::rgba(0., 0., 0., 0.).into());

    let entity = commands
        .spawn()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .with_children(|builder| {
            for y in -map_size..=map_size {
                for x in -map_size..=map_size {
                    if (x > -map_size && x < map_size && y > -map_size && y < map_size)
                        || ((x == -map_size || x == map_size) && (y == -map_size || y == map_size))
                    {
                        continue;
                    }
                    let state = NewWireWorldCellState::new(if x == 0 && y == -map_size {
                        WireWorldCellState::ElectronTail
                    } else if x == 1 && y == -map_size {
                        WireWorldCellState::ElectronHead
                    } else {
                        WireWorldCellState::Conductor
                    });
                    builder
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite::new(Vec2::splat(sprite_size - 1.)),
                            transform: Transform::from_xyz(
                                sprite_size * x as f32,
                                sprite_size * y as f32,
                                0.,
                            ),
                            material: material.clone(),
                            ..Default::default()
                        })
                        .insert(NewCell2d::new(IVec2::new(x, y)))
                        .insert(state);
                }
            }
        })
        .id();
    commands.insert_resource(MapEntity(entity));
    println!("map generated");
}
