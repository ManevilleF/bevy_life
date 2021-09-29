use crate::components::Cell;
#[cfg(feature = "2D")]
use crate::components::Cell2d;
#[cfg(feature = "3D")]
use crate::components::Cell3d;
use bevy::prelude::Entity;
use std::collections::HashMap;

#[cfg(feature = "2D")]
pub type Map2d = CellMap<Cell2d>;
#[cfg(feature = "3D")]
pub type Map3d = CellMap<Cell3d>;

/// Global Cell container resource , uses a `Hashmap`to allow non-continuous cells.
///
/// The resource is automatically added and refreshed, it may be used for clearing (see examples).
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

    pub fn clear(&mut self) {
        self.cells.clear();
    }
}
