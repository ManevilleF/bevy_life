use crate::components::cell::Cell;
use crate::components::CellState;
use crate::resources::CellMap;
use bevy::ecs::component::Component;
use bevy::prelude::*;
use std::fmt::Debug;

pub fn handle_cells<C, S>(
    mut commands: Commands,
    query: Query<(Entity, &C, &S)>,
    map: Res<CellMap<C>>,
) where
    C: Cell + Component + Debug,
    S: CellState + Component + Debug,
{
}
