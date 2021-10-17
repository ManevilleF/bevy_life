use crate::components::{Cell, CellState};
use crate::resources::CellMap;
use crate::{SimulationBatch, SimulationPause};
use bevy::log;
use bevy::prelude::*;
use bevy::tasks::ComputeTaskPool;
use std::sync::RwLock;

fn handle_cell<C, S>(
    (cell, state): (&C, &S),
    map: &CellMap<C>,
    query: &Query<(Entity, &C, &S)>,
) -> Option<S>
where
    C: Cell,
    S: CellState,
{
    let neighbor_coords = cell.neighbor_coordinates();
    let neighbor_cells = map.get_cell_entities(&neighbor_coords);
    let neighbor_states: Vec<S> = neighbor_cells
        .into_iter()
        .filter_map(|e| match query.get(e) {
            Ok((_e, _c, s)) => Some(s.clone()),
            Err(err) => {
                log::error!(
                    "Could not retrieve neighbor entity {:?} for cell at {:?}: {}",
                    e,
                    cell.coords(),
                    err
                );
                None
            }
        })
        .collect();
    let new_state = state.new_cell_state(&neighbor_states);
    if &new_state != state {
        Some(new_state)
    } else {
        None
    }
}

pub fn handle_cells<C, S>(
    mut commands: Commands,
    pool: Res<ComputeTaskPool>,
    query: Query<(Entity, &C, &S)>,
    map: Res<CellMap<C>>,
    pause: Option<Res<SimulationPause>>,
    batch: Option<Res<SimulationBatch>>,
) where
    C: Cell,
    S: CellState,
{
    if pause.is_some() {
        return;
    }
    if let Some(config) = batch {
        let vec = RwLock::new(Vec::new());
        query.par_for_each(&pool, config.batch_size, |(entity, cell, state)| {
            if let Some(new_state) = handle_cell((cell, state), &map, &query) {
                let mut lock = vec.write().unwrap();
                lock.push((entity, new_state));
            }
        });
        let lock = vec.read().unwrap();
        for (e, s) in lock.iter() {
            commands.entity(*e).insert(s.clone());
        }
    } else {
        for (entity, cell, state) in query.iter() {
            if let Some(new_state) = handle_cell((cell, state), &map, &query) {
                commands.entity(entity).insert(new_state);
            }
        }
    }
}

pub fn handle_new_cells<C>(query: Query<(Entity, &C), Added<C>>, mut map: ResMut<CellMap<C>>)
where
    C: Cell,
{
    for (entity, new_cell) in query.iter() {
        let old_entity = map.insert_cell(new_cell.coords().clone(), entity);
        if let Some(e) = old_entity {
            if e != entity {
                log::warn!(
                    "{:?} replaced {:?} at {:?} coordinates",
                    entity,
                    e,
                    new_cell.coords()
                )
            }
        }
    }
}
