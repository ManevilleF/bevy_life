use bevy::prelude::*;
use bevy_life::{MooreCell2d, RainbowCellState, RainbowGame2dPlugin};
use rand::Rng;

mod common;

use common::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(RainbowGame2dPlugin::default())
        .insert_resource(WindowDescriptor {
            title: "Rainbow game".to_string(),
            width: 1000.,
            height: 1000.,
            ..Default::default()
        })
        .add_startup_system(setup_camera.system())
        .add_startup_system(setup_map.system())
        .add_system(handle_reset::<MooreCell2d>.system())
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
    let map_size = 100;
    let sprite_size = 10.;
    let material = assets.add(Color::rgba(0., 0., 0., 0.).into());

    let entity = commands
        .spawn()
        .insert(Transform::from_xyz(
            -(map_size as f32 * sprite_size) / 2.,
            -(map_size as f32 * sprite_size) / 2.,
            0.,
        ))
        .insert(GlobalTransform::default())
        .with_children(|builder| {
            for y in 0..=map_size {
                for x in 0..=map_size {
                    let state = if rng.gen_bool(1. / 3.) {
                        RainbowCellState::Alive(if rng.gen_bool(1. / 2.) { 0. } else { 1. })
                    } else {
                        RainbowCellState::Dead
                    };
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
