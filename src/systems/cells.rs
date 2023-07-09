use crate::components::{Cell, CellState};
use crate::resources::CellMap;
use crate::{SimulationBatch, SimulationPause};
use bevy::log;
use bevy::prelude::*;
use bevy::utils::HashMap;

fn handle_cell<C, S>((cell, state): (&C, &S), map: &HashMap<C::Coordinates, S>) -> Option<S>
where
    C: Cell,
    S: CellState,
{
    let neighbor_coords = cell.neighbor_coordinates();
    let neighbor_states = neighbor_coords.iter().filter_map(|c| map.get(c));
    let new_state = state.new_cell_state(neighbor_states);
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
    pause: Option<Res<SimulationPause>>,
    batch: Option<Res<SimulationBatch>>,
) where
    C: Cell,
    S: CellState,
{
    if pause.is_some() {
        return;
    }
    let map: HashMap<_, _> = query
        .iter()
        .map(|(_entity, cell, state)| (cell.coords().clone(), state.clone()))
        .collect();
    if batch.is_some() {
        query.par_iter().for_each(|(entity, cell, state)| {
            if let Some(new_state) = handle_cell((cell, state), &map) {
                par_commands.command_scope(|mut cmd| {
                    cmd.entity(entity).insert(new_state);
                });
            }
        });
    } else {
        for (entity, cell, state) in query.iter() {
            if let Some(new_state) = handle_cell((cell, state), &map) {
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
    if removed_cells.is_empty() {
        return;
    }
    log::trace!("Removing {} cells from cell map", removed_cells.len());
    map.remove_entities(removed_cells.iter());
}
