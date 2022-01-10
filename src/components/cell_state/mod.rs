use bevy::prelude::Component;
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

/// This trait defines the state of any given `Cell`. The trait implementation will define the
/// cellular automaton rules which will be automatically applied.
///
/// Every type defining a `Cell` state and rules must implement this trait.
pub trait CellState: Component + Debug + Default + Sized + Clone + PartialEq {
    /// Defines the new state for a cell given the `neighbor_cells` states and `self`.
    ///
    /// This method defines the cellular automaton rules
    fn new_cell_state(&self, neighbor_cells: &[Self]) -> Self;

    /// Mutably applies the new state defined in `new_cell_state` to `self`
    fn apply_new_cell_state(&mut self, neighbor_cells: &[Self]) {
        *self = self.new_cell_state(neighbor_cells);
    }

    #[cfg(feature = "auto-coloring")]
    /// Color of the state, to use with `auto-coloring` feature
    fn color(&self) -> Option<bevy::render::color::Color>;
}
