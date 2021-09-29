use crate::components::{Cell, CellState};
use crate::resources::CellMap;
use bevy::ecs::component::Component;
use bevy::log;
use bevy::prelude::*;

pub fn handle_cells<C, S>(
    mut commands: Commands,
    query: Query<(Entity, &C, &S)>,
    map: Res<CellMap<C>>,
) where
    C: Cell + Component,
    S: CellState + Component,
{
    for (entity, cell, state) in query.iter() {
        let neighbor_coords = cell.neighbor_coordinates();
        let neighbor_cells = map.get_cell_entities(&neighbor_coords);
        let neighbor_states: Vec<S> = neighbor_cells
            .into_iter()
            .filter_map(|e| match query.get(e) {
                Ok((_e, _c, s)) => Some(s.clone()),
                Err(e) => {
                    log::error!("Query error: {}", e);
                    None
                }
            })
            .collect();
        let new_state = state.new_cell_state(&neighbor_states);
        if &new_state != state {
            let mut entity_cmd = commands.entity(entity);
            entity_cmd.insert(new_state);
        }
    }
}

pub fn handle_new_cells<C>(query: Query<(Entity, &C), Added<C>>, mut map: ResMut<CellMap<C>>)
where
    C: Cell + Component,
{
    for (entity, new_cell) in query.iter() {
        map.insert_cell(new_cell.coords().clone(), entity);
    }
}
