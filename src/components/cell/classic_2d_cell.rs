use crate::components::cell::Cell;
use bevy::prelude::IVec2;
use std::ops::Deref;

lazy_static::lazy_static! {
    static ref NEIGHBOR_COORDINATES: [IVec2; 8] = [
        // Bottom left
        IVec2::new(-1, 0),
        // Top Left
        IVec2::new(-1, 1),
        // Top
        IVec2::new(0, 1),
        // Top Right
        IVec2::new(1, 1),
        // Right
        IVec2::new(1, 0),
        // Bottom Right
        IVec2::new(1, -1),
        // Bottom
        IVec2::new(0, -1),
        // Bottom Left
        IVec2::new(-1, -1),
    ];
}

#[derive(Debug, Clone)]
pub struct Cell2d {
    pub coords: IVec2,
}

impl Deref for Cell2d {
    type Target = IVec2;

    fn deref(&self) -> &Self::Target {
        &self.coords
    }
}

impl Cell for Cell2d {
    type Coordinates = IVec2;

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
