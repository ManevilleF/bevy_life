use bevy::prelude::*;
use bevy_life::{ConwayCellState, GameOfLife2dPlugin, MooreCell2d, SimulationBatch};
use rand::Rng;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Game Of Life".to_string(),
            width: 1300.,
            height: 800.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GameOfLife2dPlugin::default())
        .insert_resource(SimulationBatch::default())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_map)
        .run();
}

fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn_bundle(Camera2dBundle::default());
}

fn setup_map(mut commands: Commands) {
    spawn_map(&mut commands);
}

fn spawn_map(commands: &mut Commands) {
    let mut rng = rand::thread_rng();
    let (size_x, size_y) = (300, 200);
    let sprite_size = 4.;
    let color = Color::rgba(0., 0., 0., 0.);

    commands
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
                    let state = ConwayCellState(rng.gen_bool(1. / 3.));
                    builder
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(sprite_size)),
                                color,
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(
                                sprite_size * x as f32,
                                sprite_size * y as f32,
                                0.,
                            ),
                            ..Default::default()
                        })
                        .insert(MooreCell2d::new(IVec2::new(x, y)))
                        .insert(state);
                }
            }
        });
    println!("map generated");
}
