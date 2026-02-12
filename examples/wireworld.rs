use bevy::prelude::*;
use bevy_life::{MooreCell2d, SimulationBatch, WireWorld2dPlugin, WireWorldCellState};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "WireWorld".to_string(),
                resolution: [1200.0, 800.0].into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(WireWorld2dPlugin::default().with_time_step(0.1))
        .insert_resource(SimulationBatch)
        .add_systems(Startup, (setup_camera, setup_map))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn setup_map(mut commands: Commands) {
    spawn_map(&mut commands);
}

fn spawn_map(commands: &mut Commands) {
    let sprite_size = 32.;
    let (width, height) = (5, 5);
    let grid = [
        None,
        Some(WireWorldCellState::Conductor),
        Some(WireWorldCellState::ElectronTail),
        Some(WireWorldCellState::ElectronHead),
        None,
        Some(WireWorldCellState::Conductor),
        None,
        None,
        None,
        Some(WireWorldCellState::Conductor),
        Some(WireWorldCellState::Conductor),
        None,
        None,
        None,
        Some(WireWorldCellState::Conductor),
        Some(WireWorldCellState::Conductor),
        None,
        None,
        None,
        Some(WireWorldCellState::Conductor),
        None,
        Some(WireWorldCellState::Conductor),
        Some(WireWorldCellState::Conductor),
        Some(WireWorldCellState::Conductor),
        None,
    ];

    commands
        .spawn((
            Transform::from_xyz(
                -(width as f32 - 1.) * sprite_size / 2.,
                (height as f32 - 1.) * sprite_size / 2.,
                0.,
            ),
            Visibility::default(),
        ))
        .with_children(|builder| {
            for (index, state) in grid.into_iter().enumerate() {
                if let Some(state) = state {
                    let (x, y) = (index as u32 % width, index as u32 / width);

                    builder.spawn((
                        Sprite {
                            custom_size: Some(Vec2::splat(sprite_size)),
                            ..default()
                        },
                        Transform::from_xyz(sprite_size * x as f32, -sprite_size * y as f32, 0.),
                        MooreCell2d::new(IVec2::new(x.try_into().unwrap(), y.try_into().unwrap())),
                        state,
                    ));
                };
            }
        });
    println!("map generated");
}
