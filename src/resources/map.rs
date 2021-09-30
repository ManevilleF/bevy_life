use crate::components::Cell;
#[cfg(feature = "2D")]
use crate::components::MooreCell2d;
#[cfg(feature = "3D")]
use crate::components::NeumannCell3d;
use bevy::prelude::Entity;
use std::collections::HashMap;

#[cfg(feature = "2D")]
/// A `CellMap` implementation for `Cell2d`
pub type Map2d = CellMap<MooreCell2d>;
#[cfg(feature = "3D")]
/// A `CellMap` implementation for `Cell23d`
pub type Map3d = CellMap<NeumannCell3d>;

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
    /// Retrieves every cell entity matching `coords`.
    /// If some coordinates are not stored in the cell map they will be ignored.
    pub fn get_cell_entities(&self, coords: &[C::Coordinates]) -> Vec<Entity> {
        coords
            .iter()
            .filter_map(|c| self.cells.get(c))
            .copied()
            .collect()
    }

    /// Adds a `Cell` entity to the map at `coordinates`.
    ///
    /// # Note:
    ///
    /// This operation is done automatically when you add a `Cell` component to an entity.
    pub fn insert_cell(&mut self, coordinates: C::Coordinates, entity: Entity) {
        self.cells.insert(coordinates, entity);
    }

    /// Removes a cell from the map, returning the `Entity` value if it was present.
    ///
    /// # Note:
    ///
    /// Use this method to remove cell entities from the map if you remove a `Cell` component from an `Entity`
    /// or *despawn* an `Entity` with a `Cell` component.
    pub fn remove_cell(&mut self, coordinates: &C::Coordinates) -> Option<Entity> {
        self.cells.remove(coordinates)
    }

    /// Clears the entire map
    pub fn clear(&mut self) {
        self.cells.clear();
    }
}
