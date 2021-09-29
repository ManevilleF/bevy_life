#[cfg(feature = "auto-coloring")]
use bevy::prelude::Assets;
use std::fmt::Debug;
pub use {classic_state::*, cyclic_state::*, wire_world_cell_state::*};

mod classic_state;
mod cyclic_state;
mod wire_world_cell_state;

/// This trait defines the state of any given `Cell`. The trait implementation will define the
/// cellular automaton rules which will be automatically applied.
///
/// Every type defining a `Cell` state and rules must implement this trait.
pub trait CellState: Debug + Default + Sized + Clone + PartialEq {
    /// Defines the new state for a cell given the `neighbor_cells` states and `self`.
    ///
    /// This method defines the cellular automaton rules
    fn new_cell_state(&self, neighbor_cells: &[Self]) -> Self;

    /// Mutably applies the new state defined in `new_cell_state` to `self`
    fn apply_new_cell_state(&mut self, neighbor_cells: &[Self]) {
        *self = self.new_cell_state(neighbor_cells)
    }

    #[cfg(feature = "auto-coloring")]
    /// Index of the material handle matching the current `self` state
    fn material_index(&self) -> usize;

    #[cfg(all(feature = "auto-coloring", feature = "2D"))]
    /// Builds the `CellStateMaterials2d` ressource storing every material handle for every possible state.
    fn setup_materials_2d(
        materials: &mut Assets<bevy::prelude::ColorMaterial>,
    ) -> crate::resources::materials::CellStateMaterials2d;

    #[cfg(all(feature = "auto-coloring", feature = "3D"))]
    /// Builds the `CellStateMaterials3d` ressource storing every material handle for every possible state.
    fn setup_materials_3d(
        materials: &mut Assets<bevy::prelude::StandardMaterial>,
    ) -> crate::resources::materials::CellStateMaterials3d;
}
