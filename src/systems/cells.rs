use crate::components::cell::Cell;
use crate::components::CellState;
use crate::resources::CellMap;
use crate::NewState;
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
    for (entity, cell, state) in query.iter() {
        let neighbor_coords = cell.neighbor_coordinates();
        let neighbor_cells = map.get_cell_entities(&neighbor_coords);
        let neighbor_states: Vec<&S> = neighbor_cells
            .into_iter()
            .map(|e| query.get(e).unwrap().2)
            .collect();
        let new_state = state.new_cell_state(&neighbor_states);
        let mut entity_cmd = commands.entity(entity);
        entity_cmd.insert(NewState(new_state));
    }
}
