use crate::components::cell::Cell;
use crate::components::CellState;
use crate::resources::CellMap;
use crate::{NewCell, NewState};
use bevy::ecs::component::Component;
use bevy::prelude::*;

pub fn handle_cells<C, S>(
    mut commands: Commands,
    query: Query<(Entity, &C, &S)>,
    map: Res<CellMap<C>>,
) where
    C: Cell + Component,
    S: CellState + Component,
{
    println!("handle_cells");
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

pub fn handle_new_cells<C>(
    mut commands: Commands,
    query: Query<(Entity, &NewCell<C>)>,
    mut map: ResMut<CellMap<C>>,
) where
    C: Cell + Component,
{
    println!("handle_new_cells");
    for (entity, new_cell) in query.iter() {
        let mut entity_cmd = commands.entity(entity);
        entity_cmd.insert(new_cell.0.clone());
        entity_cmd.remove::<NewCell<C>>();
        map.insert_cell(new_cell.0.coords().clone(), entity);
    }
}

pub fn handle_new_states<S>(mut commands: Commands, query: Query<(Entity, &NewState<S>)>)
where
    S: CellState + Component,
{
    println!("handle_new_states");
    for (entity, new_state) in query.iter() {
        let mut entity_cmd = commands.entity(entity);
        entity_cmd.insert(new_state.0.clone());
        entity_cmd.remove::<NewState<S>>();
    }
}
