use bevy::prelude::*;
use bevy_game_of_life::{Cell2d, ClassicCellState, ClassicGameOfLifePlugin};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(ClassicGameOfLifePlugin::new(0.2))
        .insert_resource(WindowDescriptor {
            width: 500.,
            height: 500.,
            ..Default::default()
        })
        .add_startup_system(generate_map.system())
        .run();
}

fn generate_map() {}
