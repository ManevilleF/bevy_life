use bevy::prelude::*;
use bevy_game_of_life::{ClassicGameOfLife2dPlugin, NewCell2d, NewClassicCellState};
use rand::Rng;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(ClassicGameOfLife2dPlugin::default())
        .insert_resource(WindowDescriptor {
            title: "Game Of Life".to_string(),
            width: 800.,
            height: 800.,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, mut assets: ResMut<Assets<ColorMaterial>>) {
    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // map
    let mut rng = rand::thread_rng();
    let map_size = 100;
    let sprite_size = 10.;
    let material = assets.add(Color::rgba(0., 0., 0., 0.).into());
    for y in 0..=map_size {
        for x in 0..=map_size {
            let state = NewClassicCellState::new(rng.gen_bool(1. / 3.));
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite::new(Vec2::splat(sprite_size - 1.)),
                    transform: Transform::from_xyz(
                        sprite_size * (x - map_size / 2) as f32,
                        sprite_size * (y - map_size / 2) as f32,
                        0.,
                    ),
                    material: material.clone(),
                    ..Default::default()
                })
                .insert(NewCell2d::new(IVec2::new(x, y)))
                .insert(state);
        }
    }
    println!("map generated");
}
