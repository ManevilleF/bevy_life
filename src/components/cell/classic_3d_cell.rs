use crate::components::cell::Cell;
use bevy::prelude::IVec3;
use std::ops::Deref;

#[derive(Debug)]
pub struct Cell3d {
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

    fn neighbor_coordinates(cell_coordinates: Self::Coordinates) -> Vec<Self::Coordinates> {
        vec![
            // Z - 1

            // Center
            cell_coordinates + IVec3::new(0, 0, -1),
            // Bottom left
            cell_coordinates + IVec3::new(-1, 0, -1),
            // Top Left
            cell_coordinates + IVec3::new(-1, 1, -1),
            // Top
            cell_coordinates + IVec3::new(0, 1, -1),
            // Top Right
            cell_coordinates + IVec3::new(1, 1, -1),
            // Right
            cell_coordinates + IVec3::new(1, 0, -1),
            // Bottom Right
            cell_coordinates + IVec3::new(1, -1, -1),
            // Bottom
            cell_coordinates + IVec3::new(0, -1, -1),
            // Bottom Left
            cell_coordinates + IVec3::new(-1, -1, -1),
            // Z

            // Bottom left
            cell_coordinates + IVec3::new(-1, 0, 0),
            // Top Left
            cell_coordinates + IVec3::new(-1, 1, 0),
            // Top
            cell_coordinates + IVec3::new(0, 1, 0),
            // Top Right
            cell_coordinates + IVec3::new(1, 1, 0),
            // Right
            cell_coordinates + IVec3::new(1, 0, 0),
            // Bottom Right
            cell_coordinates + IVec3::new(1, -1, 0),
            // Bottom
            cell_coordinates + IVec3::new(0, -1, 0),
            // Bottom Left
            cell_coordinates + IVec3::new(-1, -1, 0),
            // Z + 1

            // Center
            cell_coordinates + IVec3::new(0, 0, 1),
            // Bottom left
            cell_coordinates + IVec3::new(-1, 0, 1),
            // Top Left
            cell_coordinates + IVec3::new(-1, 1, 1),
            // Top
            cell_coordinates + IVec3::new(0, 1, 1),
            // Top Right
            cell_coordinates + IVec3::new(1, 1, 1),
            // Right
            cell_coordinates + IVec3::new(1, 0, 1),
            // Bottom Right
            cell_coordinates + IVec3::new(1, -1, 1),
            // Bottom
            cell_coordinates + IVec3::new(0, -1, 1),
            // Bottom Left
            cell_coordinates + IVec3::new(-1, -1, 1),
        ]
    }
}
