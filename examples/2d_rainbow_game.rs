use bevy::prelude::*;
use bevy_life::{MooreCell2d, RainbowCellState, RainbowGame2dPlugin, SimulationBatch};
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rainbow Game".to_string(),
                resolution: [1200.0, 800.0].into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RainbowGame2dPlugin::default())
        .insert_resource(SimulationBatch)
        .add_systems(Startup, (setup_camera, setup_map))
        .run();
}

fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d::default());
}

fn setup_map(mut commands: Commands) {
    spawn_map(&mut commands);
}

fn spawn_map(commands: &mut Commands) {
    let mut rng = rand::thread_rng();
    let (size_x, size_y) = (600, 400);
    let sprite_size = 2.;
    let color = Color::srgba(1., 0., 0., 1.);

    commands
        .spawn((
            Transform::from_xyz(
                -(size_x as f32 * sprite_size) / 2.,
                -(size_y as f32 * sprite_size) / 2.,
                0.,
            ),
            Visibility::default(),
        ))
        .with_children(|builder| {
            for y in 0..=size_y {
                for x in 0..=size_x {
                    let state = if rng.gen_bool(1. / 3.) {
                        RainbowCellState::Alive(if rng.gen_bool(1. / 2.) { 0. } else { 1. })
                    } else {
                        RainbowCellState::Dead
                    };
                    builder.spawn((
                        Sprite {
                            custom_size: Some(Vec2::splat(sprite_size)),
                            color,
                            ..Default::default()
                        },
                        Transform::from_xyz(sprite_size * x as f32, sprite_size * y as f32, 0.),
                        MooreCell2d::new(IVec2::new(x, y)),
                        state,
                    ));
                }
            }
        });
    println!("map generated");
}
