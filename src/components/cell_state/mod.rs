#[cfg(feature = "auto-coloring")]
use bevy::prelude::Assets;
use std::fmt::Debug;
pub use {classic_state::*, cyclic_state::*, world_wire_state::*};

mod classic_state;
mod cyclic_state;
mod world_wire_state;

pub trait CellState: Debug + Default + Sized + Clone {
    fn new_cell_state(&self, neighbor_cells: &[Self]) -> Self;

    fn apply_new_cell_state(&mut self, neighbor_cells: &[Self]) {
        *self = self.new_cell_state(neighbor_cells)
    }

    #[cfg(feature = "auto-coloring")]
    fn material_index(&self) -> usize;

    #[cfg(all(feature = "auto-coloring", feature = "2D"))]
    fn setup_materials_2d(
        materials: &mut Assets<bevy::prelude::ColorMaterial>,
    ) -> crate::resources::materials::CellStateMaterials2d;

    #[cfg(all(feature = "auto-coloring", feature = "3D"))]
    fn setup_materials_3d(
        materials: &mut Assets<bevy::prelude::StandardMaterial>,
    ) -> crate::resources::materials::CellStateMaterials3d;
}
