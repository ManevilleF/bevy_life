use bevy::prelude::Component;
pub use conway_state::*;
pub use conway_state_3d::*;
pub use cyclic_color_state::*;
pub use immigration_state::*;
pub use rainbow_state::*;
pub use wire_world_cell_state::*;

mod conway_state;
mod conway_state_3d;
mod cyclic_color_state;
mod immigration_state;
mod rainbow_state;
mod wire_world_cell_state;

/// This trait defines the state of any given `Cell`. The trait implementation
/// will define the cellular automaton rules which will be automatically
/// applied.
///
/// Every type defining a `Cell` state and rules must implement this trait.
pub trait CellState: Component + Sized + Clone + PartialEq {
    /// Defines the new state for a cell given the `neighbor_cells` states and
    /// `self`.
    ///
    /// This method defines the cellular automaton rules
    #[must_use]
    fn new_cell_state<'a>(&self, neighbor_cells: impl Iterator<Item = &'a Self>) -> Self;

    #[cfg(feature = "auto-coloring")]
    /// Color of the state, to use with `auto-coloring` feature
    #[must_use]
    fn color(&self) -> Option<bevy::prelude::Color>;
}
