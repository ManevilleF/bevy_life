use crate::components::{Cell, CellState};
use crate::resources::CellMap;
use bevy::ecs::component::Component;
use bevy::log;
use bevy::prelude::*;
use bevy::tasks::ComputeTaskPool;
use std::sync::{Arc, RwLock};

pub fn handle_cells<C, S, const BATCH_SIZE: usize>(
    mut commands: Commands,
    pool: Res<ComputeTaskPool>,
    query: Query<(Entity, &C, &S)>,
    map: Res<CellMap<C>>,
) where
    C: Cell + Component,
    S: CellState + Component,
{
    let vec = Arc::new(RwLock::new(Vec::new()));
    query.par_for_each(&pool, BATCH_SIZE, |(entity, cell, state)| {
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
            let mut lock = vec.write().unwrap();
            lock.push((entity, new_state));
        }
    });
    let lock = vec.read().unwrap();
    for (e, s) in lock.iter() {
        commands.entity(*e).insert(s.clone());
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
