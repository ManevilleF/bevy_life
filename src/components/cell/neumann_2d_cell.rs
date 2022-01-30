use crate::components::Cell;
use bevy::prelude::{Component, IVec2, Reflect};
use std::ops::Deref;

lazy_static::lazy_static! {
    static ref NEIGHBOR_COORDINATES: [IVec2; 4] = [
        // Left
        IVec2::new(-1, 0),
        // Top
        IVec2::new(0, 1),
        // Right
        IVec2::new(1, 0),
        // Bottom
        IVec2::new(0, -1),
    ];
}

/// [Neumann] 2D cell. It has 4 neighbors and uses `IVec2` coordinates.
///
/// ```ascii
///         +-------+
///         |       |
///         |  0,1  |  
///         |       |
/// +-------+-------+-------+
/// |       |       |       |
/// | -1,0  |  0,0  |  0,1  |
/// |       |       |       |
/// +-------+-------+-------+
///         |       |
///         |  0,-1 |
///         |       |
///         +-------+
/// ```
/// [Neumann]: https://en.wikipedia.org/wiki/Von_Neumann_neighborhood
#[derive(Debug, Clone, Component, Reflect)]
pub struct NeumannCell2d {
    /// The 2D cell coordinates
    pub coords: IVec2,
}

impl Deref for NeumannCell2d {
    type Target = IVec2;

    fn deref(&self) -> &Self::Target {
        &self.coords
    }
}

impl Cell for NeumannCell2d {
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

impl NeumannCell2d {
    /// Instantiates a new cell with `coords` values
    #[must_use]
    #[inline]
    pub const fn new(coords: IVec2) -> Self {
        Self { coords }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_coordinates() {
        let cell = NeumannCell2d {
            coords: IVec2::new(10, 10),
        };
        let neighbors = cell.neighbor_coordinates();
        assert_eq!(
            neighbors,
            vec![
                // Left
                IVec2::new(9, 10),
                // Top
                IVec2::new(10, 11),
                // Right
                IVec2::new(11, 10),
                // Bottom
                IVec2::new(10, 9),
            ]
        );
    }

    #[test]
    fn correct_coordinates_negative() {
        let cell = NeumannCell2d {
            coords: IVec2::new(-10, 10),
        };
        let neighbors = cell.neighbor_coordinates();
        assert_eq!(
            neighbors,
            vec![
                // Left
                IVec2::new(-11, 10),
                // Top
                IVec2::new(-10, 11),
                // Right
                IVec2::new(-9, 10),
                // Bottom
                IVec2::new(-10, 9),
            ]
        );
    }

    #[test]
    fn correct_coordinates_origin() {
        let cell = NeumannCell2d {
            coords: IVec2::new(0, 0),
        };
        let neighbors = cell.neighbor_coordinates();
        assert_eq!(
            neighbors,
            vec![
                // Left
                IVec2::new(-1, 0),
                // Top
                IVec2::new(0, 1),
                // Right
                IVec2::new(1, 0),
                // Bottom
                IVec2::new(0, -1),
            ]
        );
    }
}
