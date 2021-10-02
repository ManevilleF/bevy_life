#[cfg(feature = "auto-coloring")]
use bevy::prelude::{Assets, Color};
use std::fmt::Debug;
pub use {
    conway_state::*, conway_state_3d::*, cyclic_color_state::*, immigration_state::*,
    rainbow_state::*, wire_world_cell_state::*,
};

mod conway_state;
mod conway_state_3d;
mod cyclic_color_state;
mod immigration_state;
mod rainbow_state;
mod wire_world_cell_state;

#[cfg(feature = "auto-coloring")]
/// Enum returned by a cell state to define its color:
/// either a material handle index or a new color
pub enum ColorResponse {
    /// No material
    None,
    /// A Material handle index
    MaterialIndex(usize),
    /// A new color
    Color(Color),
}

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
    fn color_or_material_index(&self) -> ColorResponse;

    #[cfg(feature = "auto-coloring")]
    /// All available colors of the state
    fn colors() -> &'static [Color];

    #[cfg(feature = "auto-coloring")]
    /// Builds the `CellStateMaterials` ressource storing every material handle for every possible state.
    fn setup_materials<A>(
        materials: &mut Assets<A>,
    ) -> crate::resources::materials::CellStateMaterials<A>
    where
        A: bevy::asset::Asset + From<Color>,
    {
        crate::resources::materials::CellStateMaterials {
            materials: Self::colors()
                .iter()
                .map(|c| materials.add((*c).into()))
                .collect(),
        }
    }
}
