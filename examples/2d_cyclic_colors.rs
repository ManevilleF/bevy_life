use bevy::prelude::*;
use bevy_life::{CyclicColorCellState, CyclicColors2dPlugin, MooreCell2d, SimulationBatch};
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Cyclic Colors".to_string(),
                width: 1200.,
                height: 800.,
                ..Default::default()
            },
            ..default()
        }))
        .add_plugin(CyclicColors2dPlugin::default())
        .insert_resource(SimulationBatch::default())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_map)
        .run();
}

fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}

fn setup_map(mut commands: Commands) {
    spawn_map(&mut commands);
}

fn spawn_map(commands: &mut Commands) {
    let mut rng = rand::thread_rng();
    let (size_x, size_y) = (300, 200);
    let sprite_size = 4.;

    let available_states = CyclicColorCellState::available_colors();
    let state_size = available_states.len();
    commands
        .spawn(SpatialBundle::from_transform(Transform::from_xyz(
            -(size_x as f32 * sprite_size) / 2.,
            -(size_y as f32 * sprite_size) / 2.,
            0.,
        )))
        .with_children(|builder| {
            for y in 0..=size_y {
                for x in 0..=size_x {
                    let color = available_states[rng.gen_range(0..state_size)];
                    let state = CyclicColorCellState(color);
                    builder.spawn((
                        SpriteBundle {
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
                        },
                        MooreCell2d::new(IVec2::new(x, y)),
                        state,
                    ));
                }
            }
        });
    println!("map generated");
}
