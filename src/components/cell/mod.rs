#[cfg(feature = "2D")]
pub use classic_2d_cell::*;
#[cfg(feature = "3D")]
pub use classic_3d_cell::*;
use std::fmt::Debug;

#[cfg(feature = "2D")]
mod classic_2d_cell;
#[cfg(feature = "3D")]
mod classic_3d_cell;

pub trait Cell {
    type Coordinates: Clone + Debug + Send + Sync;

    fn coords(&self) -> &Self::Coordinates;

    fn neighbor_coordinates(cell_coordinates: Self::Coordinates) -> Vec<Self::Coordinates>;
}
