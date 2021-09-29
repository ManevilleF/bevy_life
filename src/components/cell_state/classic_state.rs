use crate::components::CellState;
#[cfg(feature = "auto-coloring")]
use crate::resources::materials::CellStateMaterials;
#[cfg(feature = "auto-coloring")]
use bevy::prelude::{Assets, Color, ColorMaterial};

pub type ClassicCellState = bool;

impl CellState for ClassicCellState {
    fn new_cell_state(&self, neighbor_cells: &[&Self]) -> Self {
        let alive_cells_count = neighbor_cells.iter().filter(|&&c| *c).count();
        !(2..=3).contains(&alive_cells_count)
    }

    #[cfg(feature = "auto-coloring")]
    fn material_index(&self) -> usize {
        if *self {
            1
        } else {
            0
        }
    }

    #[cfg(feature = "auto-coloring")]
    fn setup_materials(materials: &mut Assets<ColorMaterial>) -> CellStateMaterials {
        CellStateMaterials {
            materials: vec![
                materials.add(Color::WHITE.into()),
                materials.add(Color::BLACK.into()),
            ],
        }
    }
}
