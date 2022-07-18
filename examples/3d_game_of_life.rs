use bevy::prelude::*;
use bevy_life::{ConwayCell4555State, GameOfLife3dPlugin, MooreCell3d, SimulationBatch};
use rand::Rng;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "3D Game Of Life".to_string(),
            width: 1300.,
            height: 800.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GameOfLife3dPlugin::default())
        .insert_resource(SimulationBatch::default())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_map)
        .add_system(color)
        .run();
}

fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(50., 50., -100.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn setup_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Cube::new(1.)));
    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        unlit: true,
        ..Default::default()
    });
    // map
    spawn_map(&mut commands, mesh, material);
}

fn spawn_map(commands: &mut Commands, mesh: Handle<Mesh>, material: Handle<StandardMaterial>) {
    let mut rng = rand::thread_rng();
    let map_size = 60;
    commands
        .spawn()
        .insert(Transform::from_xyz(
            -(map_size as f32) / 2.,
            -(map_size as f32) / 2.,
            -(map_size as f32) / 2.,
        ))
        .insert(GlobalTransform::default())
        .with_children(|builder| {
            for z in 0..=map_size {
                for y in 0..=map_size {
                    for x in 0..=map_size {
                        let state = ConwayCell4555State(rng.gen_bool(1. / 3.));
                        builder
                            .spawn_bundle(PbrBundle {
                                mesh: mesh.clone(),
                                transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                                material: material.clone(),
                                ..Default::default()
                            })
                            .insert(MooreCell3d::new(IVec3::new(x, y, z)))
                            .insert(state);
                    }
                }
            }
        });
    println!("map generated");
}

pub fn color(
    mut query: Query<(&ConwayCell4555State, &mut Visibility), Changed<ConwayCell4555State>>,
) {
    for (state, mut visible) in query.iter_mut() {
        visible.is_visible = state.0
    }
}
