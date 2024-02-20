use bevy::prelude::Component;
use std::{fmt::Debug, hash::Hash};
#[cfg(feature = "2D")]
pub use {hexagon_2d_cell::*, moore_2d_cell::*, neumann_2d_cell::*};
#[cfg(feature = "3D")]
pub use {moore_3d_cell::*, neumann_3d_cell::*};

#[cfg(feature = "2D")]
mod hexagon_2d_cell;
#[cfg(feature = "2D")]
mod moore_2d_cell;
#[cfg(feature = "3D")]
mod moore_3d_cell;
#[cfg(feature = "2D")]
mod neumann_2d_cell;
#[cfg(feature = "3D")]
mod neumann_3d_cell;

/// Trait defining a Cell, every cell type (2d, 3d, hexagonal, etc) must
/// implement this trait and define an associated `Coordinates` type
pub trait Cell: Clone + Component {
    /// Associated coordinates type
    type Coordinates: Clone + Debug + Send + Sync + Eq + Hash;

    /// Retrieves the cell coordinates
    #[must_use]
    fn coords(&self) -> &Self::Coordinates;

    /// Retrieves the coordinates of the neighbor cells
    #[must_use]
    fn neighbor_coordinates(&self) -> impl IntoIterator<Item = Self::Coordinates>;
}
