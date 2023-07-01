use crate::components::{Cell, CellState};
use crate::resources::CellMap;
use crate::{SimulationBatch, SimulationPause};
use bevy::log;
use bevy::prelude::*;

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
        .filter_map(|e| match query.get(*e) {
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
    if &new_state == state {
        None
    } else {
        Some(new_state)
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn handle_cells<C, S>(
    mut commands: Commands,
    par_commands: ParallelCommands,
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
    if batch.is_some() {
        query.par_iter().for_each(|(entity, cell, state)| {
            if let Some(new_state) = handle_cell((cell, state), &map, &query) {
                par_commands.command_scope(|mut cmd| {
                    cmd.entity(entity).insert(new_state);
                });
            }
        });
    } else {
        for (entity, cell, state) in query.iter() {
            if let Some(new_state) = handle_cell((cell, state), &map, &query) {
                commands.entity(entity).insert(new_state);
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
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
                );
            }
        }
    }
}

pub fn handle_removed_cells<C>(mut removed_cells: RemovedComponents<C>, mut map: ResMut<CellMap<C>>)
where
    C: Cell,
{
    for entity in removed_cells.iter() {
        map.remove_entity(entity);
    }
}
