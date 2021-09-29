use crate::components::cell::Cell;
#[cfg(feature = "2D")]
use crate::components::cell::Cell2d;
#[cfg(feature = "3D")]
use crate::components::cell::Cell3d;
use bevy::prelude::Entity;
use std::collections::HashMap;

#[cfg(feature = "2D")]
pub type Map2d = CellMap<Cell2d>;
#[cfg(feature = "3D")]
pub type Map3d = CellMap<Cell3d>;

/// Button action type
#[derive(Clone)]
pub struct CellMap<C: Cell> {
    cells: HashMap<C::Coordinates, Entity>,
}

impl<C: Cell> Default for CellMap<C> {
    fn default() -> Self {
        Self {
            cells: Default::default(),
        }
    }
}

impl<C: Cell> CellMap<C> {
    pub fn get_cell_entities(&self, coords: &[C::Coordinates]) -> Vec<Entity> {
        coords
            .iter()
            .filter_map(|c| self.cells.get(c))
            .copied()
            .collect()
    }

    pub fn insert_cell(&mut self, coordinates: C::Coordinates, entity: Entity) {
        self.cells.insert(coordinates, entity);
    }
}
