use crate::components::Cell;
use bevy::prelude::{Component, IVec3};
use std::ops::Deref;

lazy_static::lazy_static! {
    static ref NEIGHBOR_COORDINATES: [IVec3; 6] = [
        // Z - 1

        // Center
        IVec3::new(0, 0, -1),

        // Z

        // Left
        IVec3::new(-1, 0, 0),
        // Top
        IVec3::new(0, 1, 0),
        // Right
        IVec3::new(1, 0, 0),
        // Bottom
        IVec3::new(0, -1, 0),

        // Z + 1

        // Center
        IVec3::new(0, 0, 1),
    ];
}

/// [Neumann] Classic cube 3D cell, it has 6 neighbors and uses `IVec3` coordinates
///
/// [Neumann]: https://en.wikipedia.org/wiki/Von_Neumann_neighborhood
#[derive(Debug, Clone, Component)]
pub struct NeumannCell3d {
    /// The 3D cell coordinates
    pub coords: IVec3,
}

impl Deref for NeumannCell3d {
    type Target = IVec3;

    fn deref(&self) -> &Self::Target {
        &self.coords
    }
}

impl Cell for NeumannCell3d {
    type Coordinates = IVec3;

    fn coords(&self) -> &Self::Coordinates {
        &self.coords
    }

    fn neighbor_coordinates(&self) -> Vec<Self::Coordinates> {
        NEIGHBOR_COORDINATES
            .iter()
            .map(|c| *c + *self.coords())
            .collect()
    }
}

impl NeumannCell3d {
    /// Instantiates a new cell with `coords` values
    pub const fn new(coords: IVec3) -> Self {
        Self { coords }
    }
}
