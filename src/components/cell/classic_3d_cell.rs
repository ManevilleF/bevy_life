use crate::components::Cell;
use bevy::prelude::IVec3;
use std::ops::Deref;

lazy_static::lazy_static! {
    static ref NEIGHBOR_COORDINATES: [IVec3; 26] = [
        // Z - 1

        // Center
        IVec3::new(0, 0, -1),
        // Bottom left
        IVec3::new(-1, 0, -1),
        // Top Left
        IVec3::new(-1, 1, -1),
        // Top
        IVec3::new(0, 1, -1),
        // Top Right
        IVec3::new(1, 1, -1),
        // Right
        IVec3::new(1, 0, -1),
        // Bottom Right
        IVec3::new(1, -1, -1),
        // Bottom
        IVec3::new(0, -1, -1),
        // Bottom Left
        IVec3::new(-1, -1, -1),
        // Z

        // Bottom left
        IVec3::new(-1, 0, 0),
        // Top Left
        IVec3::new(-1, 1, 0),
        // Top
        IVec3::new(0, 1, 0),
        // Top Right
        IVec3::new(1, 1, 0),
        // Right
        IVec3::new(1, 0, 0),
        // Bottom Right
        IVec3::new(1, -1, 0),
        // Bottom
        IVec3::new(0, -1, 0),
        // Bottom Left
        IVec3::new(-1, -1, 0),
        // Z + 1

        // Center
        IVec3::new(0, 0, 1),
        // Bottom left
        IVec3::new(-1, 0, 1),
        // Top Left
        IVec3::new(-1, 1, 1),
        // Top
        IVec3::new(0, 1, 1),
        // Top Right
        IVec3::new(1, 1, 1),
        // Right
        IVec3::new(1, 0, 1),
        // Bottom Right
        IVec3::new(1, -1, 1),
        // Bottom
        IVec3::new(0, -1, 1),
        // Bottom Left
        IVec3::new(-1, -1, 1),
    ];
}

/// Classic cube 3D cell, it has 26 neighbors and uses `IVec3` coordinates
#[derive(Debug, Clone)]
pub struct Cell3d {
    /// The 3D cell coordinates
    pub coords: IVec3,
}

impl Deref for Cell3d {
    type Target = IVec3;

    fn deref(&self) -> &Self::Target {
        &self.coords
    }
}

impl Cell for Cell3d {
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

impl Cell3d {
    /// Instantiates a new `Cell3D` with `coords` values
    pub fn new(coords: IVec3) -> Self {
        Self { coords }
    }
}
