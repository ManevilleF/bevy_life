use bevy::prelude::*;
use bevy_life::{CyclicColorCellState, CyclicColors2dPlugin, MooreCell2d, SimulationBatch};
use rand::Rng;

mod common;

use common::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Cyclic colors".to_string(),
            width: 1300.,
            height: 800.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(CyclicColors2dPlugin::default())
        .insert_resource(SimulationBatch::default())
        .add_startup_system(setup_camera.system())
        .add_startup_system(setup_map.system())
        .add_system(handle_reset_2d::<MooreCell2d>.system())
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

fn spawn_map(commands: &mut Commands, assets: &mut Assets<ColorMaterial>) {
    let mut rng = rand::thread_rng();
    let (size_x, size_y) = (150, 100);
    let sprite_size = 8.;
    let material = assets.add(Color::rgba(0., 0., 0., 0.).into());

    let available_states = CyclicColorCellState::available_colors();
    let state_size = available_states.len();
    let entity = commands
        .spawn()
        .insert(Transform::from_xyz(
            -(size_x as f32 * sprite_size) / 2.,
            -(size_y as f32 * sprite_size) / 2.,
            0.,
        ))
        .insert(GlobalTransform::default())
        .with_children(|builder| {
            for y in 0..=size_y {
                for x in 0..=size_x {
                    let state =
                        CyclicColorCellState(available_states[rng.gen_range(0..state_size)]);
                    builder
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite::new(Vec2::splat(sprite_size)),
                            transform: Transform::from_xyz(
                                sprite_size * x as f32,
                                sprite_size * y as f32,
                                0.,
                            ),
                            material: material.clone(),
                            ..Default::default()
                        })
                        .insert(MooreCell2d::new(IVec2::new(x, y)))
                        .insert(state);
                }
            }
        })
        .id();
    commands.insert_resource(MapEntity(entity));
    println!("map generated");
}
