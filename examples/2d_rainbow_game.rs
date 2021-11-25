use bevy::prelude::{
    App, AssetServer, BuildChildren, Commands, GlobalTransform, Handle, IVec2, Res, Transform,
    Vec2, WindowDescriptor,
};
use bevy::render2::{camera::OrthographicCameraBundle, color::Color, texture::Image};
use bevy::sprite2::*;
use bevy::PipelinedDefaultPlugins;
use bevy_life::{MooreCell2d, RainbowCellState, RainbowGame2dPlugin, SimulationBatch};
use rand::Rng;

mod common;

use common::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Rainbow game".to_string(),
            width: 1300.,
            height: 800.,
            ..Default::default()
        })
        .add_plugins(PipelinedDefaultPlugins)
        .add_plugin(RainbowGame2dPlugin::default())
        .insert_resource(SimulationBatch::default())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_map)
        .add_system(handle_reset_2d::<MooreCell2d>)
        .run();
}

fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("square.png");
    commands.insert_resource(TileTexture(texture.clone()));
    // map
    spawn_map(&mut commands, texture);
}

fn spawn_map(commands: &mut Commands, texture: Handle<Image>) {
    let mut rng = rand::thread_rng();
    let (size_x, size_y) = (300, 200);
    let sprite_size = 4.;
    let color = Color::rgba(1., 0., 0., 1.);

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
                    let state = if rng.gen_bool(1. / 3.) {
                        RainbowCellState::Alive(if rng.gen_bool(1. / 2.) { 0. } else { 1. })
                    } else {
                        RainbowCellState::Dead
                    };
                    builder
                        .spawn_bundle(PipelinedSpriteBundle {
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
                            texture: texture.clone(),
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
