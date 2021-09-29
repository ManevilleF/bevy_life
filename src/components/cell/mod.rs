#[cfg(feature = "2D")]
pub use classic_2d_cell::*;
#[cfg(feature = "3D")]
pub use classic_3d_cell::*;
use std::fmt::Debug;
use std::hash::Hash;

#[cfg(feature = "2D")]
mod classic_2d_cell;
#[cfg(feature = "3D")]
mod classic_3d_cell;

/// Trait defining a Cell, every cell type (2d, 3d, hexagonal, etc) must implement this trait
/// and define an associated `Coordinates` type
pub trait Cell: Debug + Clone {
    type Coordinates: Clone + Debug + Default + Send + Sync + Eq + Hash;

    /// Retrieves the cell coordinates
    fn coords(&self) -> &Self::Coordinates;

    /// Retrieves the coordinates of the neighbor cells
    fn neighbor_coordinates(&self) -> Vec<Self::Coordinates>;
}
