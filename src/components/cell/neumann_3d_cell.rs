use crate::components::Cell;
use bevy::prelude::{Component, IVec3};
use std::ops::Deref;

const NEIGHBOR_COORDINATES: [IVec3; 6] = [
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

/// [Neumann] Classic cube 3D cell, it has 6 neighbors and uses `IVec3`
/// coordinates
///
/// [Neumann]: https://en.wikipedia.org/wiki/Von_Neumann_neighborhood
#[derive(Debug, Clone, Component)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
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

    #[inline]
    fn coords(&self) -> &Self::Coordinates {
        &self.coords
    }

    #[inline]
    fn neighbor_coordinates(&self) -> impl ExactSizeIterator<Item = Self::Coordinates> + '_ {
        NEIGHBOR_COORDINATES.map(|c| c + *self.coords()).into_iter()
    }
}

impl NeumannCell3d {
    /// Instantiates a new cell with `coords` values
    #[must_use]
    #[inline]
    pub const fn new(coords: IVec3) -> Self {
        Self { coords }
    }
}
