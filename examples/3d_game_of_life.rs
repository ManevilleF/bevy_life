use bevy::prelude::*;
use bevy_life::{ConwayCell4555State, GameOfLife3dPlugin, MooreCell3d, SimulationBatch};
use rand::Rng;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            brightness: 1.0,
            ..default()
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "3D Game Of Life".to_string(),
                resolution: [1200.0, 800.0].into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GameOfLife3dPlugin::default())
        .insert_resource(SimulationBatch)
        .add_systems(Startup, (setup_camera, setup_map))
        .add_systems(Update, color)
        .run();
}

fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(100., 100., -150.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn setup_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cuboid::new(1., 1., 1.));
    let material = materials.add(Color::WHITE);
    // map
    spawn_map(&mut commands, mesh, material);
}

fn spawn_map(commands: &mut Commands, mesh: Handle<Mesh>, material: Handle<StandardMaterial>) {
    let mut rng = rand::thread_rng();
    let map_size = 80;
    commands
        .spawn(SpatialBundle::from_transform(Transform::from_xyz(
            -(map_size as f32) / 2.,
            -(map_size as f32) / 2.,
            -(map_size as f32) / 2.,
        )))
        .with_children(|builder| {
            for z in 0..=map_size {
                for y in 0..=map_size {
                    for x in 0..=map_size {
                        let state = ConwayCell4555State(rng.gen_bool(1. / 3.));
                        builder.spawn((
                            PbrBundle {
                                mesh: mesh.clone(),
                                transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                                material: material.clone(),
                                ..default()
                            },
                            MooreCell3d::new(IVec3::new(x, y, z)),
                            state,
                        ));
                    }
                }
            }
        });
    println!("map generated");
}

pub fn color(
    mut query: Query<(&ConwayCell4555State, &mut Visibility), Changed<ConwayCell4555State>>,
) {
    query.par_iter_mut().for_each(|(state, mut visible)| {
        *visible = if state.0 {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        }
    });
}
