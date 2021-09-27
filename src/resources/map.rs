use crate::components::cell::Cell;
#[cfg(feature = "2D")]
use crate::components::cell::Cell2d;
#[cfg(feature = "3D")]
use crate::components::cell::Cell3d;
use bevy::prelude::Entity;
use std::collections::HashMap;
use std::fmt::Debug;

#[cfg(feature = "2D")]
pub type Map2d = CellMap<Cell2d>;
#[cfg(feature = "3D")]
pub type Map3d = CellMap<Cell3d>;

#[derive(Clone)]
pub struct CellMap<C: Cell + Debug> {
    cells: HashMap<C::Coordinates, Entity>,
}

impl<C: Cell + Debug> Default for CellMap<C> {
    fn default() -> Self {
        Self {
            cells: Default::default(),
        }
    }
}
