#![allow(dead_code)]
use crate::spawn_map;
use bevy::ecs::component::Component;
use bevy::prelude::{
    Commands, DespawnRecursiveExt, Entity, IVec2, Input, KeyCode, Res, ResMut, Vec2, Window,
};
use bevy_life::{Cell, CellMap};

pub struct MapEntity(pub Entity);

pub fn mouse_coords(window: &Window, position: Vec2) -> Vec2 {
    let window_size = Vec2::new(window.width(), window.height());
    position - window_size / 2.
}

pub fn mouse_coords_to_cell(world_pos: Vec2, cell_size: i32) -> IVec2 {
    let mouse_position = world_pos / cell_size as f32;
    IVec2::new(
        mouse_position.x.round() as i32,
        mouse_position.y.round() as i32,
    )
}

pub fn handle_reset_2d<C>(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    map: Res<MapEntity>,
    mut cell_map: ResMut<CellMap<C>>,
) where
    C: Cell + Component,
{
    if keys.just_released(KeyCode::Space) {
        commands.entity(map.0).despawn_recursive();
        commands.remove_resource::<MapEntity>();
        cell_map.clear();
        println!("regenerating map");
        spawn_map(&mut commands);
    }
}
