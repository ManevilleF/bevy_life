#[cfg(feature = "auto-coloring")]
use crate::resources::materials::CellStateMaterials;
#[cfg(feature = "auto-coloring")]
use bevy::prelude::{Assets, ColorMaterial};
use std::fmt::Debug;
pub use {classic_state::*, world_wire_state::*};

mod classic_state;
mod world_wire_state;

pub trait CellState: Debug + Default + Sized + Clone {
    fn new_cell_state(&self, neighbor_cells: &[&Self]) -> Self;

    fn apply_new_cell_state(&mut self, neighbor_cells: &[&Self]) {
        *self = self.new_cell_state(neighbor_cells)
    }

    #[cfg(feature = "auto-coloring")]
    fn material_index(&self) -> usize;

    #[cfg(feature = "auto-coloring")]
    fn setup_materials(materials: &mut Assets<ColorMaterial>) -> CellStateMaterials;
}
