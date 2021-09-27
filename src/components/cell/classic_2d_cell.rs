use crate::components::cell::Cell;
use bevy::prelude::IVec2;
use std::ops::Deref;

#[derive(Debug)]
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

    fn neighbor_coordinates(cell_coordinates: Self::Coordinates) -> Vec<Self::Coordinates> {
        vec![
            // Bottom left
            cell_coordinates + IVec2::new(-1, 0),
            // Top Left
            cell_coordinates + IVec2::new(-1, 1),
            // Top
            cell_coordinates + IVec2::new(0, 1),
            // Top Right
            cell_coordinates + IVec2::new(1, 1),
            // Right
            cell_coordinates + IVec2::new(1, 0),
            // Bottom Right
            cell_coordinates + IVec2::new(1, -1),
            // Bottom
            cell_coordinates + IVec2::new(0, -1),
            // Bottom Left
            cell_coordinates + IVec2::new(-1, -1),
        ]
    }
}
