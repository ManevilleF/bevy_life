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

pub trait Cell: Debug {
    type Coordinates: Clone + Debug + Send + Sync + Eq + Hash;

    fn coords(&self) -> &Self::Coordinates;

    fn neighbor_coordinates(&self) -> Vec<Self::Coordinates>;
}
