use bevy::prelude::*;
use bevy_life::{CellMap, ConwayCell4555State, GameOfLife3dPlugin, MooreCell3d, SimulationBatch};
use rand::Rng;

pub struct MapEntity(pub Entity);

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
        .add_system(handle_reset_3d)
        .run();
}

pub fn handle_reset_3d(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    map: Res<MapEntity>,
    mut cell_map: ResMut<CellMap<MooreCell3d>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if keys.just_released(KeyCode::Space) {
        commands.entity(map.0).despawn_recursive();
        commands.remove_resource::<MapEntity>();
        cell_map.clear();
        println!("regenerating map");
        spawn_map(&mut commands, &mut meshes);
    }
}

fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(50., 50., -50.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn setup_map(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    // map
    spawn_map(&mut commands, &mut meshes);
}

fn spawn_map(commands: &mut Commands, meshes: &mut Assets<Mesh>) {
    let mesh = meshes.add(Mesh::from(shape::Cube::new(1.)));
    let mut rng = rand::thread_rng();
    let map_size = 50;
    let entity = commands
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
                                ..Default::default()
                            })
                            .insert(MooreCell3d::new(IVec3::new(x, y, z)))
                            .insert(state);
                    }
                }
            }
        })
        .id();
    commands.insert_resource(MapEntity(entity));
    println!("map generated");
}
